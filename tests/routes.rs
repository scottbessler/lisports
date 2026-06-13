use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
};
use lisports::{
    app,
    clients::SportsData,
    error::AppError,
    leagues::{self, PlayerFeature, ScheduleBucket},
    models::{
        BoxScore, BoxScoreTeam, Game, Leaders, MlbBoxScore, MlbBoxScoreTeam, MlbStandingsDivision,
        MlbStandingsTable, MlbStandingsTeam, NflBoxScore, NflBoxScoreTeam, NflStandingsDivision,
        NflStandingsTable, NflStandingsTeam, NhlBoxScore, NhlBoxScoreTeam, NhlStandingsDivision,
        NhlStandingsTable, NhlStandingsTeam, Period, Player, PlayerStatsPage, Scoreboard,
        SoccerBoxScore, SoccerBoxScoreTeam, SoccerStandingsGroup, SoccerStandingsTable,
        SoccerStandingsTeam, StandingsTable, StandingsTeam, Statistics, Table, Team,
        TeamStatistics,
    },
};
use tower::ServiceExt;

#[derive(Clone)]
struct FakeSportsData;

#[async_trait]
impl SportsData for FakeSportsData {
    async fn todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        Ok(nba_live_source_scoreboard())
    }

    async fn days_games(&self, day: &str) -> Result<Scoreboard, AppError> {
        if day == "2026-05-02" {
            Ok(live_scoreboard())
        } else {
            Ok(scoreboard())
        }
    }

    async fn game(&self, game_id: &str) -> Result<Option<BoxScore>, AppError> {
        if game_id == "401000000" {
            Ok(Some(live_box_score()))
        } else {
            Ok(Some(box_score()))
        }
    }

    async fn standings(&self) -> Result<StandingsTable, AppError> {
        Ok(StandingsTable {
            east: vec![standing_team(1, "Boston Celtics", "East")],
            west: vec![standing_team(2, "Los Angeles Lakers", "West")],
        })
    }

    async fn player_stats(&self, _player_id: &str) -> Result<PlayerStatsPage, AppError> {
        Ok(player_stats_page())
    }

    async fn wnba_todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        Ok(wnba_scoreboard())
    }

    async fn wnba_days_games(&self, _day: &str) -> Result<Scoreboard, AppError> {
        Ok(wnba_scoreboard())
    }

    async fn wnba_game(&self, _game_id: &str) -> Result<Option<BoxScore>, AppError> {
        Ok(Some(wnba_box_score()))
    }

    async fn wnba_standings(&self) -> Result<StandingsTable, AppError> {
        Ok(StandingsTable {
            east: vec![standing_team(1, "New York Liberty", "East")],
            west: vec![standing_team(2, "Las Vegas Aces", "West")],
        })
    }

    async fn wnba_player_stats(&self, _player_id: &str) -> Result<PlayerStatsPage, AppError> {
        Ok(player_stats_page())
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

    async fn mlb_player_stats(&self, _player_id: &str) -> Result<PlayerStatsPage, AppError> {
        Ok(player_stats_page())
    }

    async fn nfl_current_scoreboard(&self) -> Result<Scoreboard, AppError> {
        Ok(nfl_scoreboard(23))
    }

    async fn nfl_week_games(&self, week: i64) -> Result<Scoreboard, AppError> {
        Ok(nfl_scoreboard(week))
    }

    async fn nfl_game(&self, _game_id: &str) -> Result<Option<NflBoxScore>, AppError> {
        Ok(Some(nfl_box_score()))
    }

    async fn nfl_standings(&self) -> Result<NflStandingsTable, AppError> {
        Ok(NflStandingsTable {
            divisions: vec![
                NflStandingsDivision {
                    conference: "AFC".to_string(),
                    division: "East".to_string(),
                    teams: vec![nfl_standing_team(1, "Buffalo Bills", "BUF", "AFC", "East")],
                },
                NflStandingsDivision {
                    conference: "NFC".to_string(),
                    division: "East".to_string(),
                    teams: vec![nfl_standing_team(
                        1,
                        "Philadelphia Eagles",
                        "PHI",
                        "NFC",
                        "East",
                    )],
                },
            ],
        })
    }

    async fn nfl_player_stats(&self, _player_id: &str) -> Result<PlayerStatsPage, AppError> {
        Ok(player_stats_page())
    }

    async fn nhl_todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        Ok(nhl_scoreboard())
    }

    async fn nhl_days_games(&self, _day: &str) -> Result<Scoreboard, AppError> {
        Ok(nhl_scoreboard())
    }

    async fn nhl_game(&self, _game_id: &str) -> Result<Option<NhlBoxScore>, AppError> {
        Ok(Some(nhl_box_score()))
    }

    async fn nhl_standings(&self) -> Result<NhlStandingsTable, AppError> {
        Ok(NhlStandingsTable {
            divisions: vec![NhlStandingsDivision {
                conference: "East".to_string(),
                division: "Atlantic".to_string(),
                teams: vec![nhl_standing_team(
                    1,
                    "Boston Bruins",
                    "BOS",
                    "East",
                    "Atlantic",
                )],
            }],
        })
    }

    async fn nhl_player_stats(&self, _player_id: &str) -> Result<PlayerStatsPage, AppError> {
        Ok(player_stats_page())
    }

    async fn worldcup_todays_scoreboard(&self) -> Result<Scoreboard, AppError> {
        Ok(worldcup_scoreboard())
    }

    async fn worldcup_days_games(&self, _day: &str) -> Result<Scoreboard, AppError> {
        Ok(worldcup_scoreboard())
    }

    async fn worldcup_game(&self, _game_id: &str) -> Result<Option<SoccerBoxScore>, AppError> {
        Ok(Some(worldcup_box_score()))
    }

    async fn worldcup_standings(&self) -> Result<SoccerStandingsTable, AppError> {
        Ok(SoccerStandingsTable {
            groups: vec![SoccerStandingsGroup {
                group: "Group A".to_string(),
                teams: vec![soccer_standing_team(1, "Ecuador", "ECU")],
            }],
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
    assert!(body.contains(r#"<link rel="manifest" href="/public/manifest.webmanifest">"#));
    assert!(body.contains("class=\"game-card period-game-card\""));
    assert!(body.contains("<th>1</th><th>2</th><th>3</th><th>4</th><th>T</th>"));
}

#[tokio::test]
async fn today_scoreboard_urls_render_without_redirecting() {
    let (nba_status, nba_body) = request("/nba/scoreboard/today").await;
    let (mlb_status, mlb_body) = request("/mlb/scoreboard/today").await;
    let (nfl_status, nfl_body) = request("/nfl/scoreboard/today").await;
    let (wnba_status, wnba_body) = request("/wnba/scoreboard/today").await;
    let (nhl_status, nhl_body) = request("/nhl/scoreboard/today").await;
    let (worldcup_status, worldcup_body) = request("/worldcup/scoreboard/today").await;

    assert_eq!(nba_status, StatusCode::OK);
    assert!(nba_body.contains("NBA Scoreboard"));
    assert!(nba_body.contains(
        r#"<a class="button active date-current" href="/nba/scoreboard/2026-04-26">Sun 4/26 *</a>"#
    ));
    assert!(!date_nav(&nba_body).contains("/nba/scoreboard/today"));
    assert!(nba_body.contains("class=\"game-card period-game-card\""));
    assert!(nba_body.contains(r#"/nba/scoreboard/2026-04-26/game/401869385"#));
    assert!(!nba_body.contains("0042500224"));

    assert_eq!(mlb_status, StatusCode::OK);
    assert!(mlb_body.contains("MLB Scoreboard"));
    assert!(mlb_body.contains(
        r#"<a class="button active date-current" href="/mlb/scoreboard/2026-04-26">Sun 4/26 *</a>"#
    ));
    assert!(!date_nav(&mlb_body).contains("/mlb/scoreboard/today"));
    assert!(mlb_body.contains("class=\"game-card mlb-game-card\""));

    assert_eq!(nfl_status, StatusCode::OK);
    assert!(nfl_body.contains("NFL Scoreboard"));
    assert!(nfl_body.contains("Super Bowl"));
    assert!(
        nfl_body.contains(r#"<a class="button active" href="/nfl/scoreboard/23">Super Bowl</a>"#)
    );

    assert_eq!(wnba_status, StatusCode::OK);
    assert!(wnba_body.contains("WNBA Scoreboard"));
    assert!(wnba_body.contains(
        r#"<a class="button active date-current" href="/wnba/scoreboard/2026-04-26">Sun 4/26 *</a>"#
    ));
    assert!(!date_nav(&wnba_body).contains("/wnba/scoreboard/today"));
    assert!(wnba_body.contains("class=\"game-card period-game-card\""));
    assert!(wnba_body.contains(r#"/wnba/scoreboard/2026-04-26/game/401900100"#));

    assert_eq!(nhl_status, StatusCode::OK);
    assert!(nhl_body.contains("NHL Scoreboard"));
    assert!(nhl_body.contains(
        r#"<a class="button active date-current" href="/nhl/scoreboard/2026-04-26">Sun 4/26 *</a>"#
    ));
    assert!(!date_nav(&nhl_body).contains("/nhl/scoreboard/today"));
    assert!(nhl_body.contains("class=\"game-card period-game-card\""));
    assert!(nhl_body.contains(r#"/nhl/scoreboard/2026-04-26/game/401900001"#));

    assert_eq!(worldcup_status, StatusCode::OK);
    assert!(worldcup_body.contains("World Cup Scoreboard"));
    assert!(worldcup_body.contains(
        r#"<a class="button active date-current" href="/worldcup/scoreboard/2026-04-26">Sun 4/26 *</a>"#
    ));
    assert!(!date_nav(&worldcup_body).contains("/worldcup/scoreboard/today"));
    assert!(worldcup_body.contains("class=\"game-card soccer-game-card\""));
    assert!(worldcup_body.contains(r#"/worldcup/scoreboard/2026-04-26/game/633790"#));
}

#[tokio::test]
async fn dated_scoreboard_nav_marks_calendar_today() {
    let (status, body) = request("/nba/scoreboard/2026-04-27").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains(
        r#"<a class="button date-near" href="/nba/scoreboard/2026-04-26">Sun 4/26 *</a>"#
    ));
    assert!(!date_nav(&body).contains("/nba/scoreboard/today"));
}

#[tokio::test]
async fn live_scoreboard_pages_include_refresh_timestamp() {
    let (scoreboard_status, scoreboard_body) = request("/nba/scoreboard/2026-05-02").await;
    let (game_status, game_body) = request("/nba/scoreboard/2026-05-02/game/401000000").await;

    assert_eq!(scoreboard_status, StatusCode::OK);
    assert!(scoreboard_body.contains(r#"<body data-refresh-at=""#));
    assert!(scoreboard_body.contains("Q2 5:12"));

    assert_eq!(game_status, StatusCode::OK);
    assert!(game_body.contains(r#"<body data-refresh-at=""#));
    assert!(game_body.contains("scoreboard has-game"));
}

#[tokio::test]
async fn completed_scoreboard_pages_do_not_include_refresh_timestamp() {
    let (status, body) = request("/nba/scoreboard/2026-04-26").await;

    assert_eq!(status, StatusCode::OK);
    assert!(!body.contains("data-refresh-at"));
}

#[tokio::test]
async fn dayless_scoreboard_urls_redirect_to_today() {
    let (nba_status, nba_location) = request_redirect_location("/nba/scoreboard").await;
    let (mlb_status, mlb_location) = request_redirect_location("/mlb/scoreboard").await;
    let (nfl_status, nfl_location) = request_redirect_location("/nfl/scoreboard").await;
    let (nhl_status, nhl_location) = request_redirect_location("/nhl/scoreboard").await;
    let (wnba_status, wnba_location) = request_redirect_location("/wnba/scoreboard").await;
    let (worldcup_status, worldcup_location) =
        request_redirect_location("/worldcup/scoreboard").await;

    assert_eq!(nba_status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(nba_location, "/nba/scoreboard/today");
    assert_eq!(mlb_status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(mlb_location, "/mlb/scoreboard/today");
    assert_eq!(nfl_status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(nfl_location, "/nfl/scoreboard/today");
    assert_eq!(nhl_status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(nhl_location, "/nhl/scoreboard/today");
    assert_eq!(wnba_status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(wnba_location, "/wnba/scoreboard/today");
    assert_eq!(worldcup_status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(worldcup_location, "/worldcup/scoreboard/today");
}

#[tokio::test]
async fn root_redirects_to_default_league_scoreboard() {
    let (status, location) = request_redirect_location("/").await;
    assert_eq!(status, StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(location, "/nba/scoreboard");
}

#[tokio::test]
async fn league_registry_matches_route_surface() {
    for league in leagues::all() {
        assert!(league.scoreboard, "{} scoreboard undeclared", league.slug);
        assert!(league.game, "{} game undeclared", league.slug);
        assert!(league.standings, "{} standings undeclared", league.slug);

        let (scoreboard_status, scoreboard_location) =
            request_redirect_location(&format!("{}/scoreboard", league.route_base)).await;
        assert_eq!(scoreboard_status, StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(
            scoreboard_location,
            format!("{}/scoreboard/today", league.route_base)
        );

        let bucket = match league.bucket {
            ScheduleBucket::Date => "2026-04-26".to_string(),
            ScheduleBucket::Week => "1".to_string(),
        };
        let (scoreboard_status, scoreboard_body) =
            request(&format!("{}/scoreboard/{bucket}", league.route_base)).await;
        assert_eq!(scoreboard_status, StatusCode::OK);
        assert!(scoreboard_body.contains(&format!("{} Scoreboard", league.nav_label)));

        let (game_status, game_body) = request(&format!(
            "{}/scoreboard/{bucket}/game/{}",
            league.route_base,
            matrix_game_id(league.slug),
        ))
        .await;
        assert_eq!(game_status, StatusCode::OK);
        assert!(game_body.contains("scoreboard has-game"));

        let (standings_status, standings_body) =
            request(&format!("{}/standings", league.route_base)).await;
        assert_eq!(standings_status, StatusCode::OK);
        assert!(standings_body.contains(&format!("{} Standings", league.nav_label)));

        match league.player {
            PlayerFeature::Supported => {
                let (status, _) = request(&format!("{}/player/4278073", league.route_base)).await;
                assert_eq!(status, StatusCode::OK);
            }
            PlayerFeature::Unsupported => {
                let (status, _) = request(&format!("{}/player/4278073", league.route_base)).await;
                assert_eq!(status, StatusCode::NOT_FOUND);
            }
        }
    }
}

#[tokio::test]
async fn manifest_is_served_from_public_assets() {
    let (status, body) = request("/public/manifest.webmanifest").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains(r#""name": "LiSports""#));
    assert!(body.contains(r#""start_url": "/nba/scoreboard/today""#));
    assert!(body.contains(r#""display": "standalone""#));
}

#[tokio::test]
async fn game_view_renders_selected_box_score() {
    let (status, body) = request("/nba/scoreboard/2026-04-26/game/401869385").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("scoreboard has-game"));
    assert!(!body.contains("All games"));
    assert!(!body.contains(r#"<div class="game-list">"#));
    assert!(body.contains("table class=\"sortable box-score-table\""));
    assert!(body.contains("Jaylen Brown"));
    assert!(body.contains(
        r#"<td class="text"><strong>Team</strong></td><td class="num">240</td><td class="num">128</td>"#
    ));
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
async fn wnba_scoreboard_renders_nav_and_game_cards() {
    let (status, body) = request("/wnba/scoreboard/2026-04-26").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("WNBA Scoreboard"));
    assert!(body.contains("WNBA Standings"));
    assert!(body.contains("class=\"game-card period-game-card\""));
    assert!(body.contains("<th>1</th><th>2</th><th>3</th><th>4</th><th>T</th>"));
    assert!(body.contains("teamlogos/wnba/500/ny.png"));
}

#[tokio::test]
async fn wnba_game_view_renders_selected_box_score() {
    let (status, body) = request("/wnba/scoreboard/2026-04-26/game/401900100").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("scoreboard has-game"));
    assert!(!body.contains("All games"));
    assert!(body.contains("table class=\"sortable box-score-table\""));
    assert!(body.contains("Breanna Stewart"));
    assert!(body.contains(r#"<a href="/wnba/player/2984190">Breanna Stewart*</a>"#));
    assert!(!body.contains("/nba/player/2984190"));
    assert!(body.contains("teamlogos/wnba/500/ny.png"));
}

#[tokio::test]
async fn wnba_standings_render_sortable_tables() {
    let (status, body) = request("/wnba/standings").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("WNBA Standings"));
    assert!(body.contains("New York Liberty"));
    assert!(body.contains("Las Vegas Aces"));
    assert!(body.contains("table class=\"sortable\""));
}

#[tokio::test]
async fn wnba_player_route_renders_stats() {
    let (status, body) = request("/wnba/player/2984190").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("WNBA Player"));
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
    assert!(body.contains("class=\"game-card mlb-game-card\""));
    assert!(body.contains("<th>R</th><th>H</th><th>E</th>"));
    assert!(!body.contains("<th>1</th><th>2</th><th>3</th>"));
    assert!(body.contains(r#"<td class="score-total">-</td><td>-</td><td>-</td>"#));
    assert!(!body.contains("<td>237</td>"));
    assert!(!body.contains("<td>21</td>"));
}

#[tokio::test]
async fn mlb_game_view_renders_selected_box_score() {
    let (status, body) = request("/mlb/scoreboard/2026-04-26/game/401815095").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("scoreboard has-game"));
    assert!(!body.contains("All games"));
    assert!(body.contains(r#"<span class="step-status">7:10 PM PDT</span>"#));
    assert!(body.contains(r#"<span>MIA <small>(13-15)</small></span><strong>0</strong>"#));
    assert!(body.contains(r#"<span>LAD <small>(19-9)</small></span><strong>0</strong>"#));
    assert!(!body.contains(r#"<div class="game-list">"#));
    assert!(body.contains("Line Score"));
    assert!(body.contains("<th>1</th><th>2</th><th>3</th>"));
    assert!(body.contains(r#"<td>3</td><td>2</td><td>-</td>"#));
    assert!(body.contains("Batting"));
    assert!(body.contains("Pitching"));
    assert!(body.contains(r#"<a href="/mlb/player/646240">Rafael Devers</a>"#));
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
async fn mlb_player_route_renders_stats() {
    let (status, body) = request("/mlb/player/12345").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("MLB Player"));
    assert!(body.contains("Summary"));
    assert!(body.contains("Game Log"));
    assert!(body.contains("table class=\"sortable\""));
}

#[tokio::test]
async fn nfl_scoreboard_renders_nav_and_game_cards() {
    let (status, body) = request("/nfl/scoreboard/1").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("NFL Scoreboard"));
    assert!(body.contains("NFL Standings"));
    assert!(body.contains("Week 1"));
    assert!(body.contains("class=\"game-card period-game-card\""));
    assert!(body.contains("<th>T</th>"));
}

#[tokio::test]
async fn nfl_scoreboard_renders_playoff_week_window() {
    let (status, body) = request("/nfl/scoreboard/23").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("Conf Champ"));
    assert!(body.contains("Pro Bowl"));
    assert!(body.contains("Super Bowl"));
    assert!(!body.contains(r#"href="/nfl/scoreboard/1""#));
}

#[tokio::test]
async fn nfl_game_view_renders_selected_box_score() {
    let (status, body) = request("/nfl/scoreboard/1/game/401772845").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("scoreboard has-game"));
    assert!(body.contains("Team Stats"));
    assert!(body.contains(r#"<th class="num">PHI</th><th class="num">TB</th>"#));
    assert!(body.contains(r#"<td class="num">200</td><td class="num good">376</td>"#));
    assert!(body.contains(r#"<td class="num good">0</td><td class="num">2</td>"#));
    assert!(body.contains("Philadelphia Passing"));
    assert!(body.contains(r#"<a href="/nfl/player/4040715">Jalen Hurts</a>"#));
}

#[tokio::test]
async fn nfl_standings_render_sortable_tables() {
    let (status, body) = request("/nfl/standings").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("NFL Standings"));
    assert!(body.contains("AFC East"));
    assert!(body.contains("NFC East"));
    assert!(body.contains("Buffalo Bills"));
    assert!(body.contains("table class=\"sortable\""));
}

#[tokio::test]
async fn nfl_player_route_renders_stats() {
    let (status, body) = request("/nfl/player/12345").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("NFL Player"));
    assert!(body.contains("Summary"));
    assert!(body.contains("Game Log"));
    assert!(body.contains("table class=\"sortable\""));
}

#[tokio::test]
async fn nhl_scoreboard_renders_nav_and_game_cards() {
    let (status, body) = request("/nhl/scoreboard/2026-04-26").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("NHL Scoreboard"));
    assert!(body.contains("NHL Standings"));
    assert!(body.contains("class=\"game-card period-game-card\""));
    assert!(body.contains("<th>1</th><th>2</th><th>3</th><th>T</th>"));
    assert!(!body.contains("<th>4</th>"));
}

#[tokio::test]
async fn nhl_game_view_renders_selected_box_score() {
    let (status, body) = request("/nhl/scoreboard/2026-04-26/game/401900001").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("scoreboard has-game"));
    assert!(body.contains("Team Stats"));
    assert!(body.contains("BOS</th><th"));
    assert!(body.contains("NYR</th>"));
    assert!(body.contains(r#"<td class="num good">31</td><td class="num">29</td>"#));
    assert!(body.contains("Boston Skaters"));
    assert!(body.contains(r#"<a href="/nhl/player/3114778">David Pastrnak</a>"#));
}

#[tokio::test]
async fn nhl_standings_render_sortable_tables() {
    let (status, body) = request("/nhl/standings").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("NHL Standings"));
    assert!(body.contains("East Atlantic"));
    assert!(body.contains("Boston Bruins"));
    assert!(body.contains("OTL"));
    assert!(body.contains("table class=\"sortable\""));
}

#[tokio::test]
async fn nhl_player_route_renders_stats() {
    let (status, body) = request("/nhl/player/12345").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("NHL Player"));
    assert!(body.contains("Summary"));
    assert!(body.contains("Game Log"));
    assert!(body.contains("table class=\"sortable\""));
}

#[tokio::test]
async fn worldcup_scoreboard_renders_nav_and_game_cards() {
    let (status, body) = request("/worldcup/scoreboard/2026-04-26").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("World Cup Scoreboard"));
    assert!(body.contains("World Cup Standings"));
    assert!(body.contains("class=\"game-card soccer-game-card\""));
    assert!(body.contains("<th>Score</th>"));
    assert!(body.contains("teamlogos/countries/500/ecu.png"));
    assert!(body.contains("ECU"));
}

#[tokio::test]
async fn worldcup_game_view_renders_selected_match_stats() {
    let (status, body) = request("/worldcup/scoreboard/2026-04-26/game/633790").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("World Cup Match"));
    assert!(body.contains("scoreboard has-game"));
    assert!(body.contains("Team Stats"));
    assert!(body.contains("Possession"));
    assert!(body.contains("Ecuador"));
}

#[tokio::test]
async fn worldcup_standings_render_sortable_tables() {
    let (status, body) = request("/worldcup/standings").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("World Cup Standings"));
    assert!(body.contains("Group A"));
    assert!(body.contains("Ecuador"));
    assert!(body.contains("table class=\"sortable\""));
}

#[tokio::test]
async fn invalid_route_params_return_bad_request() {
    let (bad_day, _) = request("/nba/scoreboard/not-a-day").await;
    let (bad_game, _) = request("/nba/scoreboard/2026-04-26/game/abc").await;
    let (bad_player, _) = request("/nba/player/abc").await;
    let (bad_mlb_day, _) = request("/mlb/scoreboard/not-a-day").await;
    let (bad_mlb_game, _) = request("/mlb/scoreboard/2026-04-26/game/abc").await;
    let (bad_mlb_player, _) = request("/mlb/player/abc").await;
    let (bad_nfl_day, _) = request("/nfl/scoreboard/not-a-week").await;
    let (bad_nfl_week, _) = request("/nfl/scoreboard/24").await;
    let (bad_nfl_game, _) = request("/nfl/scoreboard/1/game/abc").await;
    let (bad_nfl_player, _) = request("/nfl/player/abc").await;
    let (bad_nhl_day, _) = request("/nhl/scoreboard/not-a-day").await;
    let (bad_nhl_game, _) = request("/nhl/scoreboard/2026-04-26/game/abc").await;
    let (bad_nhl_player, _) = request("/nhl/player/abc").await;
    let (bad_wnba_day, _) = request("/wnba/scoreboard/not-a-day").await;
    let (bad_wnba_game, _) = request("/wnba/scoreboard/2026-04-26/game/abc").await;
    let (bad_wnba_player, _) = request("/wnba/player/abc").await;
    let (bad_worldcup_day, _) = request("/worldcup/scoreboard/not-a-day").await;
    let (bad_worldcup_game, _) = request("/worldcup/scoreboard/2026-04-26/game/abc").await;
    assert_eq!(bad_day, StatusCode::BAD_REQUEST);
    assert_eq!(bad_game, StatusCode::BAD_REQUEST);
    assert_eq!(bad_player, StatusCode::BAD_REQUEST);
    assert_eq!(bad_mlb_day, StatusCode::BAD_REQUEST);
    assert_eq!(bad_mlb_game, StatusCode::BAD_REQUEST);
    assert_eq!(bad_mlb_player, StatusCode::BAD_REQUEST);
    assert_eq!(bad_nfl_day, StatusCode::BAD_REQUEST);
    assert_eq!(bad_nfl_week, StatusCode::BAD_REQUEST);
    assert_eq!(bad_nfl_game, StatusCode::BAD_REQUEST);
    assert_eq!(bad_nfl_player, StatusCode::BAD_REQUEST);
    assert_eq!(bad_nhl_day, StatusCode::BAD_REQUEST);
    assert_eq!(bad_nhl_game, StatusCode::BAD_REQUEST);
    assert_eq!(bad_nhl_player, StatusCode::BAD_REQUEST);
    assert_eq!(bad_wnba_day, StatusCode::BAD_REQUEST);
    assert_eq!(bad_wnba_game, StatusCode::BAD_REQUEST);
    assert_eq!(bad_wnba_player, StatusCode::BAD_REQUEST);
    assert_eq!(bad_worldcup_day, StatusCode::BAD_REQUEST);
    assert_eq!(bad_worldcup_game, StatusCode::BAD_REQUEST);
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

async fn request_redirect_location(uri: &str) -> (StatusCode, String) {
    let app = app::router(Arc::new(FakeSportsData));
    let response = app
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let location = response
        .headers()
        .get(header::LOCATION)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_string();
    (status, location)
}

fn date_nav(body: &str) -> &str {
    let start = body.find(r#"<div class="date-nav">"#).unwrap();
    let after_start = &body[start..];
    let end = after_start.find("</div>").unwrap() + "</div>".len();
    &after_start[..end]
}

fn player_stats_page() -> PlayerStatsPage {
    PlayerStatsPage {
        tables: vec![
            Table {
                name: "Summary".to_string(),
                headers: vec!["Split".to_string(), "PTS".to_string()],
                rows: vec![vec!["Season Total".to_string(), "20.1".to_string()]],
                first_column_links: Vec::new(),
            },
            Table {
                name: "Game Log".to_string(),
                headers: vec!["Date".to_string(), "PTS".to_string()],
                rows: vec![vec!["2026-04-26".to_string(), "31".to_string()]],
                first_column_links: Vec::new(),
            },
        ],
    }
}

fn matrix_game_id(slug: &str) -> &'static str {
    match slug {
        "nba" => "401869385",
        "wnba" => "401900100",
        "mlb" => "401815095",
        "nfl" => "401772845",
        "nhl" => "401900001",
        "worldcup" => "633790",
        _ => "1",
    }
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

fn nba_live_source_scoreboard() -> Scoreboard {
    let mut scoreboard = scoreboard();
    scoreboard.game_date = "2026-05-11".to_string();
    scoreboard.games[0].game_id = "0042500224".to_string();
    scoreboard
}

fn live_scoreboard() -> Scoreboard {
    let mut away_team = celtics_team();
    away_team.score = 55;
    away_team.periods = periods([29, 26, 0, 0]);
    let mut home_team = lakers_team();
    home_team.score = 51;
    home_team.periods = periods([24, 27, 0, 0]);
    Scoreboard {
        game_date: "2026-05-02".to_string(),
        games: vec![Game {
            game_id: "401000000".to_string(),
            game_status: 2,
            game_status_text: "Q2 5:12".to_string(),
            period: 2,
            game_clock: "5:12".to_string(),
            game_time_utc: "2026-05-02T23:30:00Z".to_string(),
            away_team,
            home_team,
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
                field_goals_made: 45,
                free_throws_attempted: 20,
                free_throws_made: 18,
                turnovers: 11,
                minutes: 240,
                points: 128,
                rebounds_total: 43,
                assists: 28,
                three_pointers_attempted: 35,
                three_pointers_made: 14,
                ..TeamStatistics::default()
            },
        },
        home_team: BoxScoreTeam {
            team: lakers_team(),
            players: vec![],
            statistics: TeamStatistics {
                field_goals_attempted: 91,
                field_goals_made: 37,
                free_throws_attempted: 18,
                free_throws_made: 14,
                turnovers: 14,
                minutes: 240,
                points: 96,
                rebounds_total: 39,
                assists: 22,
                three_pointers_attempted: 32,
                three_pointers_made: 8,
                ..TeamStatistics::default()
            },
        },
    }
}

fn live_box_score() -> BoxScore {
    let mut game = box_score();
    game.game_id = "401000000".to_string();
    game.game_status = 2;
    game
}

fn wnba_scoreboard() -> Scoreboard {
    Scoreboard {
        game_date: "2026-04-26".to_string(),
        games: vec![Game {
            game_id: "401900100".to_string(),
            game_status: 3,
            game_status_text: "Final".to_string(),
            period: 4,
            game_clock: String::new(),
            game_time_utc: "2026-04-26T19:00:00Z".to_string(),
            away_team: liberty_team(),
            home_team: aces_team(),
            away_leaders: Leaders::default(),
            home_leaders: Leaders::default(),
        }],
    }
}

fn wnba_box_score() -> BoxScore {
    BoxScore {
        game_id: "401900100".to_string(),
        game_status: 3,
        away_team: BoxScoreTeam {
            team: liberty_team(),
            players: vec![Player {
                person_id: 2984190,
                name: "Breanna Stewart".to_string(),
                starter: true,
                played: true,
                statistics: Statistics {
                    minutes: 34,
                    points: 28,
                    rebounds_total: 10,
                    assists: 4,
                    field_goals_made: 10,
                    field_goals_attempted: 18,
                    three_pointers_made: 2,
                    three_pointers_attempted: 5,
                    free_throws_made: 6,
                    free_throws_attempted: 7,
                    plus_minus_points: 8,
                    ..Statistics::default()
                },
            }],
            statistics: TeamStatistics {
                field_goals_attempted: 72,
                field_goals_made: 34,
                free_throws_attempted: 18,
                free_throws_made: 15,
                turnovers: 9,
                minutes: 200,
                points: 92,
                rebounds_total: 36,
                assists: 21,
                three_pointers_attempted: 25,
                three_pointers_made: 9,
                ..TeamStatistics::default()
            },
        },
        home_team: BoxScoreTeam {
            team: aces_team(),
            players: vec![],
            statistics: TeamStatistics {
                field_goals_attempted: 70,
                field_goals_made: 31,
                free_throws_attempted: 20,
                free_throws_made: 16,
                turnovers: 12,
                minutes: 200,
                points: 85,
                rebounds_total: 32,
                assists: 19,
                three_pointers_attempted: 22,
                three_pointers_made: 7,
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

fn liberty_team() -> Team {
    Team {
        team_id: 9,
        team_name: "Liberty".to_string(),
        team_city: "New York".to_string(),
        team_tricode: "NY".to_string(),
        wins: 32,
        losses: 8,
        display_record: "32-8".to_string(),
        score: 92,
        hits: 0,
        errors: 0,
        periods: periods([24, 22, 25, 21]),
    }
}

fn aces_team() -> Team {
    Team {
        team_id: 17,
        team_name: "Aces".to_string(),
        team_city: "Las Vegas".to_string(),
        team_tricode: "LV".to_string(),
        wins: 27,
        losses: 13,
        display_record: "27-13".to_string(),
        score: 85,
        hits: 0,
        errors: 0,
        periods: periods([21, 25, 18, 21]),
    }
}

fn mlb_scoreboard() -> Scoreboard {
    Scoreboard {
        game_date: "2026-04-26".to_string(),
        games: vec![
            Game {
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
            },
            Game {
                game_id: "401815099".to_string(),
                game_status: 1,
                game_status_text: "7:10 PM PDT".to_string(),
                period: 0,
                game_clock: String::new(),
                game_time_utc: "2026-04-27T02:10:00Z".to_string(),
                away_team: marlins_team(),
                home_team: dodgers_team(),
                away_leaders: Leaders::default(),
                home_leaders: Leaders::default(),
            },
        ],
    }
}

fn mlb_box_score() -> MlbBoxScore {
    let mut away_team = red_sox_team();
    away_team.periods = Vec::new();
    let mut home_team = orioles_team();
    home_team.periods = Vec::new();
    MlbBoxScore {
        game_id: "401815095".to_string(),
        game_status: 3,
        away_team: MlbBoxScoreTeam {
            team: away_team,
            batting: Table {
                name: "Batting".to_string(),
                headers: vec!["Name".to_string(), "AB".to_string(), "RBI".to_string()],
                rows: vec![vec![
                    "Rafael Devers".to_string(),
                    "4".to_string(),
                    "2".to_string(),
                ]],
                first_column_links: vec!["/mlb/player/646240".to_string()],
            },
            pitching: Table {
                name: "Pitching".to_string(),
                headers: vec!["Name".to_string(), "IP".to_string(), "K".to_string()],
                rows: vec![vec![
                    "Connelly Early".to_string(),
                    "6.2".to_string(),
                    "4".to_string(),
                ]],
                first_column_links: vec!["/mlb/player/4894460".to_string()],
            },
        },
        home_team: MlbBoxScoreTeam {
            team: home_team,
            batting: Table {
                name: "Batting".to_string(),
                headers: vec!["Name".to_string(), "AB".to_string(), "RBI".to_string()],
                rows: vec![vec![
                    "Gunnar Henderson".to_string(),
                    "4".to_string(),
                    "2".to_string(),
                ]],
                first_column_links: vec!["/mlb/player/4917402".to_string()],
            },
            pitching: Table {
                name: "Pitching".to_string(),
                headers: vec!["Name".to_string(), "IP".to_string(), "K".to_string()],
                rows: vec![vec![
                    "Kyle Bradish".to_string(),
                    "5".to_string(),
                    "3".to_string(),
                ]],
                first_column_links: vec!["/mlb/player/4241443".to_string()],
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

fn marlins_team() -> Team {
    Team {
        team_id: 28,
        team_name: "Marlins".to_string(),
        team_city: "Miami".to_string(),
        team_tricode: "MIA".to_string(),
        wins: 13,
        losses: 15,
        display_record: "13-15".to_string(),
        score: 0,
        hits: 237,
        errors: 21,
        periods: Vec::new(),
    }
}

fn dodgers_team() -> Team {
    Team {
        team_id: 19,
        team_name: "Dodgers".to_string(),
        team_city: "Los Angeles".to_string(),
        team_tricode: "LAD".to_string(),
        wins: 19,
        losses: 9,
        display_record: "19-9".to_string(),
        score: 0,
        hits: 264,
        errors: 8,
        periods: Vec::new(),
    }
}

fn nfl_scoreboard(week: i64) -> Scoreboard {
    Scoreboard {
        game_date: week.to_string(),
        games: vec![Game {
            game_id: "401772845".to_string(),
            game_status: 3,
            game_status_text: "Final".to_string(),
            period: 4,
            game_clock: String::new(),
            game_time_utc: "2026-01-04T18:00:00Z".to_string(),
            away_team: eagles_team(),
            home_team: buccaneers_team(),
            away_leaders: Leaders::default(),
            home_leaders: Leaders::default(),
        }],
    }
}

fn nfl_box_score() -> NflBoxScore {
    NflBoxScore {
        game_id: "401772845".to_string(),
        game_status: 3,
        away_team: NflBoxScoreTeam {
            team: eagles_team(),
            team_stats: Table {
                name: "Team Stats".to_string(),
                headers: vec!["Stat".to_string(), "Value".to_string()],
                rows: vec![
                    vec!["Total Yards".to_string(), "200".to_string()],
                    vec!["Turnovers".to_string(), "0".to_string()],
                ],
                first_column_links: Vec::new(),
            },
            player_stats: vec![Table {
                name: "Philadelphia Passing".to_string(),
                headers: vec!["Name".to_string(), "C/ATT".to_string(), "YDS".to_string()],
                rows: vec![vec![
                    "Jalen Hurts".to_string(),
                    "15/24".to_string(),
                    "130".to_string(),
                ]],
                first_column_links: vec!["/nfl/player/4040715".to_string()],
            }],
        },
        home_team: NflBoxScoreTeam {
            team: buccaneers_team(),
            team_stats: Table {
                name: "Team Stats".to_string(),
                headers: vec!["Stat".to_string(), "Value".to_string()],
                rows: vec![
                    vec!["Total Yards".to_string(), "376".to_string()],
                    vec!["Turnovers".to_string(), "2".to_string()],
                ],
                first_column_links: Vec::new(),
            },
            player_stats: vec![Table {
                name: "Tampa Bay Passing".to_string(),
                headers: vec!["Name".to_string(), "C/ATT".to_string(), "YDS".to_string()],
                rows: vec![vec![
                    "Baker Mayfield".to_string(),
                    "22/40".to_string(),
                    "272".to_string(),
                ]],
                first_column_links: vec!["/nfl/player/3052587".to_string()],
            }],
        },
    }
}

fn nhl_scoreboard() -> Scoreboard {
    Scoreboard {
        game_date: "2026-04-26".to_string(),
        games: vec![Game {
            game_id: "401900001".to_string(),
            game_status: 3,
            game_status_text: "Final".to_string(),
            period: 3,
            game_clock: String::new(),
            game_time_utc: "2026-04-26T23:00:00Z".to_string(),
            away_team: bruins_team(),
            home_team: rangers_team(),
            away_leaders: Leaders::default(),
            home_leaders: Leaders::default(),
        }],
    }
}

fn nhl_box_score() -> NhlBoxScore {
    NhlBoxScore {
        game_id: "401900001".to_string(),
        game_status: 3,
        away_team: NhlBoxScoreTeam {
            team: bruins_team(),
            team_stats: Table {
                name: "Team Stats".to_string(),
                headers: vec!["Stat".to_string(), "Value".to_string()],
                rows: vec![
                    vec!["Shots".to_string(), "31".to_string()],
                    vec!["Hits".to_string(), "24".to_string()],
                ],
                first_column_links: Vec::new(),
            },
            player_stats: vec![Table {
                name: "Boston Skaters".to_string(),
                headers: vec!["Name".to_string(), "G".to_string(), "A".to_string()],
                rows: vec![vec![
                    "David Pastrnak".to_string(),
                    "1".to_string(),
                    "1".to_string(),
                ]],
                first_column_links: vec!["/nhl/player/3114778".to_string()],
            }],
        },
        home_team: NhlBoxScoreTeam {
            team: rangers_team(),
            team_stats: Table {
                name: "Team Stats".to_string(),
                headers: vec!["Stat".to_string(), "Value".to_string()],
                rows: vec![
                    vec!["Shots".to_string(), "29".to_string()],
                    vec!["Hits".to_string(), "20".to_string()],
                ],
                first_column_links: Vec::new(),
            },
            player_stats: vec![Table {
                name: "New York Skaters".to_string(),
                headers: vec!["Name".to_string(), "G".to_string(), "A".to_string()],
                rows: vec![vec![
                    "Artemi Panarin".to_string(),
                    "0".to_string(),
                    "1".to_string(),
                ]],
                first_column_links: vec!["/nhl/player/3114777".to_string()],
            }],
        },
    }
}

fn worldcup_scoreboard() -> Scoreboard {
    Scoreboard {
        game_date: "2026-04-26".to_string(),
        games: vec![Game {
            game_id: "633790".to_string(),
            game_status: 3,
            game_status_text: "FT".to_string(),
            period: 2,
            game_clock: String::new(),
            game_time_utc: "2026-04-26T16:00:00Z".to_string(),
            away_team: ecuador_team(),
            home_team: qatar_team(),
            away_leaders: Leaders::default(),
            home_leaders: Leaders::default(),
        }],
    }
}

fn worldcup_box_score() -> SoccerBoxScore {
    SoccerBoxScore {
        game_id: "633790".to_string(),
        game_status: 3,
        away_team: SoccerBoxScoreTeam {
            team: ecuador_team(),
            team_stats: Table {
                name: "Team Stats".to_string(),
                headers: vec!["Stat".to_string(), "Value".to_string()],
                rows: vec![
                    vec!["Possession".to_string(), "52.9".to_string()],
                    vec!["SHOTS".to_string(), "6".to_string()],
                ],
                first_column_links: Vec::new(),
            },
        },
        home_team: SoccerBoxScoreTeam {
            team: qatar_team(),
            team_stats: Table {
                name: "Team Stats".to_string(),
                headers: vec!["Stat".to_string(), "Value".to_string()],
                rows: vec![
                    vec!["Possession".to_string(), "47.1".to_string()],
                    vec!["SHOTS".to_string(), "5".to_string()],
                ],
                first_column_links: Vec::new(),
            },
        },
    }
}

fn eagles_team() -> Team {
    Team {
        team_id: 21,
        team_name: "Eagles".to_string(),
        team_city: "Philadelphia".to_string(),
        team_tricode: "PHI".to_string(),
        wins: 13,
        losses: 4,
        display_record: "13-4".to_string(),
        score: 31,
        hits: 0,
        errors: 0,
        periods: periods([14, 10, 7, 0]),
    }
}

fn buccaneers_team() -> Team {
    Team {
        team_id: 27,
        team_name: "Buccaneers".to_string(),
        team_city: "Tampa Bay".to_string(),
        team_tricode: "TB".to_string(),
        wins: 10,
        losses: 7,
        display_record: "10-7".to_string(),
        score: 25,
        hits: 0,
        errors: 0,
        periods: periods([3, 3, 14, 5]),
    }
}

fn bruins_team() -> Team {
    Team {
        team_id: 1,
        team_name: "Bruins".to_string(),
        team_city: "Boston".to_string(),
        team_tricode: "BOS".to_string(),
        wins: 45,
        losses: 27,
        display_record: "45-27-10".to_string(),
        score: 3,
        hits: 0,
        errors: 0,
        periods: periods([1, 1, 1]),
    }
}

fn rangers_team() -> Team {
    Team {
        team_id: 13,
        team_name: "Rangers".to_string(),
        team_city: "New York".to_string(),
        team_tricode: "NYR".to_string(),
        wins: 47,
        losses: 25,
        display_record: "47-25-10".to_string(),
        score: 2,
        hits: 0,
        errors: 0,
        periods: periods([0, 1, 1]),
    }
}

fn ecuador_team() -> Team {
    Team {
        team_id: 209,
        team_name: "Ecuador".to_string(),
        team_city: "Ecuador".to_string(),
        team_tricode: "ECU".to_string(),
        wins: 1,
        losses: 0,
        display_record: "1-0-0".to_string(),
        score: 2,
        hits: 0,
        errors: 0,
        periods: Vec::new(),
    }
}

fn qatar_team() -> Team {
    Team {
        team_id: 4398,
        team_name: "Qatar".to_string(),
        team_city: "Qatar".to_string(),
        team_tricode: "QAT".to_string(),
        wins: 0,
        losses: 1,
        display_record: "0-1-0".to_string(),
        score: 0,
        hits: 0,
        errors: 0,
        periods: Vec::new(),
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
        team_tricode: if name.contains("Liberty") {
            "NY".to_string()
        } else if name.contains("Aces") {
            "LV".to_string()
        } else {
            name.split_whitespace()
                .map(|part| part.chars().next().unwrap_or_default())
                .collect()
        },
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

fn soccer_standing_team(rank: i64, name: &str, tricode: &str) -> SoccerStandingsTeam {
    SoccerStandingsTeam {
        team_id: rank,
        team_name: name.to_string(),
        team_tricode: tricode.to_string(),
        rank,
        games_played: 1,
        wins: 1,
        draws: 0,
        losses: 0,
        goals_for: 2,
        goals_against: 0,
        goal_diff: "+2".to_string(),
        points: 3,
        record: "1-0-0".to_string(),
    }
}

fn nfl_standing_team(
    rank: i64,
    name: &str,
    tricode: &str,
    conference: &str,
    division: &str,
) -> NflStandingsTeam {
    NflStandingsTeam {
        team_id: rank,
        team_name: name.to_string(),
        team_tricode: tricode.to_string(),
        conference: conference.to_string(),
        division: division.to_string(),
        playoff_rank: rank,
        wins: 13,
        losses: 4,
        ties: 0,
        win_pct: ".765".to_string(),
        games_back: "-".to_string(),
        points_for: 474,
        points_against: 336,
        point_diff: "+138".to_string(),
        home: "7-2".to_string(),
        road: "6-2".to_string(),
        division_record: "5-1".to_string(),
        conference_record: "10-2".to_string(),
        streak: "W8".to_string(),
    }
}

fn nhl_standing_team(
    rank: i64,
    name: &str,
    tricode: &str,
    conference: &str,
    division: &str,
) -> NhlStandingsTeam {
    NhlStandingsTeam {
        team_id: rank,
        team_name: name.to_string(),
        team_tricode: tricode.to_string(),
        conference: conference.to_string(),
        division: division.to_string(),
        playoff_rank: rank,
        wins: 45,
        losses: 27,
        ot_losses: 10,
        points: 100,
        games_back: "-".to_string(),
        goals_for: 263,
        goals_against: 241,
        goal_diff: "+22".to_string(),
        home: "24-12-5".to_string(),
        road: "21-15-5".to_string(),
        division_record: "14-8-4".to_string(),
        last_ten: "6-3-1".to_string(),
        streak: "W2".to_string(),
    }
}
