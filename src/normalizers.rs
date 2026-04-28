use serde_json::Value;

use crate::{
    clients::{
        EspnPlayerGamelogDto, EspnScoreboardDto, EspnStandingsDto, EspnSummaryDto,
        NbaTodaysScoreboardDto,
    },
    error::AppError,
    models::{
        BoxScore, BoxScoreTeam, Game, Leaders, MlbBoxScore, MlbBoxScoreTeam, MlbStandingsDivision,
        MlbStandingsTable, MlbStandingsTeam, NflBoxScore, NflBoxScoreTeam, NflStandingsDivision,
        NflStandingsTable, NflStandingsTeam, Period, Player, PlayerStatsPage, Scoreboard,
        StandingsTable, StandingsTeam, Statistics, Table, Team, TeamStatistics,
    },
};

pub fn nba_today_scoreboard(data: NbaTodaysScoreboardDto) -> Result<Scoreboard, AppError> {
    let day = str_at(&data.scoreboard, &["gameDate"]).unwrap_or_default();
    let games = array_at(&data.scoreboard, &["games"])
        .iter()
        .map(nba_game)
        .collect();
    Ok(Scoreboard {
        game_date: day,
        games,
    })
}

pub fn espn_scoreboard(day: &str, data: EspnScoreboardDto) -> Result<Scoreboard, AppError> {
    espn_scoreboard_with(day, data, espn_competitor_to_team)
}

pub fn espn_mlb_scoreboard(day: &str, data: EspnScoreboardDto) -> Result<Scoreboard, AppError> {
    espn_scoreboard_with(day, data, espn_mlb_competitor_to_team)
}

pub fn espn_nfl_scoreboard(day: &str, data: EspnScoreboardDto) -> Result<Scoreboard, AppError> {
    espn_scoreboard_with(day, data, espn_nfl_competitor_to_team)
}

fn espn_scoreboard_with(
    day: &str,
    data: EspnScoreboardDto,
    team_fn: fn(&Value, &Value) -> Team,
) -> Result<Scoreboard, AppError> {
    let mut games = Vec::new();
    for event in data.events {
        let comp = event
            .pointer("/competitions/0")
            .ok_or_else(|| AppError::parse("missing competition"))?;
        let status = comp.get("status").unwrap_or(&Value::Null);
        let competitors = array_at(comp, &["competitors"]);
        let home = competitors
            .iter()
            .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("home"))
            .ok_or_else(|| AppError::parse("missing home team"))?;
        let away = competitors
            .iter()
            .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("away"))
            .ok_or_else(|| AppError::parse("missing away team"))?;
        let home_team = team_fn(home, comp);
        let away_team = team_fn(away, comp);
        games.push(Game {
            game_id: str_at(&event, &["id"]).unwrap_or_default(),
            game_status: espn_status_to_game_status(status),
            game_status_text: str_at(status, &["type", "shortDetail"])
                .or_else(|| str_at(status, &["type", "description"]))
                .unwrap_or_default(),
            period: i64_at(status, &["period"]),
            game_clock: str_at(status, &["displayClock"]).unwrap_or_default(),
            game_time_utc: str_at(&event, &["date"]).unwrap_or_default(),
            home_leaders: extract_leaders(home),
            away_leaders: extract_leaders(away),
            home_team,
            away_team,
        });
    }
    Ok(Scoreboard {
        game_date: day.to_string(),
        games,
    })
}

pub fn espn_summary(data: EspnSummaryDto) -> Result<BoxScore, AppError> {
    let header_comp = data
        .header
        .pointer("/competitions/0")
        .ok_or_else(|| AppError::parse("missing header competition"))?;
    let status = header_comp.get("status").unwrap_or(&Value::Null);
    let competitors = array_at(header_comp, &["competitors"]);
    let home_comp = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("home"))
        .ok_or_else(|| AppError::parse("missing home"))?;
    let away_comp = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("away"))
        .ok_or_else(|| AppError::parse("missing away"))?;
    let home_abbr = str_at(home_comp, &["team", "abbreviation"]).unwrap_or_default();
    let away_abbr = str_at(away_comp, &["team", "abbreviation"]).unwrap_or_default();
    Ok(BoxScore {
        game_id: str_at(&data.header, &["id"]).unwrap_or_default(),
        game_status: espn_status_to_game_status(status),
        home_team: summary_team(&data.boxscore, &home_abbr, home_comp),
        away_team: summary_team(&data.boxscore, &away_abbr, away_comp),
    })
}

pub fn espn_mlb_summary(data: EspnSummaryDto) -> Result<MlbBoxScore, AppError> {
    let header_comp = data
        .header
        .pointer("/competitions/0")
        .ok_or_else(|| AppError::parse("missing header competition"))?;
    let status = header_comp.get("status").unwrap_or(&Value::Null);
    let competitors = array_at(header_comp, &["competitors"]);
    let home_comp = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("home"))
        .ok_or_else(|| AppError::parse("missing home"))?;
    let away_comp = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("away"))
        .ok_or_else(|| AppError::parse("missing away"))?;
    Ok(MlbBoxScore {
        game_id: str_at(&data.header, &["id"]).unwrap_or_default(),
        game_status: espn_status_to_game_status(status),
        home_team: mlb_summary_team(&data.boxscore, home_comp),
        away_team: mlb_summary_team(&data.boxscore, away_comp),
    })
}

pub fn espn_nfl_summary(data: EspnSummaryDto) -> Result<NflBoxScore, AppError> {
    let header_comp = data
        .header
        .pointer("/competitions/0")
        .ok_or_else(|| AppError::parse("missing header competition"))?;
    let status = header_comp.get("status").unwrap_or(&Value::Null);
    let competitors = array_at(header_comp, &["competitors"]);
    let home_comp = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("home"))
        .ok_or_else(|| AppError::parse("missing home"))?;
    let away_comp = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("away"))
        .ok_or_else(|| AppError::parse("missing away"))?;
    Ok(NflBoxScore {
        game_id: str_at(&data.header, &["id"]).unwrap_or_default(),
        game_status: espn_status_to_game_status(status),
        home_team: nfl_summary_team(&data.boxscore, home_comp),
        away_team: nfl_summary_team(&data.boxscore, away_comp),
    })
}

pub fn espn_standings(data: EspnStandingsDto) -> StandingsTable {
    let mut east = Vec::new();
    let mut west = Vec::new();
    for group in data.children {
        let conf = str_at(&group, &["abbreviation"])
            .unwrap_or_else(|| str_at(&group, &["name"]).unwrap_or_default());
        let conference = if conf.to_lowercase().contains("west") {
            "West"
        } else {
            "East"
        };
        for entry in array_at(&group, &["standings", "entries"]) {
            let abbr = str_at(&entry, &["team", "abbreviation"]).unwrap_or_default();
            let (_, id, _, name) = team_mapping(&abbr).unwrap_or((
                abbr.as_str(),
                i64_from_str(&str_at(&entry, &["team", "id"]).unwrap_or_default()),
                "",
                "",
            ));
            let stats = array_at(&entry, &["stats"]);
            let row = StandingsTeam {
                team_id: id,
                team_name: if name.is_empty() {
                    str_at(&entry, &["team", "name"]).unwrap_or_default()
                } else {
                    name.to_string()
                },
                conference: conference.to_string(),
                playoff_rank: stat_value(&stats, "playoffSeed")
                    .or_else(|| stat_value(&stats, "rank"))
                    .unwrap_or(0.0) as i64,
                wins: stat_value(&stats, "wins").unwrap_or(0.0) as i64,
                losses: stat_value(&stats, "losses").unwrap_or(0.0) as i64,
                win_pct: stat_value(&stats, "winPercent").unwrap_or(0.0),
                conference_games_back: stat_value(&stats, "gamesBehind").unwrap_or(0.0),
                points_pg: stat_value(&stats, "avgPointsFor").unwrap_or(0.0),
                opp_points_pg: stat_value(&stats, "avgPointsAgainst").unwrap_or(0.0),
                diff_points_pg: stat_value(&stats, "differential").unwrap_or(0.0),
                home: stat_display(&stats, "Home")
                    .or_else(|| stat_display(&stats, "home"))
                    .unwrap_or_default(),
                road: stat_display(&stats, "Road")
                    .or_else(|| stat_display(&stats, "road"))
                    .unwrap_or_default(),
                last_ten: stat_display(&stats, "Last Ten Games")
                    .or_else(|| stat_display(&stats, "lastTen"))
                    .unwrap_or_default(),
                current_streak: stat_value(&stats, "streak").unwrap_or(0.0) as i64,
            };
            if conference == "West" {
                west.push(row);
            } else {
                east.push(row);
            }
        }
    }
    StandingsTable { east, west }
}

pub fn espn_mlb_standings(data: EspnStandingsDto) -> MlbStandingsTable {
    let mut divisions = vec![
        MlbStandingsDivision {
            league: "AL".to_string(),
            division: "East".to_string(),
            teams: Vec::new(),
        },
        MlbStandingsDivision {
            league: "AL".to_string(),
            division: "Central".to_string(),
            teams: Vec::new(),
        },
        MlbStandingsDivision {
            league: "AL".to_string(),
            division: "West".to_string(),
            teams: Vec::new(),
        },
        MlbStandingsDivision {
            league: "NL".to_string(),
            division: "East".to_string(),
            teams: Vec::new(),
        },
        MlbStandingsDivision {
            league: "NL".to_string(),
            division: "Central".to_string(),
            teams: Vec::new(),
        },
        MlbStandingsDivision {
            league: "NL".to_string(),
            division: "West".to_string(),
            teams: Vec::new(),
        },
    ];
    for group in data.children {
        for entry in array_at(&group, &["standings", "entries"]) {
            let stats = array_at(&entry, &["stats"]);
            let team_tricode = str_at(&entry, &["team", "abbreviation"]).unwrap_or_default();
            let (league, division) = mlb_division(&team_tricode).unwrap_or_else(|| {
                let abbr = str_at(&group, &["abbreviation"])
                    .unwrap_or_else(|| str_at(&group, &["name"]).unwrap_or_default());
                let league = if abbr.to_lowercase().contains("nl")
                    || str_at(&group, &["name"])
                        .unwrap_or_default()
                        .to_lowercase()
                        .contains("national")
                {
                    "NL"
                } else {
                    "AL"
                };
                (league, "Other")
            });
            let row = MlbStandingsTeam {
                team_id: str_at(&entry, &["team", "id"])
                    .map(|s| i64_from_str(&s))
                    .unwrap_or(0),
                team_name: str_at(&entry, &["team", "displayName"])
                    .or_else(|| str_at(&entry, &["team", "name"]))
                    .unwrap_or_default(),
                team_tricode,
                league: league.to_string(),
                division: division.to_string(),
                playoff_rank: stat_value(&stats, "playoffSeed")
                    .or_else(|| stat_value(&stats, "rank"))
                    .unwrap_or(0.0) as i64,
                wins: stat_value(&stats, "wins").unwrap_or(0.0) as i64,
                losses: stat_value(&stats, "losses").unwrap_or(0.0) as i64,
                win_pct: stat_display(&stats, "winPercent").unwrap_or_default(),
                games_back: String::new(),
                runs_scored: stat_value(&stats, "pointsFor").unwrap_or(0.0) as i64,
                runs_allowed: stat_value(&stats, "pointsAgainst").unwrap_or(0.0) as i64,
                run_diff: stat_display(&stats, "pointDifferential").unwrap_or_default(),
                streak: stat_display(&stats, "streak").unwrap_or_default(),
            };
            if let Some(group) = divisions
                .iter_mut()
                .find(|group| group.league == league && group.division == division)
            {
                group.teams.push(row);
            }
        }
    }
    for division in &mut divisions {
        division.teams.sort_by(|left, right| {
            win_pct_value(&right)
                .partial_cmp(&win_pct_value(&left))
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| right.wins.cmp(&left.wins))
                .then_with(|| left.losses.cmp(&right.losses))
        });
        let Some(leader) = division.teams.first().cloned() else {
            continue;
        };
        let mut last_record: Option<(i64, i64)> = None;
        let mut display_rank = 0;
        for (index, team) in division.teams.iter_mut().enumerate() {
            let record = (team.wins, team.losses);
            if last_record != Some(record) {
                display_rank = index as i64 + 1;
                last_record = Some(record);
            }
            team.playoff_rank = display_rank;
            team.games_back = format_games_back(games_back_from(&leader, team));
        }
    }
    MlbStandingsTable { divisions }
}

pub fn espn_nfl_standings(data: EspnStandingsDto) -> NflStandingsTable {
    let mut divisions = nfl_empty_divisions();
    for group in data.children {
        collect_nfl_standings(&group, None, None, &mut divisions);
    }
    for division in &mut divisions {
        division.teams.sort_by(|left, right| {
            right
                .wins
                .cmp(&left.wins)
                .then_with(|| left.losses.cmp(&right.losses))
                .then_with(|| right.ties.cmp(&left.ties))
        });
        for (idx, team) in division.teams.iter_mut().enumerate() {
            if team.playoff_rank == 0 {
                team.playoff_rank = idx as i64 + 1;
            }
        }
    }
    NflStandingsTable { divisions }
}

pub fn espn_player_gamelog(player_id: &str, data: EspnPlayerGamelogDto) -> PlayerStatsPage {
    let mut summary_rows = Vec::new();
    let mut game_rows = Vec::new();
    for season in data.season_types {
        let season_name = str_at(&season, &["displayName"]).unwrap_or_else(|| "Season".to_string());
        for summary in array_at(&season, &["summary", "stats"]) {
            let mut row = vec![format!(
                "{} {}",
                season_name,
                str_at(&summary, &["displayName"]).unwrap_or_else(|| "Summary".to_string())
            )];
            row.extend(array_at(&summary, &["stats"]).iter().map(value_to_string));
            summary_rows.push(row);
        }
        for category in array_at(&season, &["categories"]) {
            let group = str_at(&category, &["displayName"]).unwrap_or_else(|| season_name.clone());
            for event in array_at(&category, &["events"]) {
                let event_id = str_at(&event, &["eventId"]).unwrap_or_default();
                let info = data.events.get(&event_id).unwrap_or(&Value::Null);
                let mut row = vec![
                    str_at(info, &["gameDate"])
                        .map(|date| date.chars().take(10).collect())
                        .unwrap_or_default(),
                    format!(
                        "{} {}",
                        str_at(info, &["atVs"]).unwrap_or_default(),
                        str_at(info, &["opponent", "abbreviation"]).unwrap_or_default()
                    ),
                    str_at(info, &["gameResult"]).unwrap_or_default(),
                    str_at(info, &["score"]).unwrap_or_default(),
                    group.clone(),
                ];
                row.extend(array_at(&event, &["stats"]).iter().map(value_to_string));
                game_rows.push(row);
            }
        }
    }
    let mut summary_headers = vec!["Split".to_string()];
    summary_headers.extend(data.labels.clone());
    let mut game_headers = ["Date", "Opp", "Result", "Score", "Group"]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();
    game_headers.extend(data.labels);
    let _ = player_id;
    PlayerStatsPage {
        tables: vec![
            Table {
                name: "Summary".to_string(),
                headers: summary_headers,
                rows: summary_rows,
            },
            Table {
                name: "Game Log".to_string(),
                headers: game_headers,
                rows: game_rows,
            },
        ],
    }
}

pub fn nba_player_stats(data: Value) -> PlayerStatsPage {
    let result_sets = array_at(&data, &["resultSets"]);
    let visible = if result_sets.len() > 5 {
        result_sets.into_iter().skip(5).collect::<Vec<_>>()
    } else {
        result_sets
    };
    PlayerStatsPage {
        tables: visible
            .iter()
            .map(|rs| Table {
                name: str_at(rs, &["name"]).unwrap_or_else(|| "Stats".to_string()),
                headers: array_at(rs, &["headers"])
                    .iter()
                    .map(value_to_string)
                    .collect(),
                rows: array_at(rs, &["rowSet"])
                    .iter()
                    .map(|row| {
                        row.as_array()
                            .into_iter()
                            .flatten()
                            .map(value_to_string)
                            .collect()
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn nba_game(game: &Value) -> Game {
    Game {
        game_id: str_at(game, &["gameId"]).unwrap_or_default(),
        game_status: i64_at(game, &["gameStatus"]),
        game_status_text: str_at(game, &["gameStatusText"]).unwrap_or_default(),
        period: i64_at(game, &["period"]),
        game_clock: str_at(game, &["gameClock"]).unwrap_or_default(),
        game_time_utc: str_at(game, &["gameTimeUTC"]).unwrap_or_default(),
        home_team: nba_team(game.get("homeTeam").unwrap_or(&Value::Null)),
        away_team: nba_team(game.get("awayTeam").unwrap_or(&Value::Null)),
        home_leaders: Leaders::default(),
        away_leaders: Leaders::default(),
    }
}

fn nba_team(team: &Value) -> Team {
    Team {
        team_id: i64_at(team, &["teamId"]),
        team_name: str_at(team, &["teamName"]).unwrap_or_default(),
        team_city: str_at(team, &["teamCity"]).unwrap_or_default(),
        team_tricode: str_at(team, &["teamTricode"]).unwrap_or_default(),
        wins: i64_at(team, &["wins"]),
        losses: i64_at(team, &["losses"]),
        display_record: format!("{}-{}", i64_at(team, &["wins"]), i64_at(team, &["losses"])),
        score: i64_at(team, &["score"]),
        hits: 0,
        errors: 0,
        periods: array_at(team, &["periods"])
            .iter()
            .map(|p| Period {
                period: i64_at(p, &["period"]),
                score: i64_at(p, &["score"]),
            })
            .collect(),
    }
}

fn espn_competitor_to_team(c: &Value, competition: &Value) -> Team {
    let espn = str_at(c, &["team", "abbreviation"]).unwrap_or_default();
    let (tri, id, city, name) = match team_mapping(&espn) {
        Some((tri, id, city, name)) => (tri.to_string(), id, city.to_string(), name.to_string()),
        None => (
            espn,
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
    let display_record =
        playoff_series_record(c, competition).unwrap_or_else(|| format!("{wins}-{losses}"));
    Team {
        team_id: id,
        team_name: name,
        team_city: city,
        team_tricode: tri,
        wins,
        losses,
        display_record,
        score: str_at(c, &["score"]).map(|s| i64_from_str(&s)).unwrap_or(0),
        hits: stat_display(&array_at(c, &["statistics"]), "hits")
            .map(|s| i64_from_str(&s))
            .unwrap_or(0),
        errors: stat_display(&array_at(c, &["statistics"]), "errors")
            .map(|s| i64_from_str(&s))
            .unwrap_or(0),
        periods: array_at(c, &["linescores"])
            .iter()
            .map(|ls| Period {
                period: i64_at(ls, &["period"]),
                score: f64_at(ls, &["value"]) as i64,
            })
            .collect(),
    }
}

fn espn_mlb_competitor_to_team(c: &Value, _competition: &Value) -> Team {
    let (wins, losses) = array_at(c, &["records"])
        .iter()
        .find(|r| str_at(r, &["type"]).as_deref() == Some("total"))
        .and_then(|r| str_at(r, &["summary"]))
        .map(|s| parse_record(&s))
        .unwrap_or((0, 0));
    let stats = array_at(c, &["statistics"]);
    let linescores = array_at(c, &["linescores"]);
    Team {
        team_id: str_at(c, &["team", "id"])
            .map(|s| i64_from_str(&s))
            .unwrap_or_else(|| str_at(c, &["id"]).map(|s| i64_from_str(&s)).unwrap_or(0)),
        team_name: str_at(c, &["team", "name"]).unwrap_or_default(),
        team_city: str_at(c, &["team", "location"]).unwrap_or_default(),
        team_tricode: str_at(c, &["team", "abbreviation"]).unwrap_or_default(),
        wins,
        losses,
        display_record: format!("{wins}-{losses}"),
        score: str_at(c, &["score"]).map(|s| i64_from_str(&s)).unwrap_or(0),
        hits: stat_display(&stats, "hits")
            .map(|s| i64_from_str(&s))
            .unwrap_or_else(|| linescore_total(&linescores, "hits")),
        errors: stat_display(&stats, "errors")
            .map(|s| i64_from_str(&s))
            .unwrap_or_else(|| linescore_total(&linescores, "errors")),
        periods: mlb_linescore_periods(&linescores),
    }
}

fn espn_nfl_competitor_to_team(c: &Value, _competition: &Value) -> Team {
    let (wins, losses) = array_at(c, &["records"])
        .iter()
        .find(|r| str_at(r, &["type"]).as_deref() == Some("total"))
        .and_then(|r| str_at(r, &["summary"]))
        .map(|s| parse_record(&s))
        .unwrap_or((0, 0));
    let team_id = str_at(c, &["team", "id"])
        .or_else(|| str_at(c, &["id"]))
        .map(|s| i64_from_str(&s))
        .unwrap_or(0);
    let abbr = str_at(c, &["team", "abbreviation"]).unwrap_or_default();
    let team_info = nfl_team_info(team_id, &abbr);
    Team {
        team_id: team_info.1,
        team_name: str_at(c, &["team", "name"]).unwrap_or_else(|| team_info.3.to_string()),
        team_city: str_at(c, &["team", "location"]).unwrap_or_else(|| team_info.2.to_string()),
        team_tricode: if abbr.is_empty() {
            team_info.0.to_string()
        } else {
            abbr
        },
        wins,
        losses,
        display_record: array_at(c, &["records"])
            .iter()
            .find(|r| str_at(r, &["type"]).as_deref() == Some("total"))
            .and_then(|r| str_at(r, &["summary"]))
            .unwrap_or_else(|| format!("{wins}-{losses}")),
        score: str_at(c, &["score"]).map(|s| i64_from_str(&s)).unwrap_or(0),
        hits: 0,
        errors: 0,
        periods: array_at(c, &["linescores"])
            .iter()
            .enumerate()
            .map(|(idx, ls)| Period {
                period: i64_at(ls, &["period"]).max(idx as i64 + 1),
                score: str_at(ls, &["displayValue"])
                    .map(|s| i64_from_str(&s))
                    .unwrap_or_else(|| f64_at(ls, &["value"]) as i64),
            })
            .collect(),
    }
}

fn summary_team(boxscore: &Value, abbr: &str, comp: &Value) -> BoxScoreTeam {
    let (tri, id, city, name) = team_mapping(abbr).unwrap_or((abbr, 0, "", ""));
    let team_stats = array_at(boxscore, &["teams"])
        .into_iter()
        .find(|t| str_at(t, &["team", "abbreviation"]).as_deref() == Some(abbr))
        .map(|t| array_at(&t, &["statistics"]))
        .unwrap_or_default();
    let players = array_at(boxscore, &["players"])
        .into_iter()
        .find(|t| str_at(t, &["team", "abbreviation"]).as_deref() == Some(abbr))
        .map(|t| summary_players(&t))
        .unwrap_or_default();
    BoxScoreTeam {
        team: Team {
            team_id: id,
            team_name: name.to_string(),
            team_city: city.to_string(),
            team_tricode: tri.to_string(),
            wins: 0,
            losses: 0,
            display_record: String::new(),
            score: str_at(comp, &["score"])
                .map(|s| i64_from_str(&s))
                .unwrap_or(0),
            hits: 0,
            errors: 0,
            periods: array_at(comp, &["linescores"])
                .iter()
                .enumerate()
                .map(|(i, ls)| Period {
                    period: i as i64 + 1,
                    score: str_at(ls, &["displayValue"])
                        .map(|s| i64_from_str(&s))
                        .unwrap_or(0),
                })
                .collect(),
        },
        players,
        statistics: summary_team_stats(
            &team_stats,
            str_at(comp, &["score"])
                .map(|s| i64_from_str(&s))
                .unwrap_or(0),
        ),
    }
}

fn mlb_summary_team(boxscore: &Value, comp: &Value) -> MlbBoxScoreTeam {
    let abbr = str_at(comp, &["team", "abbreviation"]).unwrap_or_default();
    let team = espn_mlb_competitor_to_team(comp, &Value::Null);
    let player_group = array_at(boxscore, &["players"])
        .into_iter()
        .find(|t| str_at(t, &["team", "abbreviation"]).as_deref() == Some(abbr.as_str()))
        .unwrap_or(Value::Null);
    MlbBoxScoreTeam {
        team,
        batting: mlb_stat_table(&player_group, 0, "Batting"),
        pitching: mlb_stat_table(&player_group, 1, "Pitching"),
    }
}

fn nfl_summary_team(boxscore: &Value, comp: &Value) -> NflBoxScoreTeam {
    let team = espn_nfl_competitor_to_team(comp, &Value::Null);
    let abbr = team.team_tricode.clone();
    let team_group = array_at(boxscore, &["teams"])
        .into_iter()
        .find(|t| {
            str_at(t, &["team", "abbreviation"]).as_deref() == Some(abbr.as_str())
                || str_at(t, &["homeAway"]) == str_at(comp, &["homeAway"])
        })
        .unwrap_or(Value::Null);
    let player_group = array_at(boxscore, &["players"])
        .into_iter()
        .find(|t| str_at(t, &["team", "abbreviation"]).as_deref() == Some(abbr.as_str()))
        .unwrap_or(Value::Null);
    NflBoxScoreTeam {
        team,
        team_stats: nfl_team_stats_table(&team_group),
        player_stats: nfl_player_tables(&player_group),
    }
}

fn nfl_team_stats_table(group: &Value) -> Table {
    let rows = array_at(group, &["statistics"])
        .iter()
        .filter_map(|stat| {
            let label = str_at(stat, &["label"]).or_else(|| str_at(stat, &["displayName"]))?;
            Some(vec![
                label,
                str_at(stat, &["displayValue"]).unwrap_or_else(|| value_to_string(stat)),
            ])
        })
        .collect();
    Table {
        name: "Team Stats".to_string(),
        headers: vec!["Stat".to_string(), "Value".to_string()],
        rows,
    }
}

fn nfl_player_tables(group: &Value) -> Vec<Table> {
    array_at(group, &["statistics"])
        .iter()
        .map(|stat_group| {
            let mut headers = vec!["Name".to_string()];
            headers.extend(
                array_at(stat_group, &["labels"])
                    .iter()
                    .map(value_to_string),
            );
            let mut rows: Vec<Vec<String>> = array_at(stat_group, &["athletes"])
                .iter()
                .map(|athlete| {
                    let mut row =
                        vec![str_at(athlete, &["athlete", "displayName"]).unwrap_or_default()];
                    row.extend(array_at(athlete, &["stats"]).iter().map(value_to_string));
                    row
                })
                .collect();
            let totals = array_at(stat_group, &["totals"]);
            if !totals.is_empty() {
                let mut row = vec!["Total".to_string()];
                row.extend(totals.iter().map(value_to_string));
                rows.push(row);
            }
            Table {
                name: str_at(stat_group, &["text"])
                    .or_else(|| str_at(stat_group, &["name"]))
                    .unwrap_or_else(|| "Stats".to_string()),
                headers,
                rows,
            }
        })
        .collect()
}

fn mlb_stat_table(group: &Value, index: usize, name: &str) -> Table {
    let stats = array_at(group, &["statistics"]);
    let Some(stat_group) = stats.get(index) else {
        return Table {
            name: name.to_string(),
            headers: vec!["Name".to_string()],
            rows: Vec::new(),
        };
    };
    let mut headers = vec!["Name".to_string()];
    headers.extend(
        array_at(stat_group, &["labels"])
            .iter()
            .map(value_to_string),
    );
    let rows = array_at(stat_group, &["athletes"])
        .iter()
        .map(|athlete| {
            let mut row = vec![str_at(athlete, &["athlete", "displayName"]).unwrap_or_default()];
            row.extend(array_at(athlete, &["stats"]).iter().map(value_to_string));
            row
        })
        .collect();
    Table {
        name: name.to_string(),
        headers,
        rows,
    }
}

fn summary_players(team: &Value) -> Vec<Player> {
    let mut out = Vec::new();
    for group in array_at(team, &["statistics"]) {
        let keys: Vec<String> = array_at(&group, &["keys"])
            .iter()
            .map(value_to_string)
            .collect();
        for athlete in array_at(&group, &["athletes"]) {
            let stats: Vec<String> = array_at(&athlete, &["stats"])
                .iter()
                .map(value_to_string)
                .collect();
            out.push(Player {
                person_id: str_at(&athlete, &["athlete", "id"])
                    .map(|s| i64_from_str(&s))
                    .unwrap_or(0),
                name: str_at(&athlete, &["athlete", "displayName"]).unwrap_or_default(),
                starter: bool_at(&athlete, &["starter"]),
                played: !bool_at(&athlete, &["didNotPlay"]),
                statistics: player_statistics(&keys, &stats),
            });
        }
    }
    out
}

fn player_statistics(keys: &[String], stats: &[String]) -> Statistics {
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
    Statistics {
        assists: num(&get("assists")),
        blocks: num(&get("blocks")),
        field_goals_attempted: fga,
        field_goals_made: fgm,
        fouls_personal: num(&get("fouls")),
        free_throws_attempted: fta,
        free_throws_made: ftm,
        minutes: num(&get("minutes")),
        plus_minus_points: i64_from_str(&get("plusMinus").replace('+', "")),
        points: num(&get("points")),
        rebounds_defensive: num(&get("defensiveRebounds")),
        rebounds_offensive: num(&get("offensiveRebounds")),
        rebounds_total: num(&get("rebounds")),
        steals: num(&get("steals")),
        three_pointers_attempted: tpa,
        three_pointers_made: tpm,
        turnovers: num(&get("turnovers")),
        ..Statistics::default()
    }
}

fn summary_team_stats(stats: &[Value], points: i64) -> TeamStatistics {
    let (fgm, fga) = stat_split(stats, "fieldGoalsMade-fieldGoalsAttempted");
    let (tpm, tpa) = stat_split(
        stats,
        "threePointFieldGoalsMade-threePointFieldGoalsAttempted",
    );
    let (ftm, fta) = stat_split(stats, "freeThrowsMade-freeThrowsAttempted");
    TeamStatistics {
        assists: stat_num(stats, "assists"),
        blocks: stat_num(stats, "blocks"),
        field_goals_attempted: fga,
        field_goals_made: fgm,
        fouls_personal: stat_num(stats, "fouls"),
        free_throws_attempted: fta,
        free_throws_made: ftm,
        minutes: 240,
        points,
        rebounds_defensive: stat_num(stats, "defensiveRebounds"),
        rebounds_offensive: stat_num(stats, "offensiveRebounds"),
        rebounds_total: stat_num(stats, "rebounds"),
        steals: stat_num(stats, "steals"),
        three_pointers_attempted: tpa,
        three_pointers_made: tpm,
        turnovers: stat_num(stats, "turnovers"),
    }
}

fn extract_leaders(c: &Value) -> Leaders {
    let leaders = array_at(c, &["leaders"]);
    let athlete = leaders
        .iter()
        .find(|l| str_at(l, &["name"]).as_deref() == Some("points"))
        .and_then(|l| l.pointer("/leaders/0/athlete"))
        .unwrap_or(&Value::Null);
    Leaders {
        person_id: str_at(athlete, &["id"])
            .map(|s| i64_from_str(&s))
            .unwrap_or(0),
        name: str_at(athlete, &["displayName"]).unwrap_or_default(),
        team_tricode: str_at(c, &["team", "abbreviation"]).unwrap_or_default(),
        points: leader_value(&leaders, "points"),
        rebounds: leader_value(&leaders, "rebounds"),
        assists: leader_value(&leaders, "assists"),
    }
}

fn playoff_series_record(competitor: &Value, competition: &Value) -> Option<String> {
    if str_at(competition, &["series", "type"]).as_deref() != Some("playoff") {
        return None;
    }
    let team_id = str_at(competitor, &["team", "id"]).or_else(|| str_at(competitor, &["id"]))?;
    let team_wins = array_at(competition, &["series", "competitors"])
        .iter()
        .find(|series_team| str_at(series_team, &["id"]).as_deref() == Some(team_id.as_str()))
        .map(|series_team| i64_at(series_team, &["wins"]))?;
    let opponent_wins = array_at(competition, &["series", "competitors"])
        .iter()
        .find(|series_team| str_at(series_team, &["id"]).as_deref() != Some(team_id.as_str()))
        .map(|series_team| i64_at(series_team, &["wins"]))
        .unwrap_or(0);
    Some(format!("{team_wins}-{opponent_wins}"))
}

fn espn_status_to_game_status(status: &Value) -> i64 {
    match str_at(status, &["type", "name"]).as_deref() {
        Some("STATUS_FINAL") => 3,
        Some("STATUS_IN_PROGRESS") => 2,
        _ => 1,
    }
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

fn mlb_division(abbr: &str) -> Option<(&'static str, &'static str)> {
    Some(match abbr {
        "BAL" | "BOS" | "NYY" | "TB" | "TOR" => ("AL", "East"),
        "CHW" | "CWS" | "CLE" | "DET" | "KC" | "KCR" | "MIN" => ("AL", "Central"),
        "HOU" | "LAA" | "ATH" | "OAK" | "SEA" | "TEX" => ("AL", "West"),
        "ATL" | "MIA" | "NYM" | "PHI" | "WSH" | "WAS" => ("NL", "East"),
        "CHC" | "CIN" | "MIL" | "PIT" | "STL" => ("NL", "Central"),
        "ARI" | "AZ" | "COL" | "LAD" | "SD" | "SDP" | "SF" | "SFG" => ("NL", "West"),
        _ => return None,
    })
}

fn nfl_empty_divisions() -> Vec<NflStandingsDivision> {
    ["AFC", "NFC"]
        .iter()
        .flat_map(|conference| {
            ["East", "North", "South", "West"]
                .iter()
                .map(move |division| NflStandingsDivision {
                    conference: (*conference).to_string(),
                    division: (*division).to_string(),
                    teams: Vec::new(),
                })
        })
        .collect()
}

fn collect_nfl_standings(
    group: &Value,
    conference_hint: Option<&str>,
    division_hint: Option<&str>,
    divisions: &mut [NflStandingsDivision],
) {
    let group_name = str_at(group, &["name"]).unwrap_or_default();
    let group_abbr = str_at(group, &["abbreviation"]).unwrap_or_default();
    let conference = if group_abbr == "AFC" || group_name.contains("American") {
        Some("AFC")
    } else if group_abbr == "NFC" || group_name.contains("National") {
        Some("NFC")
    } else {
        conference_hint
    };
    let division = nfl_division_name(&group_name).or(division_hint);
    for child in array_at(group, &["children"]) {
        collect_nfl_standings(&child, conference, division, divisions);
    }
    for entry in array_at(group, &["standings", "entries"]) {
        let stats = array_at(&entry, &["stats"]);
        let id = str_at(&entry, &["team", "id"])
            .map(|s| i64_from_str(&s))
            .unwrap_or(0);
        let abbr = str_at(&entry, &["team", "abbreviation"]).unwrap_or_default();
        let (mapped_abbr, mapped_id, city, name) = nfl_team_info(id, &abbr);
        let (fallback_conference, fallback_division) =
            nfl_division(mapped_abbr).unwrap_or(("AFC", "East"));
        let conference = conference.unwrap_or(fallback_conference);
        let division = division.unwrap_or(fallback_division);
        let team_name = str_at(&entry, &["team", "displayName"])
            .or_else(|| {
                str_at(&entry, &["team", "name"]).map(|n| {
                    if n == name {
                        format!("{city} {name}")
                    } else {
                        n
                    }
                })
            })
            .unwrap_or_else(|| format!("{city} {name}"));
        let row = NflStandingsTeam {
            team_id: mapped_id,
            team_name,
            team_tricode: mapped_abbr.to_string(),
            conference: conference.to_string(),
            division: division.to_string(),
            playoff_rank: stat_value(&stats, "playoffSeed")
                .or_else(|| stat_value(&stats, "rank"))
                .unwrap_or(0.0) as i64,
            wins: stat_value(&stats, "wins").unwrap_or(0.0) as i64,
            losses: stat_value(&stats, "losses").unwrap_or(0.0) as i64,
            ties: stat_value(&stats, "ties").unwrap_or(0.0) as i64,
            win_pct: stat_display(&stats, "winPercent").unwrap_or_default(),
            games_back: stat_display(&stats, "gamesBehind").unwrap_or_default(),
            points_for: stat_value(&stats, "pointsFor").unwrap_or(0.0) as i64,
            points_against: stat_value(&stats, "pointsAgainst").unwrap_or(0.0) as i64,
            point_diff: stat_display(&stats, "pointDifferential")
                .or_else(|| stat_display(&stats, "differential"))
                .unwrap_or_default(),
            home: stat_display(&stats, "Home").unwrap_or_default(),
            road: stat_display(&stats, "Road").unwrap_or_default(),
            division_record: stat_display(&stats, "vs. Div.")
                .or_else(|| stat_display(&stats, "divisionRecord"))
                .unwrap_or_default(),
            conference_record: stat_display(&stats, "vs. Conf.").unwrap_or_default(),
            streak: stat_display(&stats, "streak").unwrap_or_default(),
        };
        if let Some(group) = divisions
            .iter_mut()
            .find(|group| group.conference == conference && group.division == division)
        {
            group.teams.push(row);
        }
    }
}

fn nfl_division_name(name: &str) -> Option<&'static str> {
    if name.contains("East") {
        Some("East")
    } else if name.contains("North") {
        Some("North")
    } else if name.contains("South") {
        Some("South")
    } else if name.contains("West") {
        Some("West")
    } else {
        None
    }
}

fn nfl_division(abbr: &str) -> Option<(&'static str, &'static str)> {
    Some(match abbr {
        "BUF" | "MIA" | "NE" | "NYJ" => ("AFC", "East"),
        "BAL" | "CIN" | "CLE" | "PIT" => ("AFC", "North"),
        "HOU" | "IND" | "JAX" | "TEN" => ("AFC", "South"),
        "DEN" | "KC" | "LV" | "LAC" => ("AFC", "West"),
        "DAL" | "NYG" | "PHI" | "WSH" => ("NFC", "East"),
        "CHI" | "DET" | "GB" | "MIN" => ("NFC", "North"),
        "ATL" | "CAR" | "NO" | "TB" => ("NFC", "South"),
        "ARI" | "LAR" | "SF" | "SEA" => ("NFC", "West"),
        _ => return None,
    })
}

fn nfl_team_info(id: i64, abbr: &str) -> (&'static str, i64, &'static str, &'static str) {
    match (id, abbr) {
        (22, _) | (_, "ARI") => ("ARI", 22, "Arizona", "Cardinals"),
        (1, _) | (_, "ATL") => ("ATL", 1, "Atlanta", "Falcons"),
        (33, _) | (_, "BAL") => ("BAL", 33, "Baltimore", "Ravens"),
        (2, _) | (_, "BUF") => ("BUF", 2, "Buffalo", "Bills"),
        (29, _) | (_, "CAR") => ("CAR", 29, "Carolina", "Panthers"),
        (3, _) | (_, "CHI") => ("CHI", 3, "Chicago", "Bears"),
        (4, _) | (_, "CIN") => ("CIN", 4, "Cincinnati", "Bengals"),
        (5, _) | (_, "CLE") => ("CLE", 5, "Cleveland", "Browns"),
        (6, _) | (_, "DAL") => ("DAL", 6, "Dallas", "Cowboys"),
        (7, _) | (_, "DEN") => ("DEN", 7, "Denver", "Broncos"),
        (8, _) | (_, "DET") => ("DET", 8, "Detroit", "Lions"),
        (9, _) | (_, "GB") => ("GB", 9, "Green Bay", "Packers"),
        (34, _) | (_, "HOU") => ("HOU", 34, "Houston", "Texans"),
        (11, _) | (_, "IND") => ("IND", 11, "Indianapolis", "Colts"),
        (30, _) | (_, "JAX") => ("JAX", 30, "Jacksonville", "Jaguars"),
        (12, _) | (_, "KC") => ("KC", 12, "Kansas City", "Chiefs"),
        (13, _) | (_, "LV") => ("LV", 13, "Las Vegas", "Raiders"),
        (24, _) | (_, "LAC") => ("LAC", 24, "Los Angeles", "Chargers"),
        (14, _) | (_, "LAR") => ("LAR", 14, "Los Angeles", "Rams"),
        (15, _) | (_, "MIA") => ("MIA", 15, "Miami", "Dolphins"),
        (16, _) | (_, "MIN") => ("MIN", 16, "Minnesota", "Vikings"),
        (17, _) | (_, "NE") => ("NE", 17, "New England", "Patriots"),
        (18, _) | (_, "NO") => ("NO", 18, "New Orleans", "Saints"),
        (19, _) | (_, "NYG") => ("NYG", 19, "New York", "Giants"),
        (20, _) | (_, "NYJ") => ("NYJ", 20, "New York", "Jets"),
        (21, _) | (_, "PHI") => ("PHI", 21, "Philadelphia", "Eagles"),
        (23, _) | (_, "PIT") => ("PIT", 23, "Pittsburgh", "Steelers"),
        (25, _) | (_, "SF") => ("SF", 25, "San Francisco", "49ers"),
        (26, _) | (_, "SEA") => ("SEA", 26, "Seattle", "Seahawks"),
        (27, _) | (_, "TB") => ("TB", 27, "Tampa Bay", "Buccaneers"),
        (10, _) | (_, "TEN") => ("TEN", 10, "Tennessee", "Titans"),
        (28, _) | (_, "WSH") => ("WSH", 28, "Washington", "Commanders"),
        _ => ("", id, "", ""),
    }
}

fn stat_num(stats: &[Value], name: &str) -> i64 {
    stats
        .iter()
        .find(|s| str_at(s, &["name"]).as_deref() == Some(name))
        .and_then(|s| str_at(s, &["displayValue"]))
        .map(|s| num(&s))
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
        .and_then(|s| f64_opt_at(s, &["value"]))
}
fn stat_display(stats: &[Value], name: &str) -> Option<String> {
    stats
        .iter()
        .find(|s| str_at(s, &["name"]).as_deref() == Some(name))
        .and_then(|s| str_at(s, &["displayValue"]))
}
fn mlb_linescore_periods(linescores: &[Value]) -> Vec<Period> {
    linescores
        .iter()
        .enumerate()
        .map(|(index, line)| Period {
            period: i64_at(line, &["period"]).max(index as i64 + 1),
            score: f64_opt_at(line, &["value"])
                .or_else(|| f64_opt_at(line, &["displayValue"]))
                .unwrap_or(0.0) as i64,
        })
        .collect()
}
fn linescore_total(linescores: &[Value], name: &str) -> i64 {
    linescores.iter().map(|line| i64_at(line, &[name])).sum()
}
fn leader_value(leaders: &[Value], name: &str) -> f64 {
    leaders
        .iter()
        .find(|l| str_at(l, &["name"]).as_deref() == Some(name))
        .and_then(|l| l.pointer("/leaders/0/displayValue").map(value_to_string))
        .and_then(|s| s.split_whitespace().next().and_then(|n| n.parse().ok()))
        .unwrap_or(0.0)
}
fn str_at(v: &Value, path: &[&str]) -> Option<String> {
    let mut current = v;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str().map(str::to_string)
}
fn i64_at(v: &Value, path: &[&str]) -> i64 {
    let mut current = v;
    for key in path {
        let Some(next) = current.get(*key) else {
            return 0;
        };
        current = next;
    }
    current
        .as_i64()
        .or_else(|| current.as_f64().map(|n| n as i64))
        .or_else(|| current.as_str()?.parse().ok())
        .unwrap_or(0)
}
fn f64_at(v: &Value, path: &[&str]) -> f64 {
    f64_opt_at(v, path).unwrap_or(0.0)
}
fn f64_opt_at(v: &Value, path: &[&str]) -> Option<f64> {
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
        parts.next().map(i64_from_str).unwrap_or(0),
        parts.next().map(i64_from_str).unwrap_or(0),
    )
}
fn parse_made_attempted(val: &str) -> (i64, i64) {
    let mut parts = val.split('-');
    (
        parts.next().map(num).unwrap_or(0),
        parts.next().map(num).unwrap_or(0),
    )
}
fn num(s: &str) -> i64 {
    s.parse::<f64>().map(|n| n as i64).unwrap_or(0)
}
fn i64_from_str(s: &str) -> i64 {
    s.parse::<i64>().unwrap_or(0)
}

fn win_pct_value(team: &MlbStandingsTeam) -> f64 {
    let games = team.wins + team.losses;
    if games == 0 {
        0.0
    } else {
        team.wins as f64 / games as f64
    }
}

fn games_back_from(leader: &MlbStandingsTeam, team: &MlbStandingsTeam) -> f64 {
    ((leader.wins - team.wins) as f64 + (team.losses - leader.losses) as f64) / 2.0
}

fn format_games_back(value: f64) -> String {
    if value <= 0.0 {
        "-".to_string()
    } else if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{value:.1}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_gamelog_conversion_produces_tables() {
        let data: EspnPlayerGamelogDto = serde_json::from_value(serde_json::json!({
            "labels": ["MIN", "PTS"],
            "events": {"1": {"gameDate": "2026-01-01T00:00:00Z", "atVs": "@", "opponent": {"abbreviation": "BOS"}, "gameResult": "W", "score": "100-90"}},
            "seasonTypes": [{"displayName": "Season", "summary": {"stats": [{"displayName": "Averages", "stats": ["30", "20"]}]}, "categories": [{"displayName": "january", "events": [{"eventId": "1", "stats": ["30", "20"]}]}]}]
        })).unwrap();
        let page = espn_player_gamelog("1", data);
        assert_eq!(page.tables.len(), 2);
        assert_eq!(page.tables[0].name, "Summary");
        assert_eq!(page.tables[1].rows[0][1], "@ BOS");
    }

    #[test]
    fn espn_scoreboard_conversion_produces_domain_game() {
        let data: EspnScoreboardDto = serde_json::from_value(serde_json::json!({
            "events": [{
                "id": "401869385",
                "date": "2026-04-26T19:00:00Z",
                "competitions": [{
                    "series": {"type": "playoff", "competitors": [{"id": "13", "wins": 2}, {"id": "2", "wins": 1}]},
                    "status": {"period": 4, "displayClock": "0.0", "type": {"name": "STATUS_FINAL", "shortDetail": "Final"}},
                    "competitors": [
                        {"homeAway": "away", "id": "2", "score": "128", "team": {"id": "2", "abbreviation": "BOS"}, "records": [{"type": "total", "summary": "56-26"}], "linescores": [{"period": 1, "value": 34}]},
                        {"homeAway": "home", "id": "13", "score": "96", "team": {"id": "13", "abbreviation": "LAL"}, "records": [{"type": "total", "summary": "53-29"}], "linescores": [{"period": 1, "value": 21}]}
                    ]
                }]
            }]
        })).unwrap();
        let scoreboard = espn_scoreboard("2026-04-26", data).unwrap();
        assert_eq!(scoreboard.games.len(), 1);
        assert_eq!(scoreboard.games[0].home_team.team_tricode, "LAL");
        assert_eq!(scoreboard.games[0].home_team.display_record, "2-1");
        assert_eq!(scoreboard.games[0].away_team.score, 128);
        assert_eq!(scoreboard.games[0].game_status, 3);
    }

    #[test]
    fn espn_mlb_scoreboard_conversion_produces_domain_game_with_rhe() {
        let data: EspnScoreboardDto = serde_json::from_value(serde_json::json!({
            "events": [{
                "id": "401815095",
                "date": "2026-04-26T17:35:00Z",
                "competitions": [{
                    "status": {"period": 9, "displayClock": "0.0", "type": {"name": "STATUS_FINAL", "shortDetail": "Final"}},
                    "competitors": [
                        {"homeAway": "away", "id": "2", "score": "5", "team": {"id": "2", "location": "Boston", "name": "Red Sox", "abbreviation": "BOS"}, "records": [{"type": "total", "summary": "11-17"}], "statistics": [{"name": "hits", "displayValue": "7"}, {"name": "errors", "displayValue": "0"}], "linescores": [{"period": 1, "value": 0}, {"period": 2, "value": 3}]},
                        {"homeAway": "home", "id": "1", "score": "3", "team": {"id": "1", "location": "Baltimore", "name": "Orioles", "abbreviation": "BAL"}, "records": [{"type": "total", "summary": "13-15"}], "statistics": [{"name": "hits", "displayValue": "6"}, {"name": "errors", "displayValue": "1"}], "linescores": [{"period": 1, "value": 0}, {"period": 2, "value": 1}]}
                    ]
                }]
            }]
        }))
        .unwrap();
        let scoreboard = espn_mlb_scoreboard("2026-04-26", data).unwrap();
        assert_eq!(scoreboard.games.len(), 1);
        assert_eq!(scoreboard.games[0].away_team.team_name, "Red Sox");
        assert_eq!(scoreboard.games[0].away_team.team_id, 2);
        assert_eq!(scoreboard.games[0].away_team.score, 5);
        assert_eq!(scoreboard.games[0].away_team.hits, 7);
        assert_eq!(scoreboard.games[0].home_team.errors, 1);
        assert_eq!(scoreboard.games[0].game_status, 3);
    }

    #[test]
    fn espn_mlb_summary_conversion_produces_batting_and_pitching_tables() {
        let data: EspnSummaryDto = serde_json::from_value(serde_json::json!({
            "header": {
                "id": "401815095",
                "competitions": [{
                    "status": {"type": {"name": "STATUS_FINAL"}},
                    "competitors": [
                        {"homeAway": "away", "score": "5", "team": {"id": "2", "location": "Boston", "name": "Red Sox", "abbreviation": "BOS"}, "records": [{"type": "total", "summary": "11-17"}], "statistics": [{"name": "hits", "displayValue": "7"}, {"name": "errors", "displayValue": "0"}], "linescores": []},
                        {"homeAway": "home", "score": "3", "team": {"id": "1", "location": "Baltimore", "name": "Orioles", "abbreviation": "BAL"}, "records": [{"type": "total", "summary": "13-15"}], "statistics": [{"name": "hits", "displayValue": "6"}, {"name": "errors", "displayValue": "1"}], "linescores": []}
                    ]
                }]
            },
            "boxscore": {
                "players": [
                    {"team": {"abbreviation": "BOS"}, "statistics": [
                        {"labels": ["AB", "RBI"], "athletes": [{"athlete": {"displayName": "Rafael Devers"}, "stats": ["4", "2"]}]},
                        {"labels": ["IP", "K"], "athletes": [{"athlete": {"displayName": "Connelly Early"}, "stats": ["6.2", "4"]}]}
                    ]},
                    {"team": {"abbreviation": "BAL"}, "statistics": [
                        {"labels": ["AB", "RBI"], "athletes": [{"athlete": {"displayName": "Gunnar Henderson"}, "stats": ["4", "2"]}]},
                        {"labels": ["IP", "K"], "athletes": [{"athlete": {"displayName": "Kyle Bradish"}, "stats": ["5", "3"]}]}
                    ]}
                ]
            },
            "gameInfo": null
        }))
        .unwrap();
        let game = espn_mlb_summary(data).unwrap();
        assert_eq!(game.away_team.batting.headers, vec!["Name", "AB", "RBI"]);
        assert_eq!(game.away_team.batting.rows[0][0], "Rafael Devers");
        assert_eq!(game.away_team.pitching.rows[0][0], "Connelly Early");
        assert_eq!(game.home_team.team.errors, 1);
    }

    #[test]
    fn espn_mlb_summary_conversion_reads_display_value_linescores() {
        let data: EspnSummaryDto = serde_json::from_value(serde_json::json!({
            "header": {
                "id": "401815112",
                "competitions": [{
                    "status": {"type": {"name": "STATUS_FINAL"}},
                    "competitors": [
                        {"homeAway": "away", "score": "3", "team": {"id": "30", "location": "Tampa Bay", "name": "Rays", "abbreviation": "TB"}, "records": [], "linescores": [
                            {"displayValue": "0", "hits": 0, "errors": 0},
                            {"displayValue": "1", "hits": 1, "errors": 0},
                            {"displayValue": "2", "hits": 3, "errors": 0}
                        ]},
                        {"homeAway": "home", "score": "2", "team": {"id": "5", "location": "Cleveland", "name": "Guardians", "abbreviation": "CLE"}, "records": [], "linescores": [
                            {"displayValue": "0", "hits": 1, "errors": 0},
                            {"displayValue": "2", "hits": 2, "errors": 1},
                            {"displayValue": "0", "hits": 0, "errors": 0}
                        ]}
                    ]
                }]
            },
            "boxscore": {"players": []},
            "gameInfo": null
        }))
        .unwrap();

        let game = espn_mlb_summary(data).unwrap();

        assert_eq!(game.away_team.team.periods[0].period, 1);
        assert_eq!(game.away_team.team.periods[1].score, 1);
        assert_eq!(game.away_team.team.periods[2].score, 2);
        assert_eq!(game.away_team.team.hits, 4);
        assert_eq!(game.home_team.team.hits, 3);
        assert_eq!(game.home_team.team.errors, 1);
    }

    #[test]
    fn espn_nfl_scoreboard_conversion_uses_team_mapping() {
        let data: EspnScoreboardDto = serde_json::from_value(serde_json::json!({
            "events": [{
                "id": "401772845",
                "date": "2026-01-04T18:00:00Z",
                "competitions": [{
                    "status": {"period": 4, "displayClock": "0:00", "type": {"name": "STATUS_FINAL", "shortDetail": "Final"}},
                    "competitors": [
                        {"homeAway": "away", "id": "21", "score": "31", "team": {"id": "21", "abbreviation": null}, "records": [{"type": "total", "summary": "13-4"}], "linescores": [{"period": 1, "value": 14}, {"period": 2, "value": 10}, {"period": 3, "value": 7}, {"period": 4, "value": 0}]},
                        {"homeAway": "home", "id": "27", "score": "25", "team": {"id": "27", "abbreviation": null}, "records": [{"type": "total", "summary": "10-7"}], "linescores": [{"period": 1, "value": 3}, {"period": 2, "value": 3}, {"period": 3, "value": 14}, {"period": 4, "value": 5}]}
                    ]
                }]
            }]
        }))
        .unwrap();
        let scoreboard = espn_nfl_scoreboard("2026-01-04", data).unwrap();
        assert_eq!(scoreboard.games.len(), 1);
        assert_eq!(scoreboard.games[0].away_team.team_tricode, "PHI");
        assert_eq!(scoreboard.games[0].away_team.team_name, "Eagles");
        assert_eq!(scoreboard.games[0].home_team.team_city, "Tampa Bay");
        assert_eq!(scoreboard.games[0].home_team.score, 25);
        assert_eq!(scoreboard.games[0].game_status, 3);
    }

    #[test]
    fn espn_nfl_summary_conversion_produces_team_and_player_tables() {
        let data: EspnSummaryDto = serde_json::from_value(serde_json::json!({
            "header": {
                "id": "401772845",
                "competitions": [{
                    "status": {"type": {"name": "STATUS_FINAL"}},
                    "competitors": [
                        {"homeAway": "away", "id": "21", "score": "31", "team": {"id": "21"}, "linescores": [{"displayValue": "14"}, {"displayValue": "10"}]},
                        {"homeAway": "home", "id": "27", "score": "25", "team": {"id": "27"}, "linescores": [{"displayValue": "3"}, {"displayValue": "3"}]}
                    ]
                }]
            },
            "boxscore": {
                "teams": [
                    {"homeAway": "away", "team": {"abbreviation": "PHI"}, "statistics": [{"label": "Total Yards", "displayValue": "200"}]},
                    {"homeAway": "home", "team": {"abbreviation": "TB"}, "statistics": [{"label": "Total Yards", "displayValue": "376"}]}
                ],
                "players": [
                    {"team": {"abbreviation": "PHI"}, "statistics": [{"text": "Philadelphia Passing", "labels": ["C/ATT", "YDS"], "athletes": [{"athlete": {"displayName": "Jalen Hurts"}, "stats": ["15/24", "130"]}], "totals": ["15/24", "112"]}]},
                    {"team": {"abbreviation": "TB"}, "statistics": [{"text": "Tampa Bay Passing", "labels": ["C/ATT", "YDS"], "athletes": [{"athlete": {"displayName": "Baker Mayfield"}, "stats": ["22/40", "272"]}], "totals": ["22/40", "272"]}]}
                ]
            },
            "gameInfo": null
        }))
        .unwrap();
        let game = espn_nfl_summary(data).unwrap();
        assert_eq!(game.away_team.team.team_tricode, "PHI");
        assert_eq!(game.away_team.team_stats.rows[0][0], "Total Yards");
        assert_eq!(game.away_team.player_stats[0].name, "Philadelphia Passing");
        assert_eq!(game.away_team.player_stats[0].rows[0][0], "Jalen Hurts");
        assert_eq!(game.away_team.player_stats[0].rows[1][0], "Total");
    }

    #[test]
    fn espn_mlb_standings_conversion_groups_by_division() {
        let data: EspnStandingsDto = serde_json::from_value(serde_json::json!({
            "children": [
                {"abbreviation": "AL", "standings": {"entries": [
                  {"team": {"id": "10", "displayName": "New York Yankees", "abbreviation": "NYY"}, "stats": [
                    {"name": "playoffSeed", "value": 1, "displayValue": "1"},
                    {"name": "wins", "value": 18, "displayValue": "18"},
                    {"name": "losses", "value": 10, "displayValue": "10"},
                    {"name": "winPercent", "value": 0.643, "displayValue": ".643"},
                    {"name": "gamesBehind", "value": 0, "displayValue": "-"},
                    {"name": "pointsFor", "value": 146, "displayValue": "146"},
                    {"name": "pointsAgainst", "value": 99, "displayValue": "99"},
                    {"name": "pointDifferential", "value": 47, "displayValue": "+47"},
                    {"name": "streak", "value": -1, "displayValue": "L1"}
                  ]},
                  {"team": {"id": "30", "displayName": "Tampa Bay Rays", "abbreviation": "TB"}, "stats": [
                    {"name": "playoffSeed", "value": 4, "displayValue": "4"},
                    {"name": "wins", "value": 18, "displayValue": "18"},
                    {"name": "losses", "value": 10, "displayValue": "10"},
                    {"name": "winPercent", "value": 0.643, "displayValue": ".643"},
                    {"name": "gamesBehind", "value": 0, "displayValue": "-"},
                    {"name": "pointsFor", "value": 130, "displayValue": "130"},
                    {"name": "pointsAgainst", "value": 101, "displayValue": "101"},
                    {"name": "pointDifferential", "value": 29, "displayValue": "+29"},
                    {"name": "streak", "value": 1, "displayValue": "W1"}
                  ]},
                  {"team": {"id": "1", "displayName": "Baltimore Orioles", "abbreviation": "BAL"}, "stats": [
                    {"name": "playoffSeed", "value": 8, "displayValue": "8"},
                    {"name": "wins", "value": 16, "displayValue": "16"},
                    {"name": "losses", "value": 12, "displayValue": "12"},
                    {"name": "winPercent", "value": 0.571, "displayValue": ".571"},
                    {"name": "gamesBehind", "value": 2, "displayValue": "2"},
                    {"name": "pointsFor", "value": 120, "displayValue": "120"},
                    {"name": "pointsAgainst", "value": 110, "displayValue": "110"},
                    {"name": "pointDifferential", "value": 10, "displayValue": "+10"},
                    {"name": "streak", "value": -1, "displayValue": "L1"}
                  ]}
                ]}},
                {"abbreviation": "NL", "standings": {"entries": [{"team": {"id": "15", "displayName": "Atlanta Braves", "abbreviation": "ATL"}, "stats": [
                    {"name": "playoffSeed", "value": 1, "displayValue": "1"},
                    {"name": "wins", "value": 20, "displayValue": "20"},
                    {"name": "losses", "value": 9, "displayValue": "9"},
                    {"name": "winPercent", "value": 0.69, "displayValue": ".690"},
                    {"name": "gamesBehind", "value": 0, "displayValue": "-"},
                    {"name": "pointsFor", "value": 166, "displayValue": "166"},
                    {"name": "pointsAgainst", "value": 101, "displayValue": "101"},
                    {"name": "pointDifferential", "value": 65, "displayValue": "+65"},
                    {"name": "streak", "value": 1, "displayValue": "W1"}
                ]}]}}
            ]
        }))
        .unwrap();
        let standings = espn_mlb_standings(data);
        let al_east = standings
            .divisions
            .iter()
            .find(|division| division.league == "AL" && division.division == "East")
            .unwrap();
        let nl_east = standings
            .divisions
            .iter()
            .find(|division| division.league == "NL" && division.division == "East")
            .unwrap();
        assert_eq!(al_east.teams[0].team_name, "New York Yankees");
        assert_eq!(al_east.teams[0].team_tricode, "NYY");
        assert_eq!(al_east.teams[0].playoff_rank, 1);
        assert_eq!(al_east.teams[0].games_back, "-");
        assert_eq!(al_east.teams[1].playoff_rank, 1);
        assert_eq!(al_east.teams[1].games_back, "-");
        assert_eq!(al_east.teams[2].playoff_rank, 3);
        assert_eq!(al_east.teams[2].games_back, "2");
        assert_eq!(nl_east.teams[0].team_name, "Atlanta Braves");
        assert_eq!(nl_east.teams[0].run_diff, "+65");
    }

    #[test]
    fn espn_nfl_standings_conversion_groups_by_division() {
        let data: EspnStandingsDto = serde_json::from_value(serde_json::json!({
            "children": [
                {"abbreviation": "AFC", "standings": {"entries": [
                    {"team": {"id": "2", "displayName": "Buffalo Bills", "abbreviation": "BUF"}, "stats": [
                        {"name": "playoffSeed", "value": 2, "displayValue": "2"},
                        {"name": "wins", "value": 13, "displayValue": "13"},
                        {"name": "losses", "value": 4, "displayValue": "4"},
                        {"name": "ties", "value": 0, "displayValue": "0"},
                        {"name": "winPercent", "value": 0.765, "displayValue": ".765"},
                        {"name": "gamesBehind", "value": 0, "displayValue": "-"},
                        {"name": "pointsFor", "value": 474, "displayValue": "474"},
                        {"name": "pointsAgainst", "value": 336, "displayValue": "336"},
                        {"name": "pointDifferential", "value": 138, "displayValue": "+138"},
                        {"name": "Home", "displayValue": "7-2"},
                        {"name": "Road", "displayValue": "6-2"},
                        {"name": "vs. Div.", "displayValue": "5-1"},
                        {"name": "vs. Conf.", "displayValue": "10-2"},
                        {"name": "streak", "displayValue": "W8"}
                    ]}
                ]}},
                {"abbreviation": "NFC", "standings": {"entries": [
                    {"team": {"id": "21", "displayName": "Philadelphia Eagles", "abbreviation": "PHI"}, "stats": [
                        {"name": "playoffSeed", "value": 1, "displayValue": "1"},
                        {"name": "wins", "value": 14, "displayValue": "14"},
                        {"name": "losses", "value": 3, "displayValue": "3"},
                        {"name": "ties", "value": 0, "displayValue": "0"},
                        {"name": "winPercent", "value": 0.824, "displayValue": ".824"},
                        {"name": "gamesBehind", "value": 0, "displayValue": "-"},
                        {"name": "pointsFor", "value": 490, "displayValue": "490"},
                        {"name": "pointsAgainst", "value": 320, "displayValue": "320"},
                        {"name": "pointDifferential", "value": 170, "displayValue": "+170"},
                        {"name": "Home", "displayValue": "6-3"},
                        {"name": "Road", "displayValue": "8-0"},
                        {"name": "vs. Div.", "displayValue": "5-1"},
                        {"name": "vs. Conf.", "displayValue": "9-3"},
                        {"name": "streak", "displayValue": "W3"}
                    ]}
                ]}}
            ]
        }))
        .unwrap();
        let standings = espn_nfl_standings(data);
        let afc_east = standings
            .divisions
            .iter()
            .find(|division| division.conference == "AFC" && division.division == "East")
            .unwrap();
        let nfc_east = standings
            .divisions
            .iter()
            .find(|division| division.conference == "NFC" && division.division == "East")
            .unwrap();
        assert_eq!(afc_east.teams[0].team_name, "Buffalo Bills");
        assert_eq!(afc_east.teams[0].team_tricode, "BUF");
        assert_eq!(afc_east.teams[0].conference_record, "10-2");
        assert_eq!(nfc_east.teams[0].team_name, "Philadelphia Eagles");
        assert_eq!(nfc_east.teams[0].point_diff, "+170");
    }
}
