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
    leagues::{self, League},
    models::{
        BoxScore, MlbBoxScore, MlbStandingsTable, NflBoxScore, NflStandingsTable, NhlBoxScore,
        NhlStandingsTable, PlayerStatsPage, Scoreboard, StandingsTable,
    },
    normalizers,
};

pub const LIVE_DATA_CACHE_SECONDS: u64 = 30;

#[async_trait]
pub trait SportsData: Send + Sync {
    async fn todays_scoreboard(&self) -> Result<Scoreboard, AppError>;
    async fn days_games(&self, day: &str) -> Result<Scoreboard, AppError>;
    async fn game(&self, game_id: &str) -> Result<Option<BoxScore>, AppError>;
    async fn standings(&self) -> Result<StandingsTable, AppError>;
    async fn player_stats(&self, player_id: &str) -> Result<PlayerStatsPage, AppError>;
    async fn wnba_todays_scoreboard(&self) -> Result<Scoreboard, AppError>;
    async fn wnba_days_games(&self, day: &str) -> Result<Scoreboard, AppError>;
    async fn wnba_game(&self, game_id: &str) -> Result<Option<BoxScore>, AppError>;
    async fn wnba_standings(&self) -> Result<StandingsTable, AppError>;
    async fn mlb_todays_scoreboard(&self) -> Result<Scoreboard, AppError>;
    async fn mlb_days_games(&self, day: &str) -> Result<Scoreboard, AppError>;
    async fn mlb_game(&self, game_id: &str) -> Result<Option<MlbBoxScore>, AppError>;
    async fn mlb_standings(&self) -> Result<MlbStandingsTable, AppError>;
    async fn nfl_current_scoreboard(&self) -> Result<Scoreboard, AppError>;
    async fn nfl_week_games(&self, week: i64) -> Result<Scoreboard, AppError>;
    async fn nfl_game(&self, game_id: &str) -> Result<Option<NflBoxScore>, AppError>;
    async fn nfl_standings(&self) -> Result<NflStandingsTable, AppError>;
    async fn nhl_todays_scoreboard(&self) -> Result<Scoreboard, AppError>;
    async fn nhl_days_games(&self, day: &str) -> Result<Scoreboard, AppError>;
    async fn nhl_game(&self, game_id: &str) -> Result<Option<NhlBoxScore>, AppError>;
    async fn nhl_standings(&self) -> Result<NhlStandingsTable, AppError>;
}

#[derive(Clone)]
pub struct EspnSportsData {
    http: HttpClient,
    cache: Cache,
    today_cache: Arc<RwLock<Option<TodayCache>>>,
    wnba_today_cache: Arc<RwLock<Option<TodayCache>>>,
    mlb_today_cache: Arc<RwLock<Option<TodayCache>>>,
    nfl_today_cache: Arc<RwLock<Option<TodayCache>>>,
    nhl_today_cache: Arc<RwLock<Option<TodayCache>>>,
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
            wnba_today_cache: Arc::new(RwLock::new(None)),
            mlb_today_cache: Arc::new(RwLock::new(None)),
            nfl_today_cache: Arc::new(RwLock::new(None)),
            nhl_today_cache: Arc::new(RwLock::new(None)),
        })
    }
}

enum EspnScoreboardQuery {
    Date(String),
    NflWeek { season_type: i64, week: i64 },
}

fn league(slug: &str) -> &'static League {
    leagues::by_slug(slug).expect("league registry entry")
}

fn espn_scoreboard_url(league: &League, query: EspnScoreboardQuery) -> String {
    let base = format!(
        "https://site.api.espn.com/apis/site/v2/sports/{}/{}/scoreboard",
        league.sport_path, league.league_path
    );
    match query {
        EspnScoreboardQuery::Date(day) => format!("{base}?dates={}", day.replace('-', "")),
        EspnScoreboardQuery::NflWeek { season_type, week } => {
            format!("{base}?seasontype={season_type}&week={week}")
        }
    }
}

fn espn_summary_url(league: &League, game_id: &str) -> String {
    format!(
        "https://site.api.espn.com/apis/site/v2/sports/{}/{}/summary?event={game_id}",
        league.sport_path, league.league_path
    )
}

fn espn_standings_url(league: &League) -> String {
    format!(
        "https://site.api.espn.com/apis/v2/sports/{}/{}/standings",
        league.sport_path, league.league_path
    )
}

impl EspnSportsData {
    async fn espn_scoreboard(
        &self,
        league: &League,
        source_bucket: &str,
        query: EspnScoreboardQuery,
        normalize: fn(&str, EspnScoreboardDto) -> Result<Scoreboard, AppError>,
    ) -> Result<Scoreboard, AppError> {
        let url = espn_scoreboard_url(league, query);
        let data: EspnScoreboardDto = self.http.get_json(&url, false, None).await?;
        normalize(source_bucket, data)
    }

    async fn espn_summary<T>(
        &self,
        league: &League,
        game_id: &str,
        normalize: fn(EspnSummaryDto) -> Result<T, AppError>,
    ) -> Result<T, AppError> {
        let url = espn_summary_url(league, game_id);
        let data: EspnSummaryDto = self.http.get_json(&url, false, None).await?;
        normalize(data)
    }

    async fn espn_standings<T>(
        &self,
        league: &League,
        normalize: fn(EspnStandingsDto) -> T,
    ) -> Result<T, AppError> {
        let url = espn_standings_url(league);
        let data: EspnStandingsDto = self.http.get_json(&url, false, None).await?;
        Ok(normalize(data))
    }
}

#[async_trait]
impl SportsData for EspnSportsData {
    async fn todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        if let Some(cache) = self.today_cache.read().await.as_ref()
            && cache.fetched_at.elapsed() < Duration::from_secs(LIVE_DATA_CACHE_SECONDS)
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
        let cache_key = format!("day3:{day}");
        if let Some(cached) = self.cache.get_json::<Scoreboard>(&cache_key).await? {
            return Ok(cached);
        }

        let scoreboard = self
            .espn_scoreboard(
                league("nba"),
                day,
                EspnScoreboardQuery::Date(day.to_string()),
                normalizers::espn_scoreboard,
            )
            .await?;
        if !scoreboard.games.is_empty() && scoreboard.games.iter().all(|game| game.game_status == 3)
        {
            self.cache.set_json(&cache_key, &scoreboard).await?;
        }
        Ok(scoreboard)
    }

    async fn game(&self, game_id: &str) -> Result<Option<BoxScore>, AppError> {
        let cache_key = format!("game:{game_id}");
        if let Some(cached) = self.cache.get_json::<BoxScore>(&cache_key).await? {
            return Ok(Some(cached));
        }

        let game = self
            .espn_summary(league("nba"), game_id, normalizers::espn_summary)
            .await?;
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
        let standings = self
            .espn_standings(league("nba"), normalizers::espn_standings)
            .await?;
        self.cache.set_json(&cache_key, &standings).await?;
        Ok(standings)
    }

    async fn player_stats(&self, player_id: &str) -> Result<PlayerStatsPage, AppError> {
        let cache_key = format!("player3:{player_id}");
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

    async fn wnba_todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        if let Some(cache) = self.wnba_today_cache.read().await.as_ref()
            && cache.fetched_at.elapsed() < Duration::from_secs(LIVE_DATA_CACHE_SECONDS)
        {
            return Ok(cache.scoreboard.clone());
        }

        let day = chrono::Utc::now().date_naive().to_string();
        let scoreboard = self.wnba_days_games(&day).await?;
        *self.wnba_today_cache.write().await = Some(TodayCache {
            fetched_at: Instant::now(),
            scoreboard: scoreboard.clone(),
        });
        Ok(scoreboard)
    }

    async fn wnba_days_games(&self, day: &str) -> Result<Scoreboard, AppError> {
        let cache_key = format!("wnba-day:{day}");
        if let Some(cached) = self.cache.get_json::<Scoreboard>(&cache_key).await? {
            return Ok(cached);
        }

        let scoreboard = self
            .espn_scoreboard(
                league("wnba"),
                day,
                EspnScoreboardQuery::Date(day.to_string()),
                normalizers::espn_wnba_scoreboard,
            )
            .await?;
        if scoreboard.games.iter().all(|game| game.game_status == 3) {
            self.cache.set_json(&cache_key, &scoreboard).await?;
        }
        Ok(scoreboard)
    }

    async fn wnba_game(&self, game_id: &str) -> Result<Option<BoxScore>, AppError> {
        let cache_key = format!("wnba-game:{game_id}");
        if let Some(cached) = self.cache.get_json::<BoxScore>(&cache_key).await? {
            return Ok(Some(cached));
        }

        let game = self
            .espn_summary(league("wnba"), game_id, normalizers::espn_wnba_summary)
            .await?;
        if game.game_status == 3 {
            self.cache.set_json(&cache_key, &game).await?;
        }
        Ok(Some(game))
    }

    async fn wnba_standings(&self) -> Result<StandingsTable, AppError> {
        let cache_key = format!("wnba-standings:{}", chrono::Utc::now().date_naive());
        if let Some(cached) = self.cache.get_json::<StandingsTable>(&cache_key).await? {
            return Ok(cached);
        }
        let standings = self
            .espn_standings(league("wnba"), normalizers::espn_wnba_standings)
            .await?;
        self.cache.set_json(&cache_key, &standings).await?;
        Ok(standings)
    }

    async fn mlb_todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        if let Some(cache) = self.mlb_today_cache.read().await.as_ref()
            && cache.fetched_at.elapsed() < Duration::from_secs(LIVE_DATA_CACHE_SECONDS)
        {
            return Ok(cache.scoreboard.clone());
        }

        let day = chrono::Utc::now().date_naive().to_string();
        let scoreboard = self.mlb_days_games(&day).await?;
        *self.mlb_today_cache.write().await = Some(TodayCache {
            fetched_at: Instant::now(),
            scoreboard: scoreboard.clone(),
        });
        Ok(scoreboard)
    }

    async fn mlb_days_games(&self, day: &str) -> Result<Scoreboard, AppError> {
        let cache_key = format!("mlb-day:{day}");
        if let Some(cached) = self.cache.get_json::<Scoreboard>(&cache_key).await? {
            return Ok(cached);
        }

        let scoreboard = self
            .espn_scoreboard(
                league("mlb"),
                day,
                EspnScoreboardQuery::Date(day.to_string()),
                normalizers::espn_mlb_scoreboard,
            )
            .await?;
        if scoreboard.games.iter().all(|game| game.game_status == 3) {
            self.cache.set_json(&cache_key, &scoreboard).await?;
        }
        Ok(scoreboard)
    }

    async fn mlb_game(&self, game_id: &str) -> Result<Option<MlbBoxScore>, AppError> {
        let cache_key = format!("mlb-game2:{game_id}");
        if let Some(cached) = self.cache.get_json::<MlbBoxScore>(&cache_key).await? {
            return Ok(Some(cached));
        }

        let game = self
            .espn_summary(league("mlb"), game_id, normalizers::espn_mlb_summary)
            .await?;
        if game.game_status == 3 {
            self.cache.set_json(&cache_key, &game).await?;
        }
        Ok(Some(game))
    }

    async fn mlb_standings(&self) -> Result<MlbStandingsTable, AppError> {
        let cache_key = format!("mlb-standings2:{}", chrono::Utc::now().date_naive());
        if let Some(cached) = self.cache.get_json::<MlbStandingsTable>(&cache_key).await? {
            return Ok(cached);
        }
        let standings = self
            .espn_standings(league("mlb"), normalizers::espn_mlb_standings)
            .await?;
        self.cache.set_json(&cache_key, &standings).await?;
        Ok(standings)
    }

    async fn nfl_current_scoreboard(&self) -> Result<Scoreboard, AppError> {
        if let Some(cache) = self.nfl_today_cache.read().await.as_ref()
            && cache.fetched_at.elapsed() < Duration::from_secs(LIVE_DATA_CACHE_SECONDS)
        {
            return Ok(cache.scoreboard.clone());
        }

        let mut scoreboard = None;
        for week in (1..=23).rev() {
            let candidate = self.nfl_week_games(week).await?;
            if candidate.games.iter().any(|game| game.game_status >= 2) {
                scoreboard = Some(candidate);
                break;
            }
        }
        let scoreboard = scoreboard.unwrap_or_else(|| Scoreboard {
            game_date: "1".to_string(),
            games: Vec::new(),
        });
        *self.nfl_today_cache.write().await = Some(TodayCache {
            fetched_at: Instant::now(),
            scoreboard: scoreboard.clone(),
        });
        Ok(scoreboard)
    }

    async fn nfl_week_games(&self, week: i64) -> Result<Scoreboard, AppError> {
        let cache_key = format!("nfl-week:{week}");
        if let Some(cached) = self.cache.get_json::<Scoreboard>(&cache_key).await? {
            return Ok(cached);
        }

        let (season_type, espn_week) = nfl_espn_week(week);
        let scoreboard = self
            .espn_scoreboard(
                league("nfl"),
                &week.to_string(),
                EspnScoreboardQuery::NflWeek {
                    season_type,
                    week: espn_week,
                },
                normalizers::espn_nfl_scoreboard,
            )
            .await?;
        if scoreboard.games.iter().all(|game| game.game_status == 3) {
            self.cache.set_json(&cache_key, &scoreboard).await?;
        }
        Ok(scoreboard)
    }

    async fn nfl_game(&self, game_id: &str) -> Result<Option<NflBoxScore>, AppError> {
        let cache_key = format!("nfl-game:{game_id}");
        if let Some(cached) = self.cache.get_json::<NflBoxScore>(&cache_key).await? {
            return Ok(Some(cached));
        }

        let game = self
            .espn_summary(league("nfl"), game_id, normalizers::espn_nfl_summary)
            .await?;
        if game.game_status == 3 {
            self.cache.set_json(&cache_key, &game).await?;
        }
        Ok(Some(game))
    }

    async fn nfl_standings(&self) -> Result<NflStandingsTable, AppError> {
        let cache_key = format!("nfl-standings:{}", chrono::Utc::now().date_naive());
        if let Some(cached) = self.cache.get_json::<NflStandingsTable>(&cache_key).await? {
            return Ok(cached);
        }
        let standings = self
            .espn_standings(league("nfl"), normalizers::espn_nfl_standings)
            .await?;
        self.cache.set_json(&cache_key, &standings).await?;
        Ok(standings)
    }

    async fn nhl_todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        if let Some(cache) = self.nhl_today_cache.read().await.as_ref()
            && cache.fetched_at.elapsed() < Duration::from_secs(LIVE_DATA_CACHE_SECONDS)
        {
            return Ok(cache.scoreboard.clone());
        }

        let day = chrono::Utc::now().date_naive().to_string();
        let scoreboard = self.nhl_days_games(&day).await?;
        *self.nhl_today_cache.write().await = Some(TodayCache {
            fetched_at: Instant::now(),
            scoreboard: scoreboard.clone(),
        });
        Ok(scoreboard)
    }

    async fn nhl_days_games(&self, day: &str) -> Result<Scoreboard, AppError> {
        let cache_key = format!("nhl-day:{day}");
        if let Some(cached) = self.cache.get_json::<Scoreboard>(&cache_key).await? {
            return Ok(cached);
        }

        let scoreboard = self
            .espn_scoreboard(
                league("nhl"),
                day,
                EspnScoreboardQuery::Date(day.to_string()),
                normalizers::espn_nhl_scoreboard,
            )
            .await?;
        if scoreboard.games.iter().all(|game| game.game_status == 3) {
            self.cache.set_json(&cache_key, &scoreboard).await?;
        }
        Ok(scoreboard)
    }

    async fn nhl_game(&self, game_id: &str) -> Result<Option<NhlBoxScore>, AppError> {
        let cache_key = format!("nhl-game:{game_id}");
        if let Some(cached) = self.cache.get_json::<NhlBoxScore>(&cache_key).await? {
            return Ok(Some(cached));
        }

        let game = self
            .espn_summary(league("nhl"), game_id, normalizers::espn_nhl_summary)
            .await?;
        if game.game_status == 3 {
            self.cache.set_json(&cache_key, &game).await?;
        }
        Ok(Some(game))
    }

    async fn nhl_standings(&self) -> Result<NhlStandingsTable, AppError> {
        let cache_key = format!("nhl-standings:{}", chrono::Utc::now().date_naive());
        if let Some(cached) = self.cache.get_json::<NhlStandingsTable>(&cache_key).await? {
            return Ok(cached);
        }
        let standings = self
            .espn_standings(league("nhl"), normalizers::espn_nhl_standings)
            .await?;
        self.cache.set_json(&cache_key, &standings).await?;
        Ok(standings)
    }
}

fn nfl_espn_week(week: i64) -> (i64, i64) {
    if week <= 18 {
        (2, week)
    } else {
        (3, week - 18)
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
    pub week: Option<Value>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn espn_endpoint_builders_use_registry_paths() {
        let nba = league("nba");
        assert_eq!(
            espn_scoreboard_url(nba, EspnScoreboardQuery::Date("2026-04-26".to_string())),
            "https://site.api.espn.com/apis/site/v2/sports/basketball/nba/scoreboard?dates=20260426"
        );
        assert_eq!(
            espn_summary_url(nba, "401869385"),
            "https://site.api.espn.com/apis/site/v2/sports/basketball/nba/summary?event=401869385"
        );
        assert_eq!(
            espn_standings_url(nba),
            "https://site.api.espn.com/apis/v2/sports/basketball/nba/standings"
        );
    }

    #[test]
    fn nfl_scoreboard_endpoint_uses_season_type_and_week() {
        assert_eq!(
            espn_scoreboard_url(
                league("nfl"),
                EspnScoreboardQuery::NflWeek {
                    season_type: 3,
                    week: 5,
                },
            ),
            "https://site.api.espn.com/apis/site/v2/sports/football/nfl/scoreboard?seasontype=3&week=5"
        );
    }
}
