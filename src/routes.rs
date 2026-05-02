use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::Local;

use crate::{
    app::AppState,
    error::AppError,
    render,
    validation::{nfl_week, numeric_id, parse_day},
};

pub async fn index() -> Redirect {
    Redirect::temporary("/nba/scoreboard")
}

pub async fn healthcheck() -> &'static str {
    "OK"
}

pub async fn nba_scoreboard() -> Redirect {
    Redirect::temporary("/nba/scoreboard/today")
}

pub async fn nba_scoreboard_today(State(state): State<AppState>) -> Result<Response, AppError> {
    let parsed_day = Local::now().date_naive();
    let scoreboard = state.data.days_games(&parsed_day.to_string()).await?;
    Ok(Html(render::todays_scoreboard_page(parsed_day, &scoreboard)).into_response())
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

pub async fn mlb_scoreboard() -> Redirect {
    Redirect::temporary("/mlb/scoreboard/today")
}

pub async fn mlb_scoreboard_today(State(state): State<AppState>) -> Result<Response, AppError> {
    let parsed_day = Local::now().date_naive();
    let scoreboard = state.data.mlb_days_games(&parsed_day.to_string()).await?;
    Ok(Html(render::mlb_todays_scoreboard_page(parsed_day, &scoreboard)).into_response())
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

pub async fn nfl_scoreboard() -> Redirect {
    Redirect::temporary("/nfl/scoreboard/today")
}

pub async fn nfl_scoreboard_today(State(state): State<AppState>) -> Result<Response, AppError> {
    let scoreboard = state.data.nfl_current_scoreboard().await?;
    let week = nfl_week(&scoreboard.game_date)?;
    Ok(Html(render::nfl_scoreboard_page(week, &scoreboard, None)).into_response())
}

pub async fn nfl_scoreboard_week(
    State(state): State<AppState>,
    Path(week): Path<String>,
) -> Result<Html<String>, AppError> {
    let week = nfl_week(&week)?;
    let scoreboard = state.data.nfl_week_games(week).await?;
    Ok(Html(render::nfl_scoreboard_page(week, &scoreboard, None)))
}

pub async fn nfl_game(
    State(state): State<AppState>,
    Path((week, game_id)): Path<(String, String)>,
) -> Result<Html<String>, AppError> {
    let week = nfl_week(&week)?;
    let game_id = numeric_id(&game_id, "game_id")?;
    let scoreboard = state.data.nfl_week_games(week).await?;
    let game = state.data.nfl_game(&game_id).await?;
    Ok(Html(render::nfl_scoreboard_page(
        week,
        &scoreboard,
        game.as_ref(),
    )))
}

pub async fn nfl_standings(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let standings = state.data.nfl_standings().await?;
    Ok(Html(render::nfl_standings_page(&standings)))
}
