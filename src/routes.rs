use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::Utc;

use crate::{
    app::AppState,
    error::AppError,
    render,
    validation::{numeric_id, parse_day},
};

pub async fn index() -> Redirect {
    Redirect::temporary("/nba/scoreboard")
}

pub async fn healthcheck() -> &'static str {
    "OK"
}

pub async fn nba_scoreboard(State(state): State<AppState>) -> Result<Response, AppError> {
    let scoreboard = state.data.todays_scoreboard().await?;
    let has_live_or_completed = scoreboard.games.iter().any(|game| game.game_status >= 2);
    let target = if has_live_or_completed {
        scoreboard.game_date
    } else {
        parse_day(&scoreboard.game_date)?
            .checked_sub_days(chrono::Days::new(1))
            .unwrap_or_else(|| Utc::now().date_naive())
            .to_string()
    };
    Ok(Redirect::temporary(&format!("/nba/scoreboard/{target}")).into_response())
}

pub async fn nba_scoreboard_today(State(state): State<AppState>) -> Result<Response, AppError> {
    let scoreboard = state.data.todays_scoreboard().await?;
    Ok(Redirect::temporary(&format!("/nba/scoreboard/{}", scoreboard.game_date)).into_response())
}

pub async fn nba_scoreboard_day(
    State(state): State<AppState>,
    Path(day): Path<String>,
) -> Result<Html<String>, AppError> {
    let parsed_day = parse_day(&day)?;
    let scoreboard = state.data.days_games(&day).await?;
    Ok(Html(render::scoreboard_page(parsed_day, &scoreboard, None)))
}

pub async fn nba_game(
    State(state): State<AppState>,
    Path((day, game_id)): Path<(String, String)>,
) -> Result<Html<String>, AppError> {
    let parsed_day = parse_day(&day)?;
    let game_id = numeric_id(&game_id, "game_id")?;
    let scoreboard = state.data.days_games(&day).await?;
    let game = state.data.game(&game_id).await?;
    Ok(Html(render::scoreboard_page(
        parsed_day,
        &scoreboard,
        game.as_ref(),
    )))
}

pub async fn nba_standings(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let standings = state.data.standings().await?;
    Ok(Html(render::standings_page(&standings)))
}

pub async fn nba_player(
    State(state): State<AppState>,
    Path(player_id): Path<String>,
) -> Result<Html<String>, AppError> {
    let player_id = numeric_id(&player_id, "player_id")?;
    let stats = state.data.player_stats(&player_id).await?;
    Ok(Html(render::player_page(&stats)))
}

pub async fn mlb_scoreboard(State(state): State<AppState>) -> Result<Response, AppError> {
    let scoreboard = state.data.mlb_todays_scoreboard().await?;
    let has_live_or_completed = scoreboard.games.iter().any(|game| game.game_status >= 2);
    let target = if has_live_or_completed {
        scoreboard.game_date
    } else {
        parse_day(&scoreboard.game_date)?
            .checked_sub_days(chrono::Days::new(1))
            .unwrap_or_else(|| Utc::now().date_naive())
            .to_string()
    };
    Ok(Redirect::temporary(&format!("/mlb/scoreboard/{target}")).into_response())
}

pub async fn mlb_scoreboard_today(State(state): State<AppState>) -> Result<Response, AppError> {
    let scoreboard = state.data.mlb_todays_scoreboard().await?;
    Ok(Redirect::temporary(&format!("/mlb/scoreboard/{}", scoreboard.game_date)).into_response())
}

pub async fn mlb_scoreboard_day(
    State(state): State<AppState>,
    Path(day): Path<String>,
) -> Result<Html<String>, AppError> {
    let parsed_day = parse_day(&day)?;
    let scoreboard = state.data.mlb_days_games(&day).await?;
    Ok(Html(render::mlb_scoreboard_page(
        parsed_day,
        &scoreboard,
        None,
    )))
}

pub async fn mlb_game(
    State(state): State<AppState>,
    Path((day, game_id)): Path<(String, String)>,
) -> Result<Html<String>, AppError> {
    let parsed_day = parse_day(&day)?;
    let game_id = numeric_id(&game_id, "game_id")?;
    let scoreboard = state.data.mlb_days_games(&day).await?;
    let game = state.data.mlb_game(&game_id).await?;
    Ok(Html(render::mlb_scoreboard_page(
        parsed_day,
        &scoreboard,
        game.as_ref(),
    )))
}

pub async fn mlb_standings(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let standings = state.data.mlb_standings().await?;
    Ok(Html(render::mlb_standings_page(&standings)))
}

pub async fn coming_soon() -> Html<String> {
    Html(render::coming_soon_page())
}
