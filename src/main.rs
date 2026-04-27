#![recursion_limit = "256"]

use std::{
    env,
    net::SocketAddr,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{Context, Result, anyhow};
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
};
use chrono::{Datelike, Days, NaiveDate, Utc};
use reqwest::Client;
use serde_json::{Value, json};
use tokio::sync::RwLock;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[derive(Clone)]
struct AppState {
    client: Client,
    data_path: PathBuf,
    today_cache: Arc<RwLock<Option<TodayCache>>>,
}

struct TodayCache {
    fetched_at: Instant,
    scoreboard: Value,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let data_path = env::var("DATA_PATH").unwrap_or_else(|_| "data".to_string());
    tokio::fs::create_dir_all(&data_path).await?;

    let state = AppState {
        client: Client::builder()
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/110.0")
            .timeout(Duration::from_secs(10))
            .build()?,
        data_path: PathBuf::from(data_path),
        today_cache: Arc::new(RwLock::new(None)),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/healthcheck", get(healthcheck))
        .route("/nba/scoreboard", get(nba_scoreboard))
        .route("/nba/scoreboard/today", get(nba_scoreboard_today))
        .route("/nba/scoreboard/{day}", get(nba_scoreboard_day))
        .route("/nba/scoreboard/{day}/game/{game_id}", get(nba_game))
        .route("/nba/standings", get(nba_standings))
        .route("/nba/player/{player_id}", get(nba_player))
        .route("/mlb/scoreboard", get(coming_soon))
        .route("/nfl/scoreboard", get(coming_soon))
        .nest_service("/public", ServeDir::new("public"))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Redirect {
    Redirect::temporary("/nba/scoreboard")
}

async fn healthcheck() -> &'static str {
    "OK"
}

async fn nba_scoreboard(State(state): State<AppState>) -> Response {
    match fetch_todays_scoreboard(&state).await {
        Ok(scoreboard) => {
            let today = str_at(&scoreboard, &["gameDate"]).unwrap_or_else(|| today_ymd());
            let games = array_at(&scoreboard, &["games"]);
            let has_live_or_completed = games
                .iter()
                .any(|g| i64_at(g, &["gameStatus"]).unwrap_or(0) >= 2);
            let target = if has_live_or_completed {
                today
            } else {
                shift_day(&today, -1).unwrap_or(today)
            };
            Redirect::temporary(&format!("/nba/scoreboard/{target}")).into_response()
        }
        Err(err) => error_page(
            StatusCode::BAD_GATEWAY,
            "Could not load today's scoreboard",
            &err,
        ),
    }
}

async fn nba_scoreboard_today(State(state): State<AppState>) -> Response {
    match fetch_todays_scoreboard(&state).await {
        Ok(scoreboard) => {
            let today = str_at(&scoreboard, &["gameDate"]).unwrap_or_else(|| today_ymd());
            Redirect::temporary(&format!("/nba/scoreboard/{today}")).into_response()
        }
        Err(err) => error_page(
            StatusCode::BAD_GATEWAY,
            "Could not load today's scoreboard",
            &err,
        ),
    }
}

async fn nba_scoreboard_day(State(state): State<AppState>, Path(day): Path<String>) -> Response {
    match fetch_days_games(&state, &day).await {
        Ok(games) => Html(layout(
            "NBA Scoreboard",
            &scoreboard_page(&day, &games, None),
        ))
        .into_response(),
        Err(err) => error_page(StatusCode::BAD_GATEWAY, "Could not load scoreboard", &err),
    }
}

async fn nba_game(
    State(state): State<AppState>,
    Path((day, game_id)): Path<(String, String)>,
) -> Response {
    let games = match fetch_days_games(&state, &day).await {
        Ok(games) => games,
        Err(err) => return error_page(StatusCode::BAD_GATEWAY, "Could not load scoreboard", &err),
    };
    let game = fetch_game(&state, &game_id).await.ok().flatten();
    Html(layout(
        "NBA Game",
        &scoreboard_page(&day, &games, game.as_ref()),
    ))
    .into_response()
}

async fn nba_standings(State(state): State<AppState>) -> Response {
    match fetch_standings(&state).await {
        Ok(standings) => Html(layout("NBA Standings", &standings_page(&standings))).into_response(),
        Err(err) => error_page(StatusCode::BAD_GATEWAY, "Could not load standings", &err),
    }
}

async fn nba_player(State(state): State<AppState>, Path(player_id): Path<String>) -> Response {
    match fetch_player_stats(&state, &player_id).await {
        Ok(stats) => Html(layout("NBA Player", &player_page(&stats))).into_response(),
        Err(err) => error_page(StatusCode::BAD_GATEWAY, "Could not load player stats", &err),
    }
}

async fn coming_soon() -> Html<String> {
    Html(layout(
        "Coming Soon",
        r#"<main class="center"><h1>Coming eventually?</h1></main>"#,
    ))
}

async fn fetch_todays_scoreboard(state: &AppState) -> Result<Value> {
    if let Some(cache) = state.today_cache.read().await.as_ref() {
        if cache.fetched_at.elapsed() < Duration::from_secs(30) {
            return Ok(cache.scoreboard.clone());
        }
    }
    let data = get_json(
        &state.client,
        "https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json",
        false,
    )
    .await?;
    let scoreboard = data
        .get("scoreboard")
        .cloned()
        .context("missing scoreboard")?;
    *state.today_cache.write().await = Some(TodayCache {
        fetched_at: Instant::now(),
        scoreboard: scoreboard.clone(),
    });
    Ok(scoreboard)
}

async fn fetch_days_games(state: &AppState, day: &str) -> Result<Vec<Value>> {
    let cache_key = format!("day:{day}");
    if let Some(cached) = read_cache(state, &cache_key).await? {
        return Ok(array_at(&cached, &["scoreboard", "games"]));
    }

    let games = fetch_days_games_espn(state, day).await?;
    if games.iter().all(|g| i64_at(g, &["gameStatus"]) == Some(3)) {
        let wrapped = json!({
            "meta": {"version": 1, "request": "espn", "time": Utc::now().to_rfc3339(), "code": 200},
            "scoreboard": {"gameDate": day, "leagueId": "00", "leagueName": "National Basketball Association", "games": games}
        });
        write_cache(state, &cache_key, &wrapped).await?;
        return Ok(array_at(&wrapped, &["scoreboard", "games"]));
    }
    Ok(games)
}

async fn fetch_days_games_espn(state: &AppState, day: &str) -> Result<Vec<Value>> {
    let espn_date = day.replace('-', "");
    let url = format!(
        "https://site.api.espn.com/apis/site/v2/sports/basketball/nba/scoreboard?dates={espn_date}"
    );
    let data = get_json(&state.client, &url, false).await?;
    let mut games = Vec::new();
    for event in array_at(&data, &["events"]) {
        let comp = event
            .pointer("/competitions/0")
            .context("missing competition")?;
        let status = comp.get("status").unwrap_or(&Value::Null);
        let competitors = array_at(comp, &["competitors"]);
        let home = competitors
            .iter()
            .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("home"))
            .context("missing home team")?;
        let away = competitors
            .iter()
            .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("away"))
            .context("missing away team")?;
        let home_team = espn_competitor_to_team(home);
        let away_team = espn_competitor_to_team(away);
        let home_tri = str_at(&home_team, &["teamTricode"]).unwrap_or_default();
        let away_tri = str_at(&away_team, &["teamTricode"]).unwrap_or_default();
        games.push(json!({
            "gameId": str_at(&event, &["id"]).unwrap_or_default(),
            "gameCode": format!("{day}/{away_tri}{home_tri}"),
            "gameStatus": espn_status_to_game_status(status),
            "gameStatusText": str_at(status, &["type", "shortDetail"]).or_else(|| str_at(status, &["type", "description"])).unwrap_or_default(),
            "period": i64_at(status, &["period"]).unwrap_or(0),
            "gameClock": str_at(status, &["displayClock"]).unwrap_or_default(),
            "gameTimeUTC": str_at(&event, &["date"]).unwrap_or_default(),
            "gameEt": str_at(&event, &["date"]).unwrap_or_default(),
            "regulationPeriods": 4,
            "ifNecessary": false,
            "seriesGameNumber": "",
            "seriesText": "",
            "homeTeam": home_team,
            "awayTeam": away_team,
            "gameLeaders": {"homeLeaders": extract_leaders(home), "awayLeaders": extract_leaders(away)},
            "pbOdds": {"team": null, "odds": 0, "suspended": 0}
        }));
    }
    Ok(games)
}

async fn fetch_game(state: &AppState, game_id: &str) -> Result<Option<Value>> {
    let cache_key = format!("game:{game_id}");
    if let Some(cached) = read_cache(state, &cache_key).await? {
        return Ok(cached.get("game").cloned());
    }

    let url = format!(
        "https://site.api.espn.com/apis/site/v2/sports/basketball/nba/summary?event={game_id}"
    );
    let data = get_json(&state.client, &url, false).await?;
    let game = espn_summary_to_boxscore(&data)?;
    if i64_at(&game, &["gameStatus"]) == Some(3) {
        write_cache(state, &cache_key, &json!({ "game": game })).await?;
        return Ok(read_cache(state, &cache_key)
            .await?
            .and_then(|v| v.get("game").cloned()));
    }
    Ok(Some(game))
}

async fn fetch_standings(state: &AppState) -> Result<Value> {
    let cache_key = format!("standings2:{}", today_ymd());
    if let Some(cached) = read_cache(state, &cache_key).await? {
        return Ok(cached);
    }
    let url = "https://site.api.espn.com/apis/v2/sports/basketball/nba/standings";
    let data = get_json(&state.client, url, false).await?;
    let standings = espn_standings_to_result_set(&data);
    write_cache(state, &cache_key, &standings).await?;
    Ok(standings)
}

async fn fetch_player_stats(state: &AppState, player_id: &str) -> Result<Value> {
    let cache_key = format!("player:{player_id}");
    if let Some(cached) = read_cache(state, &cache_key).await? {
        return Ok(cached);
    }
    let url = format!(
        "https://stats.nba.com/stats/playerdashboardbyyearoveryearcombined?DateFrom=&DateTo=&GameSegment=&LastNGames=0&LeagueID=00&Location=&MeasureType=Base&Month=0&OpponentTeamID=0&Outcome=&PORound=0&PaceAdjust=N&PerMode=PerGame&Period=0&PlayerID={player_id}&PlusMinus=N&Rank=N&Season=2023-24&SeasonSegment=&SeasonType=Regular%20Season&ShotClockRange=&VsConference=&VsDivision="
    );
    let data = get_json(&state.client, &url, true).await?;
    write_cache(state, &cache_key, &data).await?;
    Ok(data)
}

async fn get_json(client: &Client, url: &str, nba_headers: bool) -> Result<Value> {
    tracing::info!("fetching {url}");
    let mut req = client.get(url);
    if nba_headers {
        req = req
            .header("Accept-Language", "en-US,en;q=0.5")
            .header("Referer", "https://www.nba.com/")
            .header("Origin", "https://www.nba.com");
    }
    let response = req.send().await?;
    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        return Err(anyhow!("request failed {status}: {body}"));
    }
    serde_json::from_str(&body).context("invalid json response")
}

async fn read_cache(state: &AppState, key: &str) -> Result<Option<Value>> {
    let path = cache_path(state, key);
    match tokio::fs::read_to_string(path).await {
        Ok(text) => Ok(Some(serde_json::from_str(&text)?)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.into()),
    }
}

async fn write_cache(state: &AppState, key: &str, value: &Value) -> Result<()> {
    let path = cache_path(state, key);
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(path, serde_json::to_vec(value)?).await?;
    Ok(())
}

fn cache_path(state: &AppState, key: &str) -> PathBuf {
    state.data_path.join(format!("{key}.json"))
}

fn layout(title: &str, body: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en" data-theme="light">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <title>{}</title>
  <link rel="icon" href="/public/favicon.ico">
  <style>{}</style>
</head>
<body>
  {}
  {}
  <script>{}</script>
</body>
</html>"#,
        escape(title),
        CSS,
        nav(),
        body,
        TABLE_SORT_SCRIPT
    )
}

fn nav() -> &'static str {
    r#"<nav class="nav">
  <div class="brand">LiSports</div>
  <a href="/nba/scoreboard">NBA Scoreboard</a>
  <a href="/nba/standings">NBA Standings</a>
  <a href="/mlb/scoreboard">MLB</a>
  <a href="/nfl/scoreboard">NFL</a>
</nav>"#
}

fn scoreboard_page(day: &str, games: &[Value], selected_game: Option<&Value>) -> String {
    let mut html = String::new();
    html.push_str(r#"<main class="page">"#);
    html.push_str(&date_nav(day));
    if games.is_empty() {
        html.push_str(r#"<section class="center"><h1>No Games Scheduled</h1></section>"#);
    } else {
        let scoreboard_class = if selected_game.is_some() {
            "scoreboard has-game"
        } else {
            "scoreboard"
        };
        html.push_str(&format!(r#"<section class="{scoreboard_class}">"#));
        html.push_str(r#"<div class="game-list">"#);
        let all_completed = games.iter().all(|g| i64_at(g, &["gameStatus"]) == Some(3));
        for game in games {
            html.push_str(&format!(
                r#"<a class="game-link" href="/nba/scoreboard/{}/game/{}">{}</a>"#,
                escape_attr(day),
                escape_attr(&str_at(game, &["gameId"]).unwrap_or_default()),
                game_summary(game, !all_completed)
            ));
        }
        html.push_str("</div>");
        if let Some(game) = selected_game {
            html.push_str(&game_details(game));
        }
        html.push_str("</section>");
    }
    html.push_str("</main>");
    html
}

fn date_nav(day: &str) -> String {
    let mut html = String::from(r#"<div class="date-nav">"#);
    let parsed =
        NaiveDate::parse_from_str(day, "%Y-%m-%d").unwrap_or_else(|_| Utc::now().date_naive());
    html.push_str(&format!(
        r#"<a class="button" href="/nba/scoreboard/{}">Prev</a>"#,
        parsed.checked_sub_days(Days::new(1)).unwrap_or(parsed)
    ));
    for offset in -3..=3 {
        let d = parsed + chrono::Duration::days(offset);
        let label = if d == Utc::now().date_naive() {
            "Today".to_string()
        } else {
            format!(
                "{} {}/{}",
                weekday(d.weekday().num_days_from_sunday()),
                d.month(),
                d.day()
            )
        };
        let class = if d == parsed {
            "button active"
        } else {
            "button"
        };
        html.push_str(&format!(
            r#"<a class="{class}" href="/nba/scoreboard/{d}">{}</a>"#,
            escape(&label)
        ));
    }
    html.push_str(&format!(
        r#"<a class="button" href="/nba/scoreboard/{}">Next</a>"#,
        parsed.checked_add_days(Days::new(1)).unwrap_or(parsed)
    ));
    html.push_str("</div>");
    html
}

fn game_summary(game: &Value, show_status: bool) -> String {
    let away = game.get("awayTeam").unwrap_or(&Value::Null);
    let home = game.get("homeTeam").unwrap_or(&Value::Null);
    let mut html = String::from(r#"<table class="game-card"><thead><tr><th></th>"#);
    for p in array_at(away, &["periods"]) {
        html.push_str(&format!(
            "<th>{}</th>",
            i64_at(&p, &["period"]).unwrap_or(0)
        ));
    }
    html.push_str("<th>T</th></tr></thead><tbody>");
    html.push_str(&team_summary_row(game, away, false));
    html.push_str(&team_summary_row(game, home, true));
    if show_status {
        let colspan = array_at(away, &["periods"]).len() + 2;
        html.push_str(&format!(
            r#"<tr><th class="status" colspan="{colspan}">{}</th></tr>"#,
            escape(&str_at(game, &["gameStatusText"]).unwrap_or_default())
        ));
    }
    html.push_str("</tbody></table>");
    html
}

fn team_summary_row(game: &Value, team: &Value, is_home: bool) -> String {
    let mut html = String::from("<tr><th>");
    html.push_str(&team_logo(team, "mini-logo"));
    html.push_str(&format!(
        r#"<span title="{}">{}</span> <small>({}-{})</small> {}"#,
        escape_attr(&format!(
            "{} {}",
            str_at(team, &["teamCity"]).unwrap_or_default(),
            str_at(team, &["teamName"]).unwrap_or_default()
        )),
        escape(&str_at(team, &["teamTricode"]).unwrap_or_default()),
        i64_at(team, &["wins"]).unwrap_or(0),
        i64_at(team, &["losses"]).unwrap_or(0),
        winner(game, is_home)
    ));
    html.push_str("</th>");
    for p in array_at(team, &["periods"]) {
        let score = i64_at(&p, &["score"]).unwrap_or(0);
        html.push_str(&format!(
            "<td>{}</td>",
            if score == 0 {
                "-".to_string()
            } else {
                score.to_string()
            }
        ));
    }
    html.push_str(&format!(
        "<td>{}</td></tr>",
        i64_at(team, &["score"]).unwrap_or(0)
    ));
    html
}

fn game_details(game: &Value) -> String {
    let mut html = String::from(r#"<section class="details">"#);
    html.push_str(&team_game_details(game, false));
    html.push_str(&team_game_details(game, true));
    let away = str_at(game, &["awayTeam", "teamTricode"]).unwrap_or_default();
    let home = str_at(game, &["homeTeam", "teamTricode"]).unwrap_or_default();
    let id = str_at(game, &["gameId"]).unwrap_or_default();
    html.push_str(&format!(r#"<p><a href="https://www.nba.com/game/{}-vs-{}-{}?watchFullGame">Watch on League Pass</a></p>"#, escape_attr(&away), escape_attr(&home), escape_attr(&id)));
    html.push_str("</section>");
    html
}

fn team_game_details(game: &Value, is_home: bool) -> String {
    let team = game
        .get(if is_home { "homeTeam" } else { "awayTeam" })
        .unwrap_or(&Value::Null);
    let other = game
        .get(if is_home { "awayTeam" } else { "homeTeam" })
        .unwrap_or(&Value::Null);
    let mut html = String::from(r#"<article class="team-details">"#);
    html.push_str("<h1>");
    html.push_str(&team_logo(team, "logo"));
    html.push_str(&format!(
        "{} {} <strong>{}</strong> {}",
        escape(&str_at(team, &["teamCity"]).unwrap_or_default()),
        escape(&str_at(team, &["teamName"]).unwrap_or_default()),
        i64_at(team, &["score"]).unwrap_or(0),
        winner(game, is_home)
    ));
    html.push_str("</h1>");
    html.push_str(&team_box(team, other));
    html.push_str("</article>");
    html
}

fn team_box(team: &Value, other_team: &Value) -> String {
    let mut players = array_at(team, &["players"]);
    players.retain(|p| str_at(p, &["played"]).as_deref() == Some("1"));
    let mut html = String::from(
        r#"<div class="table-wrap"><table class="sortable"><thead><tr><th>Name</th><th>MIN</th><th>PTS</th><th>RB</th><th>AS</th><th>PIE</th><th>FG</th><th>3P</th><th>FT</th><th>PPS</th><th>TO</th><th>ST</th><th>BK</th><th>PF</th><th>+/-</th><th>USG</th></tr></thead><tbody>"#,
    );
    for p in players {
        let s = p.get("statistics").unwrap_or(&Value::Null);
        let person_id = i64_at(&p, &["personId"]).unwrap_or(0);
        html.push_str("<tr>");
        html.push_str(&format!(
            r#"<th><a href="/nba/player/{person_id}">{}{}</a></th>"#,
            escape(&str_at(&p, &["name"]).unwrap_or_default()),
            if str_at(&p, &["starter"]).as_deref() == Some("1") {
                "*"
            } else {
                ""
            }
        ));
        html.push_str(&format!("<td>{}</td>", minutes_num(s)));
        html.push_str(&stat_cell(i64_at(s, &["points"]).unwrap_or(0), 20, true));
        html.push_str(&stat_cell(
            i64_at(s, &["reboundsTotal"]).unwrap_or(0),
            10,
            true,
        ));
        html.push_str(&stat_cell(i64_at(s, &["assists"]).unwrap_or(0), 8, true));
        html.push_str(&format!(
            "<td>{}</td>",
            pie(
                s,
                team.get("statistics").unwrap_or(&Value::Null),
                other_team.get("statistics").unwrap_or(&Value::Null)
            )
        ));
        html.push_str(&format!("<td>{}</td>", shooting(s, "fieldGoals")));
        html.push_str(&format!("<td>{}</td>", shooting(s, "threePointers")));
        html.push_str(&format!("<td>{}</td>", shooting(s, "freeThrows")));
        html.push_str(&format!(
            "<td>{}</td>",
            points_per_shot(s)
                .map(|v| format!("{v:.2}"))
                .unwrap_or_default()
        ));
        html.push_str(&stat_cell(i64_at(s, &["turnovers"]).unwrap_or(0), 3, false));
        html.push_str(&stat_cell(i64_at(s, &["steals"]).unwrap_or(0), 3, true));
        html.push_str(&stat_cell(i64_at(s, &["blocks"]).unwrap_or(0), 3, true));
        html.push_str(&stat_cell(
            i64_at(s, &["foulsPersonal"]).unwrap_or(0),
            5,
            false,
        ));
        html.push_str(&format!(
            "<td>{}</td>",
            i64_at(s, &["plusMinusPoints"]).unwrap_or(0)
        ));
        html.push_str(&format!(
            "<td>{}</td>",
            usage_rate(s, team.get("statistics").unwrap_or(&Value::Null))
                .map(|v| v.to_string())
                .unwrap_or_default()
        ));
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");
    html
}

fn standings_page(standings: &Value) -> String {
    let rs = standings.pointer("/resultSets/0").unwrap_or(&Value::Null);
    let headers = array_at(rs, &["headers"]);
    let rows = array_at(rs, &["rowSet"]);
    let mut east = Vec::new();
    let mut west = Vec::new();
    for row in rows {
        if row_field(&headers, &row, "Conference").as_deref() == Some("West") {
            west.push(row);
        } else {
            east.push(row);
        }
    }
    format!(
        r#"<main class="page standings"><section>{}</section><section>{}</section></main>"#,
        standings_table("East", &headers, &east),
        standings_table("West", &headers, &west)
    )
}

fn standings_table(title: &str, headers: &[Value], rows: &[Value]) -> String {
    let mut html = format!(
        r#"<article class="panel"><h1>{}</h1><div class="table-wrap"><table class="sortable"><thead><tr><th>#</th><th>Team</th><th>W</th><th>L</th><th>%</th><th>GB</th><th>PPG</th><th>OPPG</th><th>DIFF</th><th>HM</th><th>RD</th><th>L10</th><th>STR</th></tr></thead><tbody>"#,
        escape(title)
    );
    for row in rows {
        let rank = row_field(headers, row, "PlayoffRank").unwrap_or_default();
        let team = row_field(headers, row, "TeamName").unwrap_or_default();
        let team_id = row_field(headers, row, "TeamID").unwrap_or_default();
        html.push_str("<tr>");
        html.push_str(&format!(
            "<td>{rank}</td><th>{}<span>{}</span></th>",
            team_logo_id(&team_id, &team, "mini-logo"),
            escape(&team)
        ));
        for name in [
            "WINS",
            "LOSSES",
            "WinPCT",
            "ConferenceGamesBack",
            "PointsPG",
            "OppPointsPG",
            "DiffPointsPG",
            "HOME",
            "ROAD",
            "L10",
            "CurrentStreak",
        ] {
            let val = row_field(headers, row, name).unwrap_or_default();
            let display = if name == "ConferenceGamesBack" && val == "0" {
                "-".to_string()
            } else if name == "CurrentStreak" {
                val.parse::<i64>()
                    .map(|n| {
                        if n < 0 {
                            format!("L{}", n.abs())
                        } else {
                            format!("W{}", n.abs())
                        }
                    })
                    .unwrap_or(val)
            } else {
                val
            };
            html.push_str(&format!("<td>{}</td>", escape(&display)));
        }
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div></article>");
    html
}

fn player_page(stats: &Value) -> String {
    let mut html = String::from(r#"<main class="page player">"#);
    for rs in array_at(stats, &["resultSets"]).into_iter().skip(5) {
        let title = str_at(&rs, &["name"]).unwrap_or_else(|| "Stats".to_string());
        let headers = array_at(&rs, &["headers"]);
        let rows = array_at(&rs, &["rowSet"]);
        html.push_str(&format!(
            r#"<article class="panel"><h1>{}</h1><div class="table-wrap"><table class="sortable"><thead><tr>"#,
            escape(&title)
        ));
        for h in &headers {
            html.push_str(&format!("<th>{}</th>", escape(value_to_string(h).as_str())));
        }
        html.push_str("</tr></thead><tbody>");
        for row in rows {
            html.push_str("<tr>");
            for cell in row.as_array().into_iter().flatten() {
                html.push_str(&format!("<td>{}</td>", escape(&value_to_string(cell))));
            }
            html.push_str("</tr>");
        }
        html.push_str("</tbody></table></div></article>");
    }
    html.push_str("</main>");
    html
}

fn error_page(status: StatusCode, title: &str, err: &anyhow::Error) -> Response {
    let body = layout(
        title,
        &format!(
            r#"<main class="center"><h1>{}</h1><p>{}</p></main>"#,
            escape(title),
            escape(&err.to_string())
        ),
    );
    (status, Html(body)).into_response()
}

fn espn_status_to_game_status(status: &Value) -> i64 {
    match str_at(status, &["type", "name"]).as_deref() {
        Some("STATUS_FINAL") => 3,
        Some("STATUS_IN_PROGRESS") => 2,
        _ => 1,
    }
}

fn espn_competitor_to_team(c: &Value) -> Value {
    let espn = str_at(c, &["team", "abbreviation"]).unwrap_or_default();
    let (tri, id, city, name) = match team_mapping(&espn) {
        Some((tri, id, city, name)) => (tri.to_string(), id, city.to_string(), name.to_string()),
        None => (
            espn.clone(),
            0,
            str_at(c, &["team", "location"]).unwrap_or_default(),
            str_at(c, &["team", "name"]).unwrap_or_default(),
        ),
    };
    let (wins, losses) = array_at(c, &["records"])
        .iter()
        .find(|r| str_at(r, &["type"]).as_deref() == Some("total"))
        .and_then(|r| str_at(r, &["summary"]))
        .map(|s| parse_record(&s))
        .unwrap_or((0, 0));
    let periods: Vec<Value> = array_at(c, &["linescores"]).into_iter().map(|ls| {
        let period = i64_at(&ls, &["period"]).unwrap_or(0);
        json!({"period": period, "periodType": if period <= 4 { "REGULAR" } else { "OVERTIME" }, "score": f64_at(&ls, &["value"]).unwrap_or(0.0) as i64})
    }).collect();
    json!({
        "teamId": id, "teamName": name, "teamCity": city, "teamTricode": tri,
        "wins": wins, "losses": losses, "score": str_at(c, &["score"]).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
        "inBonus": null, "timeoutsRemaining": 0, "periods": periods
    })
}

fn extract_leaders(c: &Value) -> Value {
    let leaders = array_at(c, &["leaders"]);
    let points = leader_value(&leaders, "points");
    let rebounds = leader_value(&leaders, "rebounds");
    let assists = leader_value(&leaders, "assists");
    let athlete = leaders
        .iter()
        .find(|l| str_at(l, &["name"]).as_deref() == Some("points"))
        .and_then(|l| l.pointer("/leaders/0/athlete"))
        .unwrap_or(&Value::Null);
    let tri = team_mapping(&str_at(c, &["team", "abbreviation"]).unwrap_or_default())
        .map(|m| m.0.to_string())
        .unwrap_or_default();
    json!({
        "personId": str_at(athlete, &["id"]).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
        "name": str_at(athlete, &["displayName"]).unwrap_or_default(),
        "jerseyNum": str_at(athlete, &["jersey"]).unwrap_or_default(),
        "position": str_at(athlete, &["position", "abbreviation"]).unwrap_or_default(),
        "teamTricode": tri, "points": points, "rebounds": rebounds, "assists": assists
    })
}

fn espn_summary_to_boxscore(data: &Value) -> Result<Value> {
    let header_comp = data
        .pointer("/header/competitions/0")
        .context("missing header competition")?;
    let status = header_comp.get("status").unwrap_or(&Value::Null);
    let competitors = array_at(header_comp, &["competitors"]);
    let home_comp = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("home"))
        .context("missing home")?;
    let away_comp = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("away"))
        .context("missing away")?;
    let home_abbr = str_at(home_comp, &["team", "abbreviation"]).unwrap_or_default();
    let away_abbr = str_at(away_comp, &["team", "abbreviation"]).unwrap_or_default();
    let home_team = summary_team(data, &home_abbr, home_comp);
    let away_team = summary_team(data, &away_abbr, away_comp);
    Ok(json!({
        "gameId": str_at(data, &["header", "id"]).unwrap_or_default(),
        "gameTimeLocal": str_at(header_comp, &["date"]).unwrap_or_default(),
        "gameTimeUTC": str_at(header_comp, &["date"]).unwrap_or_default(),
        "gameTimeHome": str_at(header_comp, &["date"]).unwrap_or_default(),
        "gameTimeAway": str_at(header_comp, &["date"]).unwrap_or_default(),
        "gameEt": str_at(header_comp, &["date"]).unwrap_or_default(),
        "duration": 0,
        "gameCode": "",
        "gameStatusText": str_at(status, &["type", "shortDetail"]).unwrap_or_default(),
        "gameStatus": espn_status_to_game_status(status),
        "regulationPeriods": 4,
        "period": i64_at(status, &["period"]).unwrap_or(0),
        "gameClock": str_at(status, &["displayClock"]).unwrap_or_default(),
        "attendance": i64_at(data, &["gameInfo", "attendance"]).unwrap_or(0),
        "sellout": "",
        "arena": {"arenaId": 0, "arenaName": str_at(data, &["gameInfo", "venue", "fullName"]).unwrap_or_default(), "arenaCity": str_at(data, &["gameInfo", "venue", "address", "city"]).unwrap_or_default(), "arenaState": str_at(data, &["gameInfo", "venue", "address", "state"]).unwrap_or_default(), "arenaCountry": str_at(data, &["gameInfo", "venue", "address", "country"]).unwrap_or_default(), "arenaTimezone": ""},
        "officials": [],
        "homeTeam": home_team,
        "awayTeam": away_team
    }))
}

fn summary_team(data: &Value, abbr: &str, comp: &Value) -> Value {
    let (tri, id, city, name) = team_mapping(abbr).unwrap_or((abbr, 0, "", ""));
    let team_stats = array_at(data, &["boxscore", "teams"])
        .into_iter()
        .find(|t| str_at(t, &["team", "abbreviation"]).as_deref() == Some(abbr))
        .map(|t| array_at(&t, &["statistics"]))
        .unwrap_or_default();
    let players = array_at(data, &["boxscore", "players"])
        .into_iter()
        .find(|t| str_at(t, &["team", "abbreviation"]).as_deref() == Some(abbr))
        .map(|t| summary_players(&t))
        .unwrap_or_default();
    json!({
        "teamId": id, "teamName": name, "teamCity": city, "teamTricode": tri,
        "score": str_at(comp, &["score"]).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
        "inBonus": "", "timeoutsRemaining": 0,
        "periods": array_at(comp, &["linescores"]).into_iter().enumerate().map(|(i, ls)| json!({"period": i + 1, "periodType": if i < 4 { "REGULAR" } else { "OVERTIME" }, "score": str_at(&ls, &["displayValue"]).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0)})).collect::<Vec<_>>(),
        "players": players,
        "statistics": summary_team_stats(&team_stats, str_at(comp, &["score"]).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0))
    })
}

fn summary_players(team: &Value) -> Vec<Value> {
    let mut out = Vec::new();
    for group in array_at(team, &["statistics"]) {
        let keys: Vec<String> = array_at(&group, &["keys"])
            .iter()
            .map(value_to_string)
            .collect();
        for (order, athlete) in array_at(&group, &["athletes"]).into_iter().enumerate() {
            let stats: Vec<String> = array_at(&athlete, &["stats"])
                .iter()
                .map(value_to_string)
                .collect();
            let person_id = str_at(&athlete, &["athlete", "id"])
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(0);
            let name = str_at(&athlete, &["athlete", "displayName"]).unwrap_or_default();
            out.push(json!({
                "status": "ACTIVE", "order": order, "personId": person_id,
                "jerseyNum": str_at(&athlete, &["athlete", "jersey"]).unwrap_or_default(),
                "position": str_at(&athlete, &["athlete", "position", "abbreviation"]).unwrap_or_default(),
                "starter": if bool_at(&athlete, &["starter"]) { "1" } else { "0" },
                "oncourt": "0", "played": if bool_at(&athlete, &["didNotPlay"]) { "0" } else { "1" },
                "statistics": player_statistics(&keys, &stats),
                "name": name, "nameI": "", "firstName": "", "familyName": ""
            }));
        }
    }
    out
}

fn player_statistics(keys: &[String], stats: &[String]) -> Value {
    let get = |key: &str| {
        keys.iter()
            .position(|k| k == key)
            .and_then(|i| stats.get(i))
            .cloned()
            .unwrap_or_else(|| "0".to_string())
    };
    let (fgm, fga) = parse_made_attempted(&get("fieldGoalsMade-fieldGoalsAttempted"));
    let (tpm, tpa) = parse_made_attempted(&get(
        "threePointFieldGoalsMade-threePointFieldGoalsAttempted",
    ));
    let (ftm, fta) = parse_made_attempted(&get("freeThrowsMade-freeThrowsAttempted"));
    let reb = num(&get("rebounds"));
    let oreb = num(&get("offensiveRebounds"));
    let dreb = num(&get("defensiveRebounds"));
    let plus_minus = get("plusMinus")
        .replace('+', "")
        .parse::<i64>()
        .unwrap_or(0);
    let minutes = get("minutes");
    let minute_num = minutes.parse::<i64>().unwrap_or(0);
    json!({
        "assists": num(&get("assists")), "blocks": num(&get("blocks")), "blocksReceived": 0,
        "fieldGoalsAttempted": fga, "fieldGoalsMade": fgm, "fieldGoalsPercentage": pct(fgm, fga),
        "foulsOffensive": 0, "foulsDrawn": 0, "foulsPersonal": num(&get("fouls")), "foulsTechnical": 0,
        "freeThrowsAttempted": fta, "freeThrowsMade": ftm, "freeThrowsPercentage": pct(ftm, fta),
        "minus": if plus_minus < 0 { plus_minus.abs() } else { 0 },
        "minutes": format!("{minutes}:00"), "minutesCalculated": format!("PT{minute_num}M"),
        "plus": if plus_minus > 0 { plus_minus } else { 0 }, "plusMinusPoints": plus_minus,
        "points": num(&get("points")), "pointsFastBreak": 0, "pointsInThePaint": 0, "pointsSecondChance": 0,
        "reboundsDefensive": dreb, "reboundsOffensive": oreb, "reboundsTotal": reb,
        "steals": num(&get("steals")), "threePointersAttempted": tpa, "threePointersMade": tpm, "threePointersPercentage": pct(tpm, tpa),
        "turnovers": num(&get("turnovers")), "twoPointersAttempted": fga - tpa, "twoPointersMade": fgm - tpm, "twoPointersPercentage": pct(fgm - tpm, fga - tpa)
    })
}

fn summary_team_stats(stats: &[Value], team_score: i64) -> Value {
    let (fgm, fga) = stat_split(stats, "fieldGoalsMade-fieldGoalsAttempted");
    let (tpm, tpa) = stat_split(
        stats,
        "threePointFieldGoalsMade-threePointFieldGoalsAttempted",
    );
    let (ftm, fta) = stat_split(stats, "freeThrowsMade-freeThrowsAttempted");
    json!({
        "assists": stat_num(stats, "assists"), "assistsTurnoverRatio": 0, "benchPoints": 0, "biggestLead": 0, "biggestLeadScore": "", "biggestScoringRun": 0, "biggestScoringRunScore": "",
        "blocks": stat_num(stats, "blocks"), "blocksReceived": 0, "fastBreakPointsAttempted": 0, "fastBreakPointsMade": 0, "fastBreakPointsPercentage": 0,
        "fieldGoalsAttempted": fga, "fieldGoalsEffectiveAdjusted": 0, "fieldGoalsMade": fgm, "fieldGoalsPercentage": pct(fgm, fga),
        "foulsOffensive": 0, "foulsDrawn": 0, "foulsPersonal": stat_num(stats, "fouls"), "foulsTeam": stat_num(stats, "fouls"), "foulsTechnical": 0, "foulsTeamTechnical": 0,
        "freeThrowsAttempted": fta, "freeThrowsMade": ftm, "freeThrowsPercentage": pct(ftm, fta), "leadChanges": 0,
        "minutes": "240:00", "minutesCalculated": "PT240M", "points": team_score, "pointsAgainst": 0, "pointsFastBreak": 0, "pointsFromTurnovers": 0,
        "pointsInThePaint": 0, "pointsInThePaintAttempted": 0, "pointsInThePaintMade": 0, "pointsInThePaintPercentage": 0, "pointsSecondChance": 0,
        "reboundsDefensive": stat_num(stats, "defensiveRebounds"), "reboundsOffensive": stat_num(stats, "offensiveRebounds"), "reboundsPersonal": 0, "reboundsTeam": 0, "reboundsTeamDefensive": 0, "reboundsTeamOffensive": 0, "reboundsTotal": stat_num(stats, "rebounds"),
        "secondChancePointsAttempted": 0, "secondChancePointsMade": 0, "secondChancePointsPercentage": 0, "steals": stat_num(stats, "steals"),
        "threePointersAttempted": tpa, "threePointersMade": tpm, "threePointersPercentage": pct(tpm, tpa), "timeLeading": "", "timesTied": 0, "trueShootingAttempts": 0, "trueShootingPercentage": 0,
        "turnovers": stat_num(stats, "turnovers"), "turnoversTeam": 0, "turnoversTotal": stat_num(stats, "turnovers"), "twoPointersAttempted": fga - tpa, "twoPointersMade": fgm - tpm, "twoPointersPercentage": pct(fgm - tpm, fga - tpa)
    })
}

fn espn_standings_to_result_set(data: &Value) -> Value {
    let headers = json!([
        "TeamID",
        "TeamName",
        "Conference",
        "PlayoffRank",
        "WINS",
        "LOSSES",
        "WinPCT",
        "ConferenceGamesBack",
        "PointsPG",
        "OppPointsPG",
        "DiffPointsPG",
        "HOME",
        "ROAD",
        "L10",
        "CurrentStreak"
    ]);
    let mut rows = Vec::new();
    for group in array_at(data, &["children"]) {
        let conf = str_at(&group, &["abbreviation"])
            .unwrap_or_else(|| str_at(&group, &["name"]).unwrap_or_default());
        for entry in array_at(&group, &["standings", "entries"]) {
            let abbr = str_at(&entry, &["team", "abbreviation"]).unwrap_or_default();
            let (id, name) = match team_mapping(&abbr) {
                Some((_, id, _, name)) => (id, name.to_string()),
                None => (
                    str_at(&entry, &["team", "id"])
                        .and_then(|s| s.parse::<i64>().ok())
                        .unwrap_or(0),
                    str_at(&entry, &["team", "name"]).unwrap_or_default(),
                ),
            };
            let stats = array_at(&entry, &["stats"]);
            let row = json!([
                id,
                name,
                if conf.to_lowercase().contains("west") {
                    "West"
                } else {
                    "East"
                },
                stat_value(&stats, "playoffSeed")
                    .or_else(|| stat_value(&stats, "rank"))
                    .unwrap_or(0.0),
                stat_value(&stats, "wins").unwrap_or(0.0),
                stat_value(&stats, "losses").unwrap_or(0.0),
                stat_value(&stats, "winPercent").unwrap_or(0.0),
                stat_value(&stats, "gamesBehind").unwrap_or(0.0),
                stat_value(&stats, "avgPointsFor").unwrap_or(0.0),
                stat_value(&stats, "avgPointsAgainst").unwrap_or(0.0),
                stat_value(&stats, "differential").unwrap_or(0.0),
                stat_display(&stats, "Home")
                    .or_else(|| stat_display(&stats, "home"))
                    .unwrap_or_default(),
                stat_display(&stats, "Road")
                    .or_else(|| stat_display(&stats, "road"))
                    .unwrap_or_default(),
                stat_display(&stats, "Last Ten Games")
                    .or_else(|| stat_display(&stats, "lastTen"))
                    .unwrap_or_default(),
                stat_value(&stats, "streak").unwrap_or(0.0)
            ]);
            rows.push(row);
        }
    }
    json!({"resource": "standings", "parameters": {}, "resultSets": [{"name": "Standings", "headers": headers, "rowSet": rows}]})
}

fn team_mapping(abbr: &str) -> Option<(&'static str, i64, &'static str, &'static str)> {
    Some(match abbr {
        "ATL" => ("ATL", 1610612737, "Atlanta", "Hawks"),
        "BOS" => ("BOS", 1610612738, "Boston", "Celtics"),
        "BKN" => ("BKN", 1610612751, "Brooklyn", "Nets"),
        "CHA" => ("CHA", 1610612766, "Charlotte", "Hornets"),
        "CHI" => ("CHI", 1610612741, "Chicago", "Bulls"),
        "CLE" => ("CLE", 1610612739, "Cleveland", "Cavaliers"),
        "DAL" => ("DAL", 1610612742, "Dallas", "Mavericks"),
        "DEN" => ("DEN", 1610612743, "Denver", "Nuggets"),
        "DET" => ("DET", 1610612765, "Detroit", "Pistons"),
        "GS" | "GSW" => ("GSW", 1610612744, "Golden State", "Warriors"),
        "HOU" => ("HOU", 1610612745, "Houston", "Rockets"),
        "IND" => ("IND", 1610612754, "Indiana", "Pacers"),
        "LAC" => ("LAC", 1610612746, "LA", "Clippers"),
        "LAL" => ("LAL", 1610612747, "Los Angeles", "Lakers"),
        "MEM" => ("MEM", 1610612763, "Memphis", "Grizzlies"),
        "MIA" => ("MIA", 1610612748, "Miami", "Heat"),
        "MIL" => ("MIL", 1610612749, "Milwaukee", "Bucks"),
        "MIN" => ("MIN", 1610612750, "Minnesota", "Timberwolves"),
        "NO" | "NOP" => ("NOP", 1610612740, "New Orleans", "Pelicans"),
        "NY" | "NYK" => ("NYK", 1610612752, "New York", "Knicks"),
        "OKC" => ("OKC", 1610612760, "Oklahoma City", "Thunder"),
        "ORL" => ("ORL", 1610612753, "Orlando", "Magic"),
        "PHI" => ("PHI", 1610612755, "Philadelphia", "76ers"),
        "PHX" => ("PHX", 1610612756, "Phoenix", "Suns"),
        "POR" => ("POR", 1610612757, "Portland", "Trail Blazers"),
        "SAC" => ("SAC", 1610612758, "Sacramento", "Kings"),
        "SA" | "SAS" => ("SAS", 1610612759, "San Antonio", "Spurs"),
        "TOR" => ("TOR", 1610612761, "Toronto", "Raptors"),
        "UTAH" | "UTA" => ("UTA", 1610612762, "Utah", "Jazz"),
        "WSH" | "WAS" => ("WAS", 1610612764, "Washington", "Wizards"),
        _ => return None,
    })
}

fn team_logo(team: &Value, class: &str) -> String {
    team_logo_id(
        &i64_at(team, &["teamId"]).unwrap_or(0).to_string(),
        &str_at(team, &["teamName"]).unwrap_or_default(),
        class,
    )
}

fn team_logo_id(team_id: &str, team_name: &str, class: &str) -> String {
    format!(
        r#"<img class="{class}" src="https://cdn.nba.com/logos/nba/{}/primary/L/logo.svg" alt="{}">"#,
        escape_attr(team_id),
        escape_attr(team_name)
    )
}

fn winner(game: &Value, is_home: bool) -> &'static str {
    if i64_at(game, &["gameStatus"]) != Some(3) {
        return "";
    }
    let home = i64_at(game, &["homeTeam", "score"]).unwrap_or(0);
    let away = i64_at(game, &["awayTeam", "score"]).unwrap_or(0);
    if (is_home && home > away) || (!is_home && away > home) {
        "<strong>W</strong>"
    } else {
        ""
    }
}

fn shooting(stats: &Value, prefix: &str) -> String {
    let made = i64_at(stats, &[&format!("{prefix}Made")]).unwrap_or(0);
    let attempted = i64_at(stats, &[&format!("{prefix}Attempted")]).unwrap_or(0);
    format!("{made}-{attempted}")
}

fn points_per_shot(s: &Value) -> Option<f64> {
    let fga = f64_at(s, &["fieldGoalsAttempted"])?;
    if fga <= 0.0 {
        None
    } else {
        Some((f64_at(s, &["points"]).unwrap_or(0.0) * 100.0 / fga).round() / 100.0)
    }
}

fn usage_rate(s: &Value, team: &Value) -> Option<i64> {
    let minutes = minutes_num(s) as f64;
    if minutes <= 0.0 {
        return None;
    }
    let numerator = (f64_at(s, &["fieldGoalsAttempted"]).unwrap_or(0.0)
        + 0.44 * f64_at(s, &["freeThrowsAttempted"]).unwrap_or(0.0)
        + f64_at(s, &["turnovers"]).unwrap_or(0.0))
        * (minutes_num(team) as f64 / 5.0);
    let denominator = minutes
        * (f64_at(team, &["fieldGoalsAttempted"]).unwrap_or(0.0)
            + 0.44 * f64_at(team, &["freeThrowsAttempted"]).unwrap_or(0.0)
            + f64_at(team, &["turnovers"]).unwrap_or(0.0));
    if denominator == 0.0 {
        None
    } else {
        Some((100.0 * numerator / denominator).round() as i64)
    }
}

fn pie(s: &Value, team: &Value, other: &Value) -> i64 {
    let player = f64_at(s, &["points"]).unwrap_or(0.0)
        + f64_at(s, &["fieldGoalsMade"]).unwrap_or(0.0)
        + f64_at(s, &["freeThrowsMade"]).unwrap_or(0.0)
        - f64_at(s, &["fieldGoalsAttempted"]).unwrap_or(0.0)
        - f64_at(s, &["freeThrowsAttempted"]).unwrap_or(0.0)
        + f64_at(s, &["reboundsDefensive"]).unwrap_or(0.0)
        + f64_at(s, &["reboundsOffensive"]).unwrap_or(0.0) / 2.0
        + f64_at(s, &["assists"]).unwrap_or(0.0) / 2.0
        + f64_at(s, &["steals"]).unwrap_or(0.0)
        + f64_at(s, &["blocks"]).unwrap_or(0.0) / 2.0
        - f64_at(s, &["blocksReceived"]).unwrap_or(0.0) / 2.0
        - f64_at(s, &["foulsPersonal"]).unwrap_or(0.0)
        - f64_at(s, &["turnovers"]).unwrap_or(0.0);
    let total = [
        "points",
        "fieldGoalsMade",
        "freeThrowsMade",
        "reboundsDefensive",
        "steals",
    ]
    .iter()
    .map(|k| f64_at(team, &[k]).unwrap_or(0.0) + f64_at(other, &[k]).unwrap_or(0.0))
    .sum::<f64>()
        - [
            "fieldGoalsAttempted",
            "freeThrowsAttempted",
            "foulsPersonal",
            "turnovers",
        ]
        .iter()
        .map(|k| f64_at(team, &[k]).unwrap_or(0.0) + f64_at(other, &[k]).unwrap_or(0.0))
        .sum::<f64>()
        + (f64_at(team, &["reboundsOffensive"]).unwrap_or(0.0)
            + f64_at(other, &["reboundsOffensive"]).unwrap_or(0.0))
            / 2.0
        + (f64_at(team, &["assists"]).unwrap_or(0.0) + f64_at(other, &["assists"]).unwrap_or(0.0))
            / 2.0
        + (f64_at(team, &["blocks"]).unwrap_or(0.0) + f64_at(other, &["blocks"]).unwrap_or(0.0))
            / 2.0
        - (f64_at(team, &["blocksReceived"]).unwrap_or(0.0)
            + f64_at(other, &["blocksReceived"]).unwrap_or(0.0))
            / 2.0;
    if total == 0.0 {
        0
    } else {
        (100.0 * player / total).round() as i64
    }
}

fn stat_cell(value: i64, threshold: i64, good_when_high: bool) -> String {
    let class = if (good_when_high && value >= threshold) || (!good_when_high && value < threshold)
    {
        "good"
    } else if !good_when_high && value >= threshold {
        "bad"
    } else {
        ""
    };
    format!(r#"<td class="{class}">{value}</td>"#)
}

fn minutes_num(v: &Value) -> i64 {
    str_at(v, &["minutesCalculated"])
        .and_then(|s| {
            s.strip_prefix("PT")
                .and_then(|s| s.strip_suffix('M'))
                .map(str::to_string)
        })
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn stat_num(stats: &[Value], name: &str) -> i64 {
    stats
        .iter()
        .find(|s| str_at(s, &["name"]).as_deref() == Some(name))
        .and_then(|s| str_at(s, &["displayValue"]))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn stat_split(stats: &[Value], name: &str) -> (i64, i64) {
    stats
        .iter()
        .find(|s| str_at(s, &["name"]).as_deref() == Some(name))
        .and_then(|s| str_at(s, &["displayValue"]))
        .map(|s| parse_made_attempted(&s))
        .unwrap_or((0, 0))
}

fn stat_value(stats: &[Value], name: &str) -> Option<f64> {
    stats
        .iter()
        .find(|s| str_at(s, &["name"]).as_deref() == Some(name))
        .and_then(|s| f64_at(s, &["value"]))
}

fn stat_display(stats: &[Value], name: &str) -> Option<String> {
    stats
        .iter()
        .find(|s| str_at(s, &["name"]).as_deref() == Some(name))
        .and_then(|s| str_at(s, &["displayValue"]))
}

fn leader_value(leaders: &[Value], name: &str) -> f64 {
    leaders
        .iter()
        .find(|l| str_at(l, &["name"]).as_deref() == Some(name))
        .and_then(|l| {
            str_at(l, &["leaders", "0", "displayValue"])
                .or_else(|| l.pointer("/leaders/0/displayValue").map(value_to_string))
        })
        .and_then(|s| s.split_whitespace().next().and_then(|n| n.parse().ok()))
        .unwrap_or(0.0)
}

fn row_field(headers: &[Value], row: &Value, name: &str) -> Option<String> {
    headers
        .iter()
        .position(|h| h.as_str() == Some(name))
        .and_then(|i| row.as_array()?.get(i))
        .map(value_to_string)
}

fn str_at(v: &Value, path: &[&str]) -> Option<String> {
    let mut current = v;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str().map(str::to_string)
}

fn i64_at(v: &Value, path: &[&str]) -> Option<i64> {
    let mut current = v;
    for key in path {
        current = current.get(*key)?;
    }
    current
        .as_i64()
        .or_else(|| current.as_f64().map(|n| n as i64))
        .or_else(|| current.as_str()?.parse().ok())
}

fn f64_at(v: &Value, path: &[&str]) -> Option<f64> {
    let mut current = v;
    for key in path {
        current = current.get(*key)?;
    }
    current
        .as_f64()
        .or_else(|| current.as_i64().map(|n| n as f64))
        .or_else(|| current.as_str()?.parse().ok())
}

fn bool_at(v: &Value, path: &[&str]) -> bool {
    let mut current = v;
    for key in path {
        let Some(next) = current.get(*key) else {
            return false;
        };
        current = next;
    }
    current.as_bool().unwrap_or(false)
}

fn array_at(v: &Value, path: &[&str]) -> Vec<Value> {
    let mut current = v;
    for key in path {
        let Some(next) = current.get(*key) else {
            return Vec::new();
        };
        current = next;
    }
    current.as_array().cloned().unwrap_or_default()
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        _ => v.to_string(),
    }
}

fn parse_record(summary: &str) -> (i64, i64) {
    let mut parts = summary.split('-');
    (
        parts.next().and_then(|s| s.parse().ok()).unwrap_or(0),
        parts.next().and_then(|s| s.parse().ok()).unwrap_or(0),
    )
}

fn parse_made_attempted(val: &str) -> (i64, i64) {
    let mut parts = val.split('-');
    (
        num(parts.next().unwrap_or("0")),
        num(parts.next().unwrap_or("0")),
    )
}

fn pct(made: i64, attempted: i64) -> f64 {
    if attempted > 0 {
        ((made as f64 / attempted as f64) * 1000.0).round() / 10.0
    } else {
        0.0
    }
}

fn num(s: &str) -> i64 {
    s.parse::<f64>().map(|n| n as i64).unwrap_or(0)
}

fn today_ymd() -> String {
    Utc::now().date_naive().to_string()
}

fn shift_day(day: &str, offset: i64) -> Option<String> {
    NaiveDate::parse_from_str(day, "%Y-%m-%d")
        .ok()
        .map(|d| (d + chrono::Duration::days(offset)).to_string())
}

fn weekday(n: u32) -> &'static str {
    ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"][n as usize]
}

fn escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_attr(input: &str) -> String {
    escape(input).replace('"', "&quot;")
}

const TABLE_SORT_SCRIPT: &str = r#"
(() => {
  const valueFor = (cell) => {
    const text = (cell?.innerText || "").trim();
    if (!text || text === "-") return { kind: "empty", value: "" };
    const record = text.match(/^(\d+)-(\d+)$/);
    if (record) {
      const wins = Number(record[1]);
      const losses = Number(record[2]);
      return { kind: "number", value: wins / Math.max(1, wins + losses) };
    }
    const streak = text.match(/^[WL](-?\d+)$/i);
    if (streak) {
      const amount = Number(streak[1]);
      return { kind: "number", value: text[0].toUpperCase() === "W" ? amount : -amount };
    }
    const numeric = Number(text.replace(/[%,$]/g, ""));
    if (Number.isFinite(numeric) && /^[-+]?[\d,.]+%?$/.test(text)) {
      return { kind: "number", value: numeric };
    }
    return { kind: "text", value: text.toLocaleLowerCase() };
  };

  document.querySelectorAll("table.sortable").forEach((table) => {
    const headers = Array.from(table.tHead?.rows?.[0]?.cells || []);
    const body = table.tBodies[0];
    if (!body) return;

    headers.forEach((header, index) => {
      header.tabIndex = 0;
      header.setAttribute("role", "button");
      header.setAttribute("aria-sort", "none");

      const sort = () => {
        const nextDir = header.dataset.sortDir === "asc" ? "desc" : "asc";
        headers.forEach((h) => {
          h.dataset.sortDir = "";
          h.setAttribute("aria-sort", "none");
        });
        header.dataset.sortDir = nextDir;
        header.setAttribute("aria-sort", nextDir === "asc" ? "ascending" : "descending");

        const rows = Array.from(body.rows);
        rows.sort((a, b) => {
          const left = valueFor(a.cells[index]);
          const right = valueFor(b.cells[index]);
          if (left.kind === "empty" && right.kind !== "empty") return 1;
          if (right.kind === "empty" && left.kind !== "empty") return -1;
          const result = left.kind === "number" && right.kind === "number"
            ? left.value - right.value
            : String(left.value).localeCompare(String(right.value), undefined, { numeric: true });
          return nextDir === "asc" ? result : -result;
        });
        rows.forEach((row) => body.appendChild(row));
      };

      header.addEventListener("click", sort);
      header.addEventListener("keydown", (event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          sort();
        }
      });
    });
  });
})();
"#;

const CSS: &str = r#"
:root { color-scheme: light; --bg:#e7ebee; --panel:#fff; --ink:#171b21; --muted:#5d6672; --line:#d5dbe1; --accent:#15181d; --good:#0c7a43; --bad:#b42318; }
* { box-sizing: border-box; }
body { margin:0; min-height:100vh; display:flex; flex-direction:column; background:var(--bg); color:var(--ink); font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; }
a { color:inherit; }
.page { flex:1; padding:14px; }
.nav { position:sticky; top:0; z-index:10; display:flex; gap:8px; align-items:center; justify-content:center; padding:10px; border-bottom:1px solid var(--line); background:rgba(255,255,255,.9); backdrop-filter: blur(10px); }
.nav a, .button { border:1px solid transparent; border-radius:6px; padding:7px 10px; text-decoration:none; background:#f4f6f7; white-space:nowrap; }
.nav a:hover, .button:hover, .button.active { background:var(--accent); color:white; }
.brand { font-size:20px; padding:0 8px; }
.date-nav { display:flex; gap:6px; justify-content:center; align-items:center; flex-wrap:wrap; margin-bottom:14px; }
.scoreboard { display:flex; gap:14px; align-items:flex-start; width:100%; }
.game-list { display:flex; flex-wrap:wrap; gap:12px; align-content:flex-start; }
.scoreboard.has-game { flex-wrap:nowrap; }
.scoreboard.has-game .game-list { flex:0 0 420px; max-width:420px; flex-direction:column; flex-wrap:nowrap; }
.scoreboard.has-game .game-link, .scoreboard.has-game .game-card { width:100%; }
.game-link { text-decoration:none; }
.game-card, table { border-collapse:collapse; background:var(--panel); }
.game-card { width:404px; max-width:100%; box-shadow:0 1px 6px rgba(0,0,0,.12); table-layout:auto; }
th, td { padding:6px 8px; border-bottom:1px solid var(--line); text-align:right; }
th { text-align:left; font-weight:700; }
thead th { font-size:12px; color:var(--muted); }
table.sortable thead th { cursor:pointer; user-select:none; white-space:nowrap; }
table.sortable thead th::after { content:""; display:inline-block; width:1em; color:var(--muted); }
table.sortable thead th[aria-sort="ascending"]::after { content:"▲"; font-size:10px; }
table.sortable thead th[aria-sort="descending"]::after { content:"▼"; font-size:10px; }
.status { text-align:right; color:var(--muted); }
.mini-logo { width:22px; height:22px; vertical-align:middle; margin-right:5px; }
.logo { width:28px; height:28px; vertical-align:middle; margin-right:8px; }
.details { display:flex; flex-direction:column; gap:12px; min-width:0; flex:1 1 auto; }
.team-details, .panel { background:var(--panel); padding:12px; box-shadow:0 1px 6px rgba(0,0,0,.12); }
.team-details h1 { margin:0 0 10px; font-size:22px; display:flex; align-items:center; gap:4px; }
.table-wrap { overflow:auto; max-width:100%; }
.table-wrap table { min-width:760px; width:100%; font-size:13px; }
.good { color:var(--good); font-weight:700; }
.bad { color:var(--bad); font-weight:700; }
.standings { display:flex; flex-wrap:wrap; gap:14px; align-items:flex-start; }
.standings .panel { flex:1 1 520px; }
.player { display:flex; flex-direction:column; gap:14px; }
.center { min-height:60vh; display:grid; place-items:center; text-align:center; padding:24px; }
@media (max-width: 900px) {
  .scoreboard { flex-direction:column; }
  .scoreboard.has-game .game-list, .game-list { width:100%; max-width:none; flex-basis:auto; }
  .game-link, .game-card { width:100%; }
  .nav { overflow:auto; justify-content:flex-start; }
  .brand { flex:0 0 auto; }
}
"#;
