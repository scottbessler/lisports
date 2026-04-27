use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::RwLock;

use crate::{
    cache::Cache,
    error::AppError,
    models::{BoxScore, PlayerStatsPage, Scoreboard, StandingsTable},
    normalizers,
};

#[async_trait]
pub trait SportsData: Send + Sync {
    async fn todays_scoreboard(&self) -> Result<Scoreboard, AppError>;
    async fn days_games(&self, day: &str) -> Result<Scoreboard, AppError>;
    async fn game(&self, game_id: &str) -> Result<Option<BoxScore>, AppError>;
    async fn standings(&self) -> Result<StandingsTable, AppError>;
    async fn player_stats(&self, player_id: &str) -> Result<PlayerStatsPage, AppError>;
}

#[derive(Clone)]
pub struct EspnSportsData {
    http: HttpClient,
    cache: Cache,
    today_cache: Arc<RwLock<Option<TodayCache>>>,
}

#[derive(Clone)]
struct HttpClient {
    client: Client,
}

#[derive(Clone)]
struct TodayCache {
    fetched_at: Instant,
    scoreboard: Scoreboard,
}

impl EspnSportsData {
    pub fn new(cache: Cache) -> Result<Self, AppError> {
        Ok(Self {
            http: HttpClient::new()?,
            cache,
            today_cache: Arc::new(RwLock::new(None)),
        })
    }
}

#[async_trait]
impl SportsData for EspnSportsData {
    async fn todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        if let Some(cache) = self.today_cache.read().await.as_ref()
            && cache.fetched_at.elapsed() < Duration::from_secs(30)
        {
            return Ok(cache.scoreboard.clone());
        }

        let data: NbaTodaysScoreboardDto = self
            .http
            .get_json("https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json", false, None)
            .await?;
        let scoreboard = normalizers::nba_today_scoreboard(data)?;
        *self.today_cache.write().await = Some(TodayCache {
            fetched_at: Instant::now(),
            scoreboard: scoreboard.clone(),
        });
        Ok(scoreboard)
    }

    async fn days_games(&self, day: &str) -> Result<Scoreboard, AppError> {
        let cache_key = format!("day2:{day}");
        if let Some(cached) = self.cache.get_json::<Scoreboard>(&cache_key).await? {
            return Ok(cached);
        }

        let espn_date = day.replace('-', "");
        let url = format!(
            "https://site.api.espn.com/apis/site/v2/sports/basketball/nba/scoreboard?dates={espn_date}"
        );
        let data: EspnScoreboardDto = self.http.get_json(&url, false, None).await?;
        let scoreboard = normalizers::espn_scoreboard(day, data)?;
        if scoreboard.games.iter().all(|game| game.game_status == 3) {
            self.cache.set_json(&cache_key, &scoreboard).await?;
        }
        Ok(scoreboard)
    }

    async fn game(&self, game_id: &str) -> Result<Option<BoxScore>, AppError> {
        let cache_key = format!("game:{game_id}");
        if let Some(cached) = self.cache.get_json::<BoxScore>(&cache_key).await? {
            return Ok(Some(cached));
        }

        let url = format!(
            "https://site.api.espn.com/apis/site/v2/sports/basketball/nba/summary?event={game_id}"
        );
        let data: EspnSummaryDto = self.http.get_json(&url, false, None).await?;
        let game = normalizers::espn_summary(data)?;
        if game.game_status == 3 {
            self.cache.set_json(&cache_key, &game).await?;
        }
        Ok(Some(game))
    }

    async fn standings(&self) -> Result<StandingsTable, AppError> {
        let cache_key = format!("standings2:{}", chrono::Utc::now().date_naive());
        if let Some(cached) = self.cache.get_json::<StandingsTable>(&cache_key).await? {
            return Ok(cached);
        }
        let data: EspnStandingsDto = self
            .http
            .get_json(
                "https://site.api.espn.com/apis/v2/sports/basketball/nba/standings",
                false,
                None,
            )
            .await?;
        let standings = normalizers::espn_standings(data);
        self.cache.set_json(&cache_key, &standings).await?;
        Ok(standings)
    }

    async fn player_stats(&self, player_id: &str) -> Result<PlayerStatsPage, AppError> {
        let cache_key = format!("player:{player_id}");
        if let Some(cached) = self.cache.get_json::<PlayerStatsPage>(&cache_key).await? {
            return Ok(cached);
        }

        if let Ok(data) = self.player_stats_espn(player_id).await {
            self.cache.set_json(&cache_key, &data).await?;
            return Ok(data);
        }

        let url = format!(
            "https://stats.nba.com/stats/playerdashboardbyyearoveryearcombined?DateFrom=&DateTo=&GameSegment=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerID={player_id}&PlusMinus=N&Rank=N&Season=2023-24&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&VsConference=&VsDivision="
        );
        let data: Value = self
            .http
            .get_json(&url, true, Some(Duration::from_secs(4)))
            .await?;
        let page = normalizers::nba_player_stats(data);
        self.cache.set_json(&cache_key, &page).await?;
        Ok(page)
    }
}

impl EspnSportsData {
    async fn player_stats_espn(&self, player_id: &str) -> Result<PlayerStatsPage, AppError> {
        let url = format!(
            "https://site.web.api.espn.com/apis/common/v3/sports/basketball/nba/athletes/{player_id}/gamelog"
        );
        let data: EspnPlayerGamelogDto = self.http.get_json(&url, false, None).await?;
        Ok(normalizers::espn_player_gamelog(player_id, data))
    }
}

impl HttpClient {
    fn new() -> Result<Self, AppError> {
        Ok(Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/110.0")
                .timeout(Duration::from_secs(10))
                .build()
                .map_err(AppError::upstream)?,
        })
    }

    async fn get_json<T: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        nba_headers: bool,
        timeout: Option<Duration>,
    ) -> Result<T, AppError> {
        tracing::info!("fetching {url}");
        let mut req = self.client.get(url);
        if let Some(timeout) = timeout {
            req = req.timeout(timeout);
        }
        if nba_headers {
            req = req
                .header("Accept-Language", "en-US,en;q=0.5")
                .header("Referer", "https://www.nba.com/")
                .header("Origin", "https://www.nba.com");
        }
        let response = req.send().await.map_err(AppError::upstream)?;
        let status = response.status();
        let body = response.text().await.map_err(AppError::upstream)?;
        if !status.is_success() {
            return Err(AppError::Upstream(format!(
                "request failed {status}: {body}"
            )));
        }
        serde_json::from_str(&body).map_err(AppError::parse)
    }
}

#[derive(Debug, Deserialize)]
pub struct NbaTodaysScoreboardDto {
    pub scoreboard: Value,
}

#[derive(Debug, Deserialize)]
pub struct EspnScoreboardDto {
    pub events: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct EspnSummaryDto {
    pub boxscore: Value,
    #[serde(rename = "gameInfo")]
    pub game_info: Option<Value>,
    pub header: Value,
}

#[derive(Debug, Deserialize)]
pub struct EspnStandingsDto {
    pub children: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct EspnPlayerGamelogDto {
    pub labels: Vec<String>,
    pub events: Value,
    #[serde(rename = "seasonTypes")]
    pub season_types: Vec<Value>,
}
