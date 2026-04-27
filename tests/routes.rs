use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use lisports::{
    app,
    clients::SportsData,
    error::AppError,
    models::{
        BoxScore, BoxScoreTeam, Game, Leaders, MlbBoxScore, MlbBoxScoreTeam, MlbStandingsDivision,
        MlbStandingsTable, MlbStandingsTeam, Period, Player, PlayerStatsPage, Scoreboard,
        StandingsTable, StandingsTeam, Statistics, Table, Team, TeamStatistics,
    },
};
use tower::ServiceExt;

#[derive(Clone)]
struct FakeSportsData;

#[async_trait]
impl SportsData for FakeSportsData {
    async fn todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        Ok(scoreboard())
    }

    async fn days_games(&self, _day: &str) -> Result<Scoreboard, AppError> {
        Ok(scoreboard())
    }

    async fn game(&self, _game_id: &str) -> Result<Option<BoxScore>, AppError> {
        Ok(Some(box_score()))
    }

    async fn standings(&self) -> Result<StandingsTable, AppError> {
        Ok(StandingsTable {
            east: vec![standing_team(1, "Boston Celtics", "East")],
            west: vec![standing_team(2, "Los Angeles Lakers", "West")],
        })
    }

    async fn player_stats(&self, _player_id: &str) -> Result<PlayerStatsPage, AppError> {
        Ok(PlayerStatsPage {
            tables: vec![
                Table {
                    name: "Summary".to_string(),
                    headers: vec!["Split".to_string(), "PTS".to_string()],
                    rows: vec![vec!["Season Total".to_string(), "20.1".to_string()]],
                },
                Table {
                    name: "Game Log".to_string(),
                    headers: vec!["Date".to_string(), "PTS".to_string()],
                    rows: vec![vec!["2026-04-26".to_string(), "31".to_string()]],
                },
            ],
        })
    }

    async fn mlb_todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        Ok(mlb_scoreboard())
    }

    async fn mlb_days_games(&self, _day: &str) -> Result<Scoreboard, AppError> {
        Ok(mlb_scoreboard())
    }

    async fn mlb_game(&self, _game_id: &str) -> Result<Option<MlbBoxScore>, AppError> {
        Ok(Some(mlb_box_score()))
    }

    async fn mlb_standings(&self) -> Result<MlbStandingsTable, AppError> {
        Ok(MlbStandingsTable {
            divisions: vec![
                MlbStandingsDivision {
                    league: "AL".to_string(),
                    division: "East".to_string(),
                    teams: vec![mlb_standing_team(
                        1,
                        "New York Yankees",
                        "NYY",
                        "AL",
                        "East",
                    )],
                },
                MlbStandingsDivision {
                    league: "NL".to_string(),
                    division: "East".to_string(),
                    teams: vec![mlb_standing_team(1, "Atlanta Braves", "ATL", "NL", "East")],
                },
            ],
        })
    }
}

#[tokio::test]
async fn healthcheck_returns_ok() {
    let response = request("/healthcheck").await;
    assert_eq!(response.0, StatusCode::OK);
    assert_eq!(response.1, "OK");
}

#[tokio::test]
async fn scoreboard_renders_nav_and_game_cards() {
    let (status, body) = request("/nba/scoreboard/2026-04-26").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("NBA Scoreboard"));
    assert!(body.contains("class=\"nav\""));
    assert!(body.contains("class=\"game-card\""));
}

#[tokio::test]
async fn game_view_renders_selected_box_score() {
    let (status, body) = request("/nba/scoreboard/2026-04-26/game/401869385").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("scoreboard has-game"));
    assert!(body.contains("table class=\"sortable box-score-table\""));
    assert!(body.contains("Jaylen Brown"));
}

#[tokio::test]
async fn standings_render_sortable_tables() {
    let (status, body) = request("/nba/standings").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("NBA Standings"));
    assert!(body.contains("Boston Celtics"));
    assert!(body.contains("table class=\"sortable\""));
}

#[tokio::test]
async fn player_page_renders_summary_and_game_log() {
    let (status, body) = request("/nba/player/4278073").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Summary"));
    assert!(body.contains("Game Log"));
    assert!(body.contains("table class=\"sortable\""));
}

#[tokio::test]
async fn mlb_scoreboard_renders_nav_and_game_cards() {
    let (status, body) = request("/mlb/scoreboard/2026-04-26").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("MLB Scoreboard"));
    assert!(body.contains("MLB Standings"));
    assert!(body.contains("class=\"game-card\""));
    assert!(body.contains("<th>R</th><th>H</th><th>E</th>"));
}

#[tokio::test]
async fn mlb_game_view_renders_selected_box_score() {
    let (status, body) = request("/mlb/scoreboard/2026-04-26/game/401815095").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("scoreboard has-game"));
    assert!(body.contains("Batting"));
    assert!(body.contains("Pitching"));
    assert!(body.contains("Rafael Devers"));
}

#[tokio::test]
async fn mlb_standings_render_sortable_tables() {
    let (status, body) = request("/mlb/standings").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("MLB Standings"));
    assert!(body.contains("AL East"));
    assert!(body.contains("NL East"));
    assert!(body.contains("New York Yankees"));
    assert!(body.contains("table class=\"sortable\""));
}

#[tokio::test]
async fn invalid_route_params_return_bad_request() {
    let (bad_day, _) = request("/nba/scoreboard/not-a-day").await;
    let (bad_game, _) = request("/nba/scoreboard/2026-04-26/game/abc").await;
    let (bad_player, _) = request("/nba/player/abc").await;
    let (bad_mlb_day, _) = request("/mlb/scoreboard/not-a-day").await;
    let (bad_mlb_game, _) = request("/mlb/scoreboard/2026-04-26/game/abc").await;
    assert_eq!(bad_day, StatusCode::BAD_REQUEST);
    assert_eq!(bad_game, StatusCode::BAD_REQUEST);
    assert_eq!(bad_player, StatusCode::BAD_REQUEST);
    assert_eq!(bad_mlb_day, StatusCode::BAD_REQUEST);
    assert_eq!(bad_mlb_game, StatusCode::BAD_REQUEST);
}

async fn request(uri: &str) -> (StatusCode, String) {
    let app = app::router(Arc::new(FakeSportsData));
    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, String::from_utf8(bytes.to_vec()).unwrap())
}

fn scoreboard() -> Scoreboard {
    Scoreboard {
        game_date: "2026-04-26".to_string(),
        games: vec![Game {
            game_id: "401869385".to_string(),
            game_status: 3,
            game_status_text: "Final".to_string(),
            period: 4,
            game_clock: String::new(),
            game_time_utc: "2026-04-26T19:00:00Z".to_string(),
            away_team: celtics_team(),
            home_team: lakers_team(),
            away_leaders: Leaders::default(),
            home_leaders: Leaders::default(),
        }],
    }
}

fn box_score() -> BoxScore {
    BoxScore {
        game_id: "401869385".to_string(),
        game_status: 3,
        away_team: BoxScoreTeam {
            team: celtics_team(),
            players: vec![Player {
                person_id: 1627759,
                name: "Jaylen Brown".to_string(),
                starter: true,
                played: true,
                statistics: Statistics {
                    minutes: 35,
                    points: 31,
                    rebounds_total: 8,
                    assists: 6,
                    field_goals_made: 11,
                    field_goals_attempted: 19,
                    three_pointers_made: 3,
                    three_pointers_attempted: 7,
                    free_throws_made: 6,
                    free_throws_attempted: 6,
                    plus_minus_points: 12,
                    ..Statistics::default()
                },
            }],
            statistics: TeamStatistics {
                field_goals_attempted: 88,
                free_throws_attempted: 20,
                turnovers: 11,
                minutes: 240,
                points: 128,
                ..TeamStatistics::default()
            },
        },
        home_team: BoxScoreTeam {
            team: lakers_team(),
            players: vec![],
            statistics: TeamStatistics {
                field_goals_attempted: 91,
                free_throws_attempted: 18,
                turnovers: 14,
                minutes: 240,
                points: 96,
                ..TeamStatistics::default()
            },
        },
    }
}

fn celtics_team() -> Team {
    Team {
        team_id: 1610612738,
        team_name: "Celtics".to_string(),
        team_city: "Boston".to_string(),
        team_tricode: "BOS".to_string(),
        wins: 56,
        losses: 26,
        display_record: "56-26".to_string(),
        score: 128,
        hits: 0,
        errors: 0,
        periods: periods([34, 22, 39, 33]),
    }
}

fn lakers_team() -> Team {
    Team {
        team_id: 1610612747,
        team_name: "Lakers".to_string(),
        team_city: "Los Angeles".to_string(),
        team_tricode: "LAL".to_string(),
        wins: 53,
        losses: 29,
        display_record: "53-29".to_string(),
        score: 96,
        hits: 0,
        errors: 0,
        periods: periods([21, 26, 18, 31]),
    }
}

fn mlb_scoreboard() -> Scoreboard {
    Scoreboard {
        game_date: "2026-04-26".to_string(),
        games: vec![Game {
            game_id: "401815095".to_string(),
            game_status: 3,
            game_status_text: "Final".to_string(),
            period: 9,
            game_clock: String::new(),
            game_time_utc: "2026-04-26T17:35:00Z".to_string(),
            away_team: red_sox_team(),
            home_team: orioles_team(),
            away_leaders: Leaders::default(),
            home_leaders: Leaders::default(),
        }],
    }
}

fn mlb_box_score() -> MlbBoxScore {
    MlbBoxScore {
        game_id: "401815095".to_string(),
        game_status: 3,
        away_team: MlbBoxScoreTeam {
            team: red_sox_team(),
            batting: Table {
                name: "Batting".to_string(),
                headers: vec!["Name".to_string(), "AB".to_string(), "RBI".to_string()],
                rows: vec![vec![
                    "Rafael Devers".to_string(),
                    "4".to_string(),
                    "2".to_string(),
                ]],
            },
            pitching: Table {
                name: "Pitching".to_string(),
                headers: vec!["Name".to_string(), "IP".to_string(), "K".to_string()],
                rows: vec![vec![
                    "Connelly Early".to_string(),
                    "6.2".to_string(),
                    "4".to_string(),
                ]],
            },
        },
        home_team: MlbBoxScoreTeam {
            team: orioles_team(),
            batting: Table {
                name: "Batting".to_string(),
                headers: vec!["Name".to_string(), "AB".to_string(), "RBI".to_string()],
                rows: vec![vec![
                    "Gunnar Henderson".to_string(),
                    "4".to_string(),
                    "2".to_string(),
                ]],
            },
            pitching: Table {
                name: "Pitching".to_string(),
                headers: vec!["Name".to_string(), "IP".to_string(), "K".to_string()],
                rows: vec![vec![
                    "Kyle Bradish".to_string(),
                    "5".to_string(),
                    "3".to_string(),
                ]],
            },
        },
    }
}

fn red_sox_team() -> Team {
    Team {
        team_id: 2,
        team_name: "Red Sox".to_string(),
        team_city: "Boston".to_string(),
        team_tricode: "BOS".to_string(),
        wins: 11,
        losses: 17,
        display_record: "11-17".to_string(),
        score: 5,
        hits: 7,
        errors: 0,
        periods: periods([0, 0, 0, 0, 3, 2, 0, 0, 0]),
    }
}

fn orioles_team() -> Team {
    Team {
        team_id: 1,
        team_name: "Orioles".to_string(),
        team_city: "Baltimore".to_string(),
        team_tricode: "BAL".to_string(),
        wins: 13,
        losses: 15,
        display_record: "13-15".to_string(),
        score: 3,
        hits: 6,
        errors: 1,
        periods: periods([0, 0, 0, 0, 1, 1, 0, 1, 0]),
    }
}

fn periods<const N: usize>(scores: [i64; N]) -> Vec<Period> {
    scores
        .into_iter()
        .enumerate()
        .map(|(idx, score)| Period {
            period: idx as i64 + 1,
            score,
        })
        .collect()
}

fn standing_team(rank: i64, name: &str, conference: &str) -> StandingsTeam {
    StandingsTeam {
        team_id: 1610612738 + rank,
        team_name: name.to_string(),
        conference: conference.to_string(),
        playoff_rank: rank,
        wins: 50,
        losses: 32,
        win_pct: 0.61,
        conference_games_back: 0.0,
        points_pg: 113.0,
        opp_points_pg: 109.1,
        diff_points_pg: 3.9,
        home: "28-13".to_string(),
        road: "22-19".to_string(),
        last_ten: "7-3".to_string(),
        current_streak: 2,
    }
}

fn mlb_standing_team(
    rank: i64,
    name: &str,
    tricode: &str,
    league: &str,
    division: &str,
) -> MlbStandingsTeam {
    MlbStandingsTeam {
        team_id: rank,
        team_name: name.to_string(),
        team_tricode: tricode.to_string(),
        league: league.to_string(),
        division: division.to_string(),
        playoff_rank: rank,
        wins: 18,
        losses: 10,
        win_pct: ".643".to_string(),
        games_back: "-".to_string(),
        runs_scored: 146,
        runs_allowed: 99,
        run_diff: "+47".to_string(),
        streak: "L1".to_string(),
    }
}
