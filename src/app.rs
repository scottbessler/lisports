use std::{env, net::SocketAddr, sync::Arc};

use axum::Router;
use tower_http::{services::ServeDir, trace::TraceLayer};

use crate::{
    cache::Cache,
    clients::{EspnSportsData, SportsData},
    error::AppError,
    routes,
};

#[derive(Clone)]
pub struct AppState {
    pub data: Arc<dyn SportsData>,
}

pub fn router(data: Arc<dyn SportsData>) -> Router {
    Router::new()
        .route("/", axum::routing::get(routes::index))
        .route("/healthcheck", axum::routing::get(routes::healthcheck))
        .route(
            "/nba/scoreboard",
            axum::routing::get(routes::nba_scoreboard),
        )
        .route(
            "/nba/scoreboard/today",
            axum::routing::get(routes::nba_scoreboard_today),
        )
        .route(
            "/nba/scoreboard/{day}",
            axum::routing::get(routes::nba_scoreboard_day),
        )
        .route(
            "/nba/scoreboard/{day}/game/{game_id}",
            axum::routing::get(routes::nba_game),
        )
        .route("/nba/standings", axum::routing::get(routes::nba_standings))
        .route(
            "/nba/player/{player_id}",
            axum::routing::get(routes::nba_player),
        )
        .route(
            "/wnba/scoreboard",
            axum::routing::get(routes::wnba_scoreboard),
        )
        .route(
            "/wnba/scoreboard/today",
            axum::routing::get(routes::wnba_scoreboard_today),
        )
        .route(
            "/wnba/scoreboard/{day}",
            axum::routing::get(routes::wnba_scoreboard_day),
        )
        .route(
            "/wnba/scoreboard/{day}/game/{game_id}",
            axum::routing::get(routes::wnba_game),
        )
        .route(
            "/wnba/standings",
            axum::routing::get(routes::wnba_standings),
        )
        .route(
            "/wnba/player/{player_id}",
            axum::routing::get(routes::wnba_player),
        )
        .route(
            "/mlb/scoreboard",
            axum::routing::get(routes::mlb_scoreboard),
        )
        .route(
            "/mlb/scoreboard/today",
            axum::routing::get(routes::mlb_scoreboard_today),
        )
        .route(
            "/mlb/scoreboard/{day}",
            axum::routing::get(routes::mlb_scoreboard_day),
        )
        .route(
            "/mlb/scoreboard/{day}/game/{game_id}",
            axum::routing::get(routes::mlb_game),
        )
        .route("/mlb/standings", axum::routing::get(routes::mlb_standings))
        .route(
            "/mlb/player/{player_id}",
            axum::routing::get(routes::mlb_player),
        )
        .route(
            "/nfl/scoreboard",
            axum::routing::get(routes::nfl_scoreboard),
        )
        .route(
            "/nfl/scoreboard/today",
            axum::routing::get(routes::nfl_scoreboard_today),
        )
        .route(
            "/nfl/scoreboard/{week}",
            axum::routing::get(routes::nfl_scoreboard_week),
        )
        .route(
            "/nfl/scoreboard/{week}/game/{game_id}",
            axum::routing::get(routes::nfl_game),
        )
        .route("/nfl/standings", axum::routing::get(routes::nfl_standings))
        .route(
            "/nfl/player/{player_id}",
            axum::routing::get(routes::nfl_player),
        )
        .route(
            "/nhl/scoreboard",
            axum::routing::get(routes::nhl_scoreboard),
        )
        .route(
            "/nhl/scoreboard/today",
            axum::routing::get(routes::nhl_scoreboard_today),
        )
        .route(
            "/nhl/scoreboard/{day}",
            axum::routing::get(routes::nhl_scoreboard_day),
        )
        .route(
            "/nhl/scoreboard/{day}/game/{game_id}",
            axum::routing::get(routes::nhl_game),
        )
        .route("/nhl/standings", axum::routing::get(routes::nhl_standings))
        .route(
            "/nhl/player/{player_id}",
            axum::routing::get(routes::nhl_player),
        )
        .nest_service("/public", ServeDir::new("public"))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState { data })
}

pub async fn run() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let data_path = env::var("DATA_PATH").unwrap_or_else(|_| "data".to_string());
    let cache = Cache::new(data_path);
    cache.ensure_dir().await?;
    let data = Arc::new(EspnSportsData::new(cache)?);
    let app = router(data);

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse().map_err(AppError::parse)?;
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(AppError::cache)?;
    let bound_addr = listener.local_addr().map_err(AppError::cache)?;
    println!("listening on port {}", bound_addr.port());
    tracing::info!("listening on http://{bound_addr}");
    axum::serve(listener, app).await.map_err(AppError::cache)?;
    Ok(())
}
