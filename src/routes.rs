use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect, Response},
};
use chrono::{Days, Local, NaiveDate};

use crate::{
    app::AppState,
    error::AppError,
    models::Scoreboard,
    render,
    validation::{nfl_week, numeric_id, parse_day},
};

const TODAY_LOOKBACK_DAYS: u64 = 7;

pub async fn index() -> Redirect {
    Redirect::temporary("/nba/scoreboard")
}

pub async fn healthcheck() -> &'static str {
    "OK"
}

pub async fn nba_scoreboard() -> Redirect {
    dayless_scoreboard(RouteLeague::Nba)
}

pub async fn nba_scoreboard_today(State(state): State<AppState>) -> Result<Response, AppError> {
    date_scoreboard_today(&state, RouteLeague::Nba).await
}

pub async fn nba_scoreboard_day(
    State(state): State<AppState>,
    Path(day): Path<String>,
) -> Result<Html<String>, AppError> {
    date_scoreboard_day(&state, RouteLeague::Nba, &day).await
}

pub async fn nba_game(
    State(state): State<AppState>,
    Path((day, game_id)): Path<(String, String)>,
) -> Result<Html<String>, AppError> {
    date_game(&state, RouteLeague::Nba, &day, &game_id).await
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

pub async fn wnba_scoreboard() -> Redirect {
    dayless_scoreboard(RouteLeague::Wnba)
}

pub async fn wnba_scoreboard_today(State(state): State<AppState>) -> Result<Response, AppError> {
    date_scoreboard_today(&state, RouteLeague::Wnba).await
}

pub async fn wnba_scoreboard_day(
    State(state): State<AppState>,
    Path(day): Path<String>,
) -> Result<Html<String>, AppError> {
    date_scoreboard_day(&state, RouteLeague::Wnba, &day).await
}

pub async fn wnba_game(
    State(state): State<AppState>,
    Path((day, game_id)): Path<(String, String)>,
) -> Result<Html<String>, AppError> {
    date_game(&state, RouteLeague::Wnba, &day, &game_id).await
}

pub async fn wnba_standings(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let standings = state.data.wnba_standings().await?;
    Ok(Html(render::wnba_standings_page(&standings)))
}

pub async fn wnba_player(Path(player_id): Path<String>) -> Result<Html<String>, AppError> {
    numeric_id(&player_id, "player_id")?;
    Ok(Html(render::unsupported_player_page("WNBA")))
}

pub async fn mlb_scoreboard() -> Redirect {
    dayless_scoreboard(RouteLeague::Mlb)
}

pub async fn mlb_scoreboard_today(State(state): State<AppState>) -> Result<Response, AppError> {
    date_scoreboard_today(&state, RouteLeague::Mlb).await
}

pub async fn mlb_scoreboard_day(
    State(state): State<AppState>,
    Path(day): Path<String>,
) -> Result<Html<String>, AppError> {
    date_scoreboard_day(&state, RouteLeague::Mlb, &day).await
}

pub async fn mlb_game(
    State(state): State<AppState>,
    Path((day, game_id)): Path<(String, String)>,
) -> Result<Html<String>, AppError> {
    date_game(&state, RouteLeague::Mlb, &day, &game_id).await
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

pub async fn nhl_scoreboard() -> Redirect {
    dayless_scoreboard(RouteLeague::Nhl)
}

pub async fn nhl_scoreboard_today(State(state): State<AppState>) -> Result<Response, AppError> {
    date_scoreboard_today(&state, RouteLeague::Nhl).await
}

pub async fn nhl_scoreboard_day(
    State(state): State<AppState>,
    Path(day): Path<String>,
) -> Result<Html<String>, AppError> {
    date_scoreboard_day(&state, RouteLeague::Nhl, &day).await
}

pub async fn nhl_game(
    State(state): State<AppState>,
    Path((day, game_id)): Path<(String, String)>,
) -> Result<Html<String>, AppError> {
    date_game(&state, RouteLeague::Nhl, &day, &game_id).await
}

pub async fn nhl_standings(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let standings = state.data.nhl_standings().await?;
    Ok(Html(render::nhl_standings_page(&standings)))
}

async fn date_today_scoreboard(
    state: &AppState,
    league: RouteLeague,
) -> Result<(NaiveDate, Scoreboard), AppError> {
    let feed_day = Local::now().date_naive();
    let scoreboard = days_games(state, league, &feed_day.to_string()).await?;
    let scoreboard_day = parse_day(&scoreboard.game_date).unwrap_or(feed_day);
    if has_live_or_completed_games(&scoreboard) {
        return Ok((scoreboard_day, scoreboard));
    }

    for offset in 1..=TODAY_LOOKBACK_DAYS {
        let Some(day) = feed_day.checked_sub_days(Days::new(offset)) else {
            break;
        };
        let scoreboard = days_games(state, league, &day.to_string()).await?;
        let scoreboard_day = parse_day(&scoreboard.game_date).unwrap_or(day);
        if has_live_or_completed_games(&scoreboard) {
            return Ok((scoreboard_day, scoreboard));
        }
    }
    Ok((scoreboard_day, scoreboard))
}

fn has_live_or_completed_games(scoreboard: &Scoreboard) -> bool {
    scoreboard.games.iter().any(|game| game.game_status >= 2)
}

#[derive(Clone, Copy)]
enum RouteLeague {
    Nba,
    Wnba,
    Mlb,
    Nhl,
}

impl RouteLeague {
    fn scoreboard_path(self) -> &'static str {
        match self {
            Self::Nba => "/nba/scoreboard",
            Self::Wnba => "/wnba/scoreboard",
            Self::Mlb => "/mlb/scoreboard",
            Self::Nhl => "/nhl/scoreboard",
        }
    }
}

fn dayless_scoreboard(league: RouteLeague) -> Redirect {
    Redirect::temporary(&format!("{}/today", league.scoreboard_path()))
}

async fn date_scoreboard_today(
    state: &AppState,
    league: RouteLeague,
) -> Result<Response, AppError> {
    let (today_day, scoreboard) = today_scoreboard(state, league).await?;
    let html = match league {
        RouteLeague::Nba => {
            render::scoreboard_page_with_today(today_day, &scoreboard, None, today_day)
        }
        RouteLeague::Wnba => {
            render::wnba_scoreboard_page_with_today(today_day, &scoreboard, None, today_day)
        }
        RouteLeague::Mlb => {
            render::mlb_scoreboard_page_with_today(today_day, &scoreboard, None, today_day)
        }
        RouteLeague::Nhl => {
            render::nhl_scoreboard_page_with_today(today_day, &scoreboard, None, today_day)
        }
    };
    Ok(Html(html).into_response())
}

async fn date_scoreboard_day(
    state: &AppState,
    league: RouteLeague,
    day: &str,
) -> Result<Html<String>, AppError> {
    let parsed_day = parse_day(day)?;
    let scoreboard = days_games(state, league, day).await?;
    let today_day = today_scoreboard(state, league).await?.0;
    let html = match league {
        RouteLeague::Nba => {
            render::scoreboard_page_with_today(parsed_day, &scoreboard, None, today_day)
        }
        RouteLeague::Wnba => {
            render::wnba_scoreboard_page_with_today(parsed_day, &scoreboard, None, today_day)
        }
        RouteLeague::Mlb => {
            render::mlb_scoreboard_page_with_today(parsed_day, &scoreboard, None, today_day)
        }
        RouteLeague::Nhl => {
            render::nhl_scoreboard_page_with_today(parsed_day, &scoreboard, None, today_day)
        }
    };
    Ok(Html(html))
}

async fn date_game(
    state: &AppState,
    league: RouteLeague,
    day: &str,
    game_id: &str,
) -> Result<Html<String>, AppError> {
    let parsed_day = parse_day(day)?;
    let game_id = numeric_id(game_id, "game_id")?;
    let scoreboard = days_games(state, league, day).await?;
    let today_day = today_scoreboard(state, league).await?.0;
    let html = match league {
        RouteLeague::Nba => {
            let game = state.data.game(&game_id).await?;
            render::scoreboard_page_with_today(parsed_day, &scoreboard, game.as_ref(), today_day)
        }
        RouteLeague::Wnba => {
            let game = state.data.wnba_game(&game_id).await?;
            render::wnba_scoreboard_page_with_today(
                parsed_day,
                &scoreboard,
                game.as_ref(),
                today_day,
            )
        }
        RouteLeague::Mlb => {
            let game = state.data.mlb_game(&game_id).await?;
            render::mlb_scoreboard_page_with_today(
                parsed_day,
                &scoreboard,
                game.as_ref(),
                today_day,
            )
        }
        RouteLeague::Nhl => {
            let game = state.data.nhl_game(&game_id).await?;
            render::nhl_scoreboard_page_with_today(
                parsed_day,
                &scoreboard,
                game.as_ref(),
                today_day,
            )
        }
    };
    Ok(Html(html))
}

async fn days_games(
    state: &AppState,
    league: RouteLeague,
    day: &str,
) -> Result<Scoreboard, AppError> {
    match league {
        RouteLeague::Nba => state.data.days_games(day).await,
        RouteLeague::Wnba => state.data.wnba_days_games(day).await,
        RouteLeague::Mlb => state.data.mlb_days_games(day).await,
        RouteLeague::Nhl => state.data.nhl_days_games(day).await,
    }
}

async fn today_scoreboard(
    state: &AppState,
    league: RouteLeague,
) -> Result<(NaiveDate, Scoreboard), AppError> {
    match league {
        RouteLeague::Nba | RouteLeague::Wnba | RouteLeague::Mlb | RouteLeague::Nhl => {
            date_today_scoreboard(state, league).await
        }
    }
}
