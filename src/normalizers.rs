use std::collections::{HashMap, HashSet};

use chrono::Datelike;
use serde_json::Value;

use crate::{
    clients::{
        EspnPlayerGamelogDto, EspnScoreboardDto, EspnStandingsDto, EspnSummaryDto,
        NbaTodaysScoreboardDto,
    },
    error::AppError,
    leagues::{LeagueId, ScheduleBucket},
    models::{
        BoxScore, BoxScoreTeam, BracketMatch, BracketRound, BracketSlot, BracketTable, Game,
        Leaders, MlbBoxScore, MlbBoxScoreTeam, MlbStandingsDivision, MlbStandingsTable,
        MlbStandingsTeam, NflBoxScore, NflBoxScoreTeam, NflStandingsDivision, NflStandingsTable,
        NflStandingsTeam, NhlBoxScore, NhlBoxScoreTeam, NhlStandingsDivision, NhlStandingsTable,
        NhlStandingsTeam, Period, Player, PlayerStatsPage, Scoreboard, SoccerBoxScore,
        SoccerBoxScoreTeam, SoccerEvent, SoccerStandingsGroup, SoccerStandingsTable,
        SoccerStandingsTeam, StandingsTable, StandingsTeam, Statistics, Table, Team,
        TeamStatistics,
    },
};

/// Knockout rounds in bracket display order (left to right), paired with their
/// human-readable labels. ESPN exposes the round via each event's `season.slug`.
const BRACKET_ROUNDS: &[(&str, &str)] = &[
    ("round-of-32", "Round of 32"),
    ("round-of-16", "Round of 16"),
    ("quarterfinals", "Quarterfinals"),
    ("semifinals", "Semifinals"),
    ("final", "Final"),
];

const THIRD_PLACE_ROUND: &str = "3rd-place-match";

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

pub fn espn_wnba_scoreboard(day: &str, data: EspnScoreboardDto) -> Result<Scoreboard, AppError> {
    espn_scoreboard_with(day, data, espn_wnba_competitor_to_team)
}

pub fn espn_mlb_scoreboard(day: &str, data: EspnScoreboardDto) -> Result<Scoreboard, AppError> {
    espn_scoreboard_with(day, data, espn_mlb_competitor_to_team)
}

pub fn espn_nfl_scoreboard(day: &str, data: EspnScoreboardDto) -> Result<Scoreboard, AppError> {
    espn_scoreboard_with(day, data, espn_nfl_competitor_to_team)
}

pub fn espn_nhl_scoreboard(day: &str, data: EspnScoreboardDto) -> Result<Scoreboard, AppError> {
    espn_scoreboard_with(day, data, espn_nhl_competitor_to_team)
}

pub fn espn_soccer_scoreboard(day: &str, data: EspnScoreboardDto) -> Result<Scoreboard, AppError> {
    espn_scoreboard_with(day, data, espn_soccer_competitor_to_team)
}

/// Builds a knockout bracket from an ESPN soccer scoreboard feed covering the
/// knockout stage. `match_numbers` maps an event id to its FIFA match number,
/// which is used to order the matches within each round. When match numbers are
/// available the matches are reordered into a proper bracket tree (each round
/// laid out so a match sits between the two earlier matches that feed it);
/// otherwise the feed/match-number order is used as a best-effort fallback.
pub fn espn_soccer_bracket(
    data: EspnScoreboardDto,
    match_numbers: &HashMap<String, i64>,
) -> Result<BracketTable, AppError> {
    let events: Vec<Value> = data.events;
    let mut by_id: HashMap<String, usize> = HashMap::new();
    let mut round_ids: HashMap<String, Vec<String>> = HashMap::new();
    let mut third_place_id: Option<String> = None;

    for (index, event) in events.iter().enumerate() {
        let Some(id) = str_at(event, &["id"]) else {
            continue;
        };
        let slug = str_at(event, &["season", "slug"]).unwrap_or_default();
        by_id.insert(id.clone(), index);
        if BRACKET_ROUNDS.iter().any(|(round, _)| *round == slug) {
            round_ids.entry(slug).or_default().push(id);
        } else if slug == THIRD_PLACE_ROUND {
            third_place_id = Some(id);
        }
    }

    // Within each round, order by FIFA match number (falling back to kickoff
    // time then id) and record each match's 1-based slot for resolving feeders.
    let mut slot_lookup: HashMap<(String, i64), String> = HashMap::new();
    for (round, ids) in round_ids.iter_mut() {
        ids.sort_by(|a, b| {
            bracket_sort_key(match_numbers, &events, &by_id, a).cmp(&bracket_sort_key(
                match_numbers,
                &events,
                &by_id,
                b,
            ))
        });
        for (slot, id) in ids.iter().enumerate() {
            slot_lookup.insert((round.clone(), slot as i64 + 1), id.clone());
        }
    }

    if let Some(tree_order) = bracket_tree_order(&events, &by_id, &round_ids, &slot_lookup) {
        round_ids = tree_order;
    }

    let mut rounds = Vec::new();
    for (round, label) in BRACKET_ROUNDS {
        let Some(ids) = round_ids.get(*round) else {
            continue;
        };
        if ids.is_empty() {
            continue;
        }
        let matches = ids
            .iter()
            .filter_map(|id| by_id.get(id).map(|&i| bracket_match(&events[i])))
            .collect::<Vec<_>>();
        rounds.push(BracketRound {
            name: (*label).to_string(),
            matches,
        });
    }

    let third_place =
        third_place_id.and_then(|id| by_id.get(&id).map(|&i| bracket_match(&events[i])));

    Ok(BracketTable {
        rounds,
        third_place,
    })
}

fn bracket_sort_key(
    match_numbers: &HashMap<String, i64>,
    events: &[Value],
    by_id: &HashMap<String, usize>,
    id: &str,
) -> (i64, String, String) {
    let match_number = match_numbers.get(id).copied().unwrap_or(i64::MAX);
    let date = by_id
        .get(id)
        .and_then(|&i| str_at(&events[i], &["date"]))
        .unwrap_or_default();
    (match_number, date, id.to_string())
}

/// Re-orders each knockout round into bracket-tree order by walking from the
/// final down through the feeder references encoded in each match's placeholder
/// team names (e.g. "Round of 32 3 Winner"). Returns `None` if the references do
/// not form a complete, consistent tree, so the caller can keep the simpler
/// match-number ordering.
fn bracket_tree_order(
    events: &[Value],
    by_id: &HashMap<String, usize>,
    round_ids: &HashMap<String, Vec<String>>,
    slot_lookup: &HashMap<(String, i64), String>,
) -> Option<HashMap<String, Vec<String>>> {
    let chain = ["semifinals", "quarterfinals", "round-of-16", "round-of-32"];
    let final_id = slot_lookup.get(&("final".to_string(), 1))?.clone();
    let mut ordered: HashMap<String, Vec<String>> = HashMap::new();
    ordered.insert("final".to_string(), vec![final_id.clone()]);
    let mut current = vec![final_id];

    for child_round in chain {
        let mut winner_lookup: HashMap<i64, String> = HashMap::new();
        for id in round_ids.get(child_round)? {
            let event = &events[*by_id.get(id)?];
            for competitor in bracket_competitors(event)? {
                if !bool_at(&competitor, &["winner"]) {
                    continue;
                }
                let team_id = str_at(&competitor, &["team", "id"])
                    .map(|s| i64_from_str(&s))
                    .filter(|id| *id != 0)?;
                winner_lookup.insert(team_id, id.clone());
            }
        }

        let mut next = Vec::new();
        for parent_id in &current {
            let event = &events[*by_id.get(parent_id)?];
            for competitor in bracket_competitors(event)? {
                let name = str_at(&competitor, &["team", "displayName"]).unwrap_or_default();
                if let Some((feeder_round, slot)) = parse_bracket_feeder(&name) {
                    if feeder_round != child_round {
                        return None;
                    }
                    next.push(slot_lookup.get(&(child_round.to_string(), slot))?.clone());
                    continue;
                }
                let team_id = str_at(&competitor, &["team", "id"])
                    .map(|s| i64_from_str(&s))
                    .filter(|id| *id != 0)?;
                next.push(winner_lookup.get(&team_id)?.clone());
            }
        }
        let expected: HashSet<&String> = round_ids.get(child_round)?.iter().collect();
        let actual: HashSet<&String> = next.iter().collect();
        if next.len() != expected.len() || actual != expected {
            return None;
        }
        ordered.insert(child_round.to_string(), next.clone());
        current = next;
    }

    Some(ordered)
}

/// Parses a placeholder team name like "Round of 32 3 Winner" into the round it
/// feeds from and the source match's 1-based slot within that round.
fn parse_bracket_feeder(name: &str) -> Option<(&'static str, i64)> {
    const FEEDERS: &[(&str, &str)] = &[
        ("Round of 32 ", "round-of-32"),
        ("Round of 16 ", "round-of-16"),
        ("Quarterfinal ", "quarterfinals"),
        ("Semifinal ", "semifinals"),
    ];
    for (prefix, round) in FEEDERS {
        if let Some(rest) = name.strip_prefix(prefix)
            && let Some(slot) = rest.strip_suffix(" Winner")
            && let Ok(slot) = slot.trim().parse::<i64>()
        {
            return Some((round, slot));
        }
    }
    None
}

fn bracket_competitors(event: &Value) -> Option<[Value; 2]> {
    let comp = event.pointer("/competitions/0")?;
    let competitors = array_at(comp, &["competitors"]);
    let home = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("home"))?
        .clone();
    let away = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("away"))?
        .clone();
    Some([home, away])
}

fn bracket_match(event: &Value) -> BracketMatch {
    let comp = event.pointer("/competitions/0").unwrap_or(&Value::Null);
    let status = comp.get("status").unwrap_or(&Value::Null);
    let competitors = array_at(comp, &["competitors"]);
    let home = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("home"))
        .cloned()
        .unwrap_or(Value::Null);
    let away = competitors
        .iter()
        .find(|c| str_at(c, &["homeAway"]).as_deref() == Some("away"))
        .cloned()
        .unwrap_or(Value::Null);
    BracketMatch {
        game_id: str_at(event, &["id"]).unwrap_or_default(),
        game_status: espn_status_to_game_status(status),
        game_status_text: str_at(status, &["type", "shortDetail"])
            .or_else(|| str_at(status, &["type", "description"]))
            .unwrap_or_default(),
        game_time_utc: str_at(event, &["date"]).unwrap_or_default(),
        home: bracket_slot(&home),
        away: bracket_slot(&away),
    }
}

fn bracket_slot(competitor: &Value) -> BracketSlot {
    BracketSlot {
        team_id: str_at(competitor, &["team", "id"])
            .map(|s| i64_from_str(&s))
            .unwrap_or(0),
        name: str_at(competitor, &["team", "displayName"])
            .or_else(|| str_at(competitor, &["team", "name"]))
            .unwrap_or_default(),
        short_name: str_at(competitor, &["team", "shortDisplayName"])
            .or_else(|| str_at(competitor, &["team", "abbreviation"]))
            .unwrap_or_default(),
        team_tricode: str_at(competitor, &["team", "abbreviation"]).unwrap_or_default(),
        logo: str_at(competitor, &["team", "logo"]).unwrap_or_default(),
        score: str_at(competitor, &["score"]).unwrap_or_default(),
        shootout_score: str_at(competitor, &["shootoutScore"]).unwrap_or_default(),
        winner: bool_at(competitor, &["winner"]),
        placeholder: !bool_at(competitor, &["team", "isActive"]),
    }
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
    espn_basketball_summary_with(data, espn_competitor_to_team)
}

fn espn_basketball_summary_with(
    data: EspnSummaryDto,
    team_fn: fn(&Value, &Value) -> Team,
) -> Result<BoxScore, AppError> {
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
    Ok(BoxScore {
        game_id: str_at(&data.header, &["id"]).unwrap_or_default(),
        game_status: espn_status_to_game_status(status),
        home_team: summary_team(&data.boxscore, home_comp, team_fn),
        away_team: summary_team(&data.boxscore, away_comp, team_fn),
    })
}

pub fn espn_wnba_summary(data: EspnSummaryDto) -> Result<BoxScore, AppError> {
    espn_basketball_summary_with(data, espn_wnba_competitor_to_team)
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

pub fn espn_nhl_summary(data: EspnSummaryDto) -> Result<NhlBoxScore, AppError> {
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
    Ok(NhlBoxScore {
        game_id: str_at(&data.header, &["id"]).unwrap_or_default(),
        game_status: espn_status_to_game_status(status),
        home_team: nhl_summary_team(&data.boxscore, home_comp, header_comp),
        away_team: nhl_summary_team(&data.boxscore, away_comp, header_comp),
    })
}

pub fn espn_soccer_summary(data: EspnSummaryDto) -> Result<SoccerBoxScore, AppError> {
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
    Ok(SoccerBoxScore {
        game_id: str_at(&data.header, &["id"]).unwrap_or_default(),
        game_status: espn_status_to_game_status(status),
        home_team: soccer_summary_team(&data.boxscore, home_comp),
        away_team: soccer_summary_team(&data.boxscore, away_comp),
        events: soccer_events(&data.key_events, &competitors),
    })
}

pub fn espn_standings(data: EspnStandingsDto) -> StandingsTable {
    espn_basketball_standings_with(data, true)
}

fn espn_basketball_standings_with(data: EspnStandingsDto, use_nba_mapping: bool) -> StandingsTable {
    let mut east = Vec::new();
    let mut west = Vec::new();
    for group in data.children {
        let name = str_at(&group, &["name"]).unwrap_or_default();
        let abbr = str_at(&group, &["abbreviation"]).unwrap_or_default();
        let is_west = name.to_lowercase().contains("west")
            || abbr.to_lowercase().contains("west")
            || abbr.eq_ignore_ascii_case("w");
        let conference = if is_west { "West" } else { "East" };
        for entry in array_at(&group, &["standings", "entries"]) {
            let abbr = str_at(&entry, &["team", "abbreviation"]).unwrap_or_default();
            let fallback_id = i64_from_str(&str_at(&entry, &["team", "id"]).unwrap_or_default());
            let mapped = use_nba_mapping.then(|| team_mapping(&abbr)).flatten();
            let (_, id, _, name) = mapped.unwrap_or((abbr.as_str(), fallback_id, "", ""));
            let stats = array_at(&entry, &["stats"]);
            let row = StandingsTeam {
                team_id: id,
                team_name: if name.is_empty() {
                    str_at(&entry, &["team", "name"]).unwrap_or_default()
                } else {
                    name.to_string()
                },
                team_tricode: abbr,
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

pub fn espn_wnba_standings(data: EspnStandingsDto) -> StandingsTable {
    espn_basketball_standings_with(data, false)
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
            win_pct_value(right)
                .partial_cmp(&win_pct_value(left))
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

pub fn espn_nhl_standings(data: EspnStandingsDto) -> NhlStandingsTable {
    let mut divisions = nhl_empty_divisions();
    for group in data.children {
        collect_nhl_standings(&group, None, None, &mut divisions);
    }
    for division in &mut divisions {
        division.teams.sort_by(|left, right| {
            right
                .points
                .cmp(&left.points)
                .then_with(|| right.wins.cmp(&left.wins))
                .then_with(|| left.losses.cmp(&right.losses))
                .then_with(|| left.ot_losses.cmp(&right.ot_losses))
        });
        for (idx, team) in division.teams.iter_mut().enumerate() {
            if team.playoff_rank == 0 {
                team.playoff_rank = idx as i64 + 1;
            }
        }
    }
    NhlStandingsTable { divisions }
}

pub fn espn_soccer_standings(data: EspnStandingsDto) -> SoccerStandingsTable {
    SoccerStandingsTable {
        groups: data
            .children
            .into_iter()
            .filter_map(|group| {
                let teams: Vec<SoccerStandingsTeam> = array_at(&group, &["standings", "entries"])
                    .iter()
                    .map(soccer_standings_team)
                    .collect();
                if teams.is_empty() {
                    None
                } else {
                    Some(SoccerStandingsGroup {
                        group: str_at(&group, &["name"])
                            .or_else(|| str_at(&group, &["abbreviation"]))
                            .unwrap_or_else(|| "Group".to_string()),
                        teams,
                    })
                }
            })
            .collect(),
    }
}

pub fn espn_player_gamelog(player_id: &str, data: EspnPlayerGamelogDto) -> PlayerStatsPage {
    let source_labels = data.labels;
    let mut summary_rows = Vec::new();
    let mut game_rows = Vec::new();
    for season in data.season_types {
        let season_name = str_at(&season, &["displayName"]).unwrap_or_else(|| "Season".to_string());
        for summary in array_at(&season, &["summary", "stats"]) {
            let summary_name =
                str_at(&summary, &["displayName"]).unwrap_or_else(|| "Summary".to_string());
            let mut row = vec![format!("{} {}", season_name, summary_name)];
            let stats: Vec<String> = array_at(&summary, &["stats"])
                .iter()
                .map(value_to_string)
                .collect();
            row.extend(ordered_native_stats(&source_labels, &stats));
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
                let stats: Vec<String> = array_at(&event, &["stats"])
                    .iter()
                    .map(value_to_string)
                    .collect();
                row.extend(ordered_native_stats(&source_labels, &stats));
                game_rows.push(row);
            }
        }
    }
    let mut summary_headers = vec!["Split".to_string()];
    summary_headers.extend(source_labels.clone());
    let mut game_headers = ["Date", "Opp", "Result", "Score", "Group"]
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();
    game_headers.extend(source_labels);
    let _ = player_id;
    PlayerStatsPage {
        tables: vec![
            Table {
                name: "Summary".to_string(),
                headers: summary_headers,
                rows: summary_rows,
                first_column_links: Vec::new(),
            },
            Table {
                name: "Game Log".to_string(),
                headers: game_headers,
                rows: game_rows,
                first_column_links: Vec::new(),
            },
        ],
    }
}

fn ordered_native_stats(labels: &[String], stats: &[String]) -> Vec<String> {
    labels
        .iter()
        .enumerate()
        .map(|(index, _)| stats.get(index).cloned().unwrap_or_default())
        .collect()
}

#[derive(Debug, Clone)]
pub struct RosterAthlete {
    pub id: String,
    pub name: String,
    pub position: String,
}

pub struct TeamScheduleResult {
    pub team_id: i64,
    pub team_name: String,
    pub team_tricode: String,
    pub record: String,
    pub games: Table,
}

/// ESPN buckets its scoreboard by US Eastern date; approximate with UTC-5 so the
/// box-score link lands on the day the game appears under.
fn eastern_game_day(date_utc: &str) -> String {
    for fmt in [
        "%Y-%m-%dT%H:%MZ",
        "%Y-%m-%dT%H:%M:%SZ",
        "%Y-%m-%dT%H:%M:%S%.fZ",
    ] {
        if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(date_utc, fmt) {
            return (naive - chrono::Duration::hours(5)).date().to_string();
        }
    }
    date_utc.chars().take(10).collect()
}

/// Season start year used by the ESPN core statistics API, derived from the
/// earliest game on the team schedule (handles both calendar-year and
/// fall-to-spring sports without per-league rules).
pub fn team_schedule_season_year(events: &[Value]) -> i64 {
    events
        .iter()
        .filter_map(|event| str_at(event, &["date"]))
        .filter_map(|date| date.get(0..4).and_then(|year| year.parse::<i64>().ok()))
        .min()
        .unwrap_or_else(|| i64::from(chrono::Utc::now().date_naive().year()))
}

pub fn roster_athletes(athletes: &[Value]) -> Vec<RosterAthlete> {
    // Basketball/soccer rosters list athletes flat; MLB/NFL/NHL group them by
    // position under an `items` array. Flatten both shapes uniformly.
    fn collect(out: &mut Vec<RosterAthlete>, athlete: &Value) {
        if let Some(items) = athlete.get("items").and_then(Value::as_array) {
            for item in items {
                collect(out, item);
            }
            return;
        }
        let Some(id) = str_at(athlete, &["id"]) else {
            return;
        };
        let name = str_at(athlete, &["displayName"])
            .or_else(|| str_at(athlete, &["fullName"]))
            .unwrap_or_default();
        let position = str_at(athlete, &["position", "abbreviation"])
            .or_else(|| str_at(athlete, &["position", "name"]))
            .unwrap_or_default();
        out.push(RosterAthlete { id, name, position });
    }

    let mut out = Vec::new();
    for athlete in athletes {
        collect(&mut out, athlete);
    }
    out
}

/// A single game on a team's schedule, normalized so both the recent-results
/// and upcoming-schedule tables can share extraction logic.
struct ScheduleEvent {
    day: String,
    matchup: String,
    link: String,
    completed: bool,
    won: bool,
    score: String,
    date: String,
}

fn schedule_event(
    route_base: &str,
    bucket: ScheduleBucket,
    team_id: i64,
    event: &Value,
) -> ScheduleEvent {
    let event_id = str_at(event, &["id"]).unwrap_or_default();
    let date = str_at(event, &["date"]).unwrap_or_default();
    let day = eastern_game_day(&date);
    let competition = array_at(event, &["competitions"])
        .into_iter()
        .next()
        .unwrap_or(Value::Null);
    let completed = bool_at(&competition, &["status", "type", "completed"]);
    let competitors = array_at(&competition, &["competitors"]);
    let team_side = competitors
        .iter()
        .find(|competitor| i64_at(competitor, &["team", "id"]) == team_id);
    let opponent_side = competitors
        .iter()
        .find(|competitor| i64_at(competitor, &["team", "id"]) != team_id);

    let home_away = team_side
        .map(|side| str_at(side, &["homeAway"]).unwrap_or_default())
        .unwrap_or_default();
    let prefix = if home_away == "away" { "@" } else { "vs" };
    let opponent = opponent_side
        .and_then(|side| str_at(side, &["team", "abbreviation"]))
        .unwrap_or_default();
    let won = team_side
        .map(|side| bool_at(side, &["winner"]))
        .unwrap_or(false);
    let team_score = team_side
        .and_then(|side| str_at(side, &["score", "displayValue"]))
        .unwrap_or_default();
    let opponent_score = opponent_side
        .and_then(|side| str_at(side, &["score", "displayValue"]))
        .unwrap_or_default();

    // Date-bucketed sports link box scores by game day; the NFL buckets its
    // scoreboard by week, so its links must carry the week number instead.
    let link = match bucket {
        ScheduleBucket::Date => format!("{route_base}/scoreboard/{day}/game/{event_id}"),
        ScheduleBucket::Week => {
            let week = i64_at(event, &["week", "number"]);
            format!("{route_base}/scoreboard/{week}/game/{event_id}")
        }
    };

    ScheduleEvent {
        day,
        matchup: format!("{prefix} {opponent}"),
        link,
        completed,
        won,
        score: format!("{team_score}-{opponent_score}"),
        date,
    }
}

pub fn espn_team_schedule(
    route_base: &str,
    bucket: ScheduleBucket,
    team: &Value,
    events: &[Value],
) -> TeamScheduleResult {
    let team_id = i64_at(team, &["id"]);
    let team_name = str_at(team, &["displayName"]).unwrap_or_default();
    let team_tricode = str_at(team, &["abbreviation"]).unwrap_or_default();
    let record = str_at(team, &["recordSummary"]).unwrap_or_default();

    let mut completed: Vec<ScheduleEvent> = events
        .iter()
        .map(|event| schedule_event(route_base, bucket, team_id, event))
        .filter(|event| event.completed)
        .collect();
    completed.sort_by(|a, b| b.date.cmp(&a.date));
    completed.truncate(10);

    let mut rows = Vec::new();
    let mut first_column_links = Vec::new();
    for event in completed {
        rows.push(vec![
            event.day,
            event.matchup,
            if event.won {
                "W".to_string()
            } else {
                "L".to_string()
            },
            event.score,
        ]);
        first_column_links.push(event.link);
    }

    TeamScheduleResult {
        team_id,
        team_name,
        team_tricode,
        record,
        games: Table {
            name: "Last 10 Games".to_string(),
            headers: ["Date", "Opp", "Result", "Score"]
                .iter()
                .map(|header| header.to_string())
                .collect(),
            rows,
            first_column_links,
        },
    }
}

/// The next (up to five) games that have not yet been completed, ordered
/// soonest first, for the team page's upcoming-schedule section.
pub fn espn_upcoming_games(
    route_base: &str,
    bucket: ScheduleBucket,
    team: &Value,
    events: &[Value],
) -> Table {
    let team_id = i64_at(team, &["id"]);
    let mut upcoming: Vec<ScheduleEvent> = events
        .iter()
        .map(|event| schedule_event(route_base, bucket, team_id, event))
        .filter(|event| !event.completed)
        .collect();
    upcoming.sort_by(|a, b| a.date.cmp(&b.date));
    upcoming.truncate(5);

    let mut rows = Vec::new();
    let mut first_column_links = Vec::new();
    for event in upcoming {
        rows.push(vec![event.day, event.matchup]);
        first_column_links.push(event.link);
    }

    Table {
        name: "Next Games".to_string(),
        headers: ["Date", "Opp"].iter().map(|h| h.to_string()).collect(),
        rows,
        first_column_links,
    }
}

/// (column label, ESPN category name or "" to search all, ESPN stat name)
type StatColumn = (&'static str, &'static str, &'static str);

const BASKETBALL_STATS: &[StatColumn] = &[
    ("GP", "", "gamesPlayed"),
    ("MIN", "", "avgMinutes"),
    ("PTS", "", "avgPoints"),
    ("REB", "", "avgRebounds"),
    ("AST", "", "avgAssists"),
    ("STL", "", "avgSteals"),
    ("BLK", "", "avgBlocks"),
    ("FG%", "", "fieldGoalPct"),
    ("3P%", "", "threePointPct"),
    ("FT%", "", "freeThrowPct"),
    ("TO", "", "avgTurnovers"),
];

const MLB_STATS: &[StatColumn] = &[
    ("GP", "batting", "gamesPlayed"),
    ("AVG", "batting", "avg"),
    ("HR", "batting", "homeRuns"),
    ("RBI", "batting", "RBIs"),
    ("H", "batting", "hits"),
    ("R", "batting", "runs"),
    ("SB", "batting", "stolenBases"),
    ("OPS", "batting", "OPS"),
];

const NHL_STATS: &[StatColumn] = &[
    ("GP", "general", "games"),
    ("G", "offensive", "goals"),
    ("A", "offensive", "assists"),
    ("PTS", "offensive", "points"),
    ("+/-", "general", "plusMinus"),
    ("TOI/G", "general", "timeOnIcePerGame"),
];

const NFL_STATS: &[StatColumn] = &[
    ("GP", "general", "gamesPlayed"),
    ("CMP", "passing", "completions"),
    ("PYDS", "passing", "passingYards"),
    ("PTD", "passing", "passingTouchdowns"),
    ("INT", "passing", "interceptions"),
    ("RYDS", "rushing", "rushingYards"),
    ("RTD", "rushing", "rushingTouchdowns"),
    ("REC", "receiving", "receptions"),
    ("RcYDS", "receiving", "receivingYards"),
    ("RcTD", "receiving", "receivingTouchdowns"),
];

/// Sport-appropriate player-stat columns for a league's team page. Soccer
/// leagues have no athlete season stats and return an empty slice.
pub fn team_stat_columns(league: LeagueId) -> &'static [StatColumn] {
    match league {
        LeagueId::Nba | LeagueId::Wnba => BASKETBALL_STATS,
        LeagueId::Mlb => MLB_STATS,
        LeagueId::Nhl => NHL_STATS,
        LeagueId::Nfl => NFL_STATS,
        LeagueId::WorldCup | LeagueId::Nwsl => &[],
    }
}

fn core_stat(stats: &Value, category: &str, name: &str) -> String {
    for cat in array_at(stats, &["splits", "categories"]) {
        if !category.is_empty() && str_at(&cat, &["name"]).as_deref() != Some(category) {
            continue;
        }
        for stat in array_at(&cat, &["stats"]) {
            if str_at(&stat, &["name"]).as_deref() == Some(name) {
                return str_at(&stat, &["displayValue"]).unwrap_or_default();
            }
        }
    }
    String::new()
}

pub fn espn_team_player_stats(
    route_base: &str,
    players: &[(RosterAthlete, Option<Value>)],
    columns: &[StatColumn],
) -> Table {
    let mut headers = vec!["Player".to_string(), "Pos".to_string()];
    headers.extend(columns.iter().map(|(label, _, _)| label.to_string()));

    let mut rows = Vec::new();
    let mut first_column_links = Vec::new();
    for (athlete, stats) in players {
        let Some(stats) = stats else {
            continue;
        };
        let values: Vec<String> = columns
            .iter()
            .map(|(_, category, name)| core_stat(stats, category, name))
            .collect();
        // Skip players with no meaningful stats (e.g. MLB pitchers in a batting
        // table, or NFL defenders in an offensive table) by ignoring the leading
        // games-played column when deciding whether a row carries data.
        if !values.iter().skip(1).any(|value| !value.is_empty()) {
            continue;
        }
        let mut row = vec![athlete.name.clone(), athlete.position.clone()];
        row.extend(values);
        rows.push(row);
        first_column_links.push(format!("{route_base}/player/{}", athlete.id));
    }

    Table {
        name: "Player Stats".to_string(),
        headers,
        rows,
        first_column_links,
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
                first_column_links: Vec::new(),
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
        shootout_score: None,
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
        shootout_score: None,
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

fn espn_wnba_competitor_to_team(c: &Value, competition: &Value) -> Team {
    let (wins, losses) = array_at(c, &["records"])
        .iter()
        .find(|r| str_at(r, &["type"]).as_deref() == Some("total"))
        .and_then(|r| str_at(r, &["summary"]))
        .map(|s| parse_record(&s))
        .unwrap_or((0, 0));
    let display_record =
        playoff_series_record(c, competition).unwrap_or_else(|| format!("{wins}-{losses}"));
    Team {
        team_id: str_at(c, &["team", "id"])
            .or_else(|| str_at(c, &["id"]))
            .map(|s| i64_from_str(&s))
            .unwrap_or(0),
        team_name: str_at(c, &["team", "name"]).unwrap_or_default(),
        team_city: str_at(c, &["team", "location"]).unwrap_or_default(),
        team_tricode: str_at(c, &["team", "abbreviation"]).unwrap_or_default(),
        wins,
        losses,
        display_record,
        score: str_at(c, &["score"]).map(|s| i64_from_str(&s)).unwrap_or(0),
        shootout_score: None,
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
        shootout_score: None,
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
        shootout_score: None,
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

fn espn_nhl_competitor_to_team(c: &Value, competition: &Value) -> Team {
    let (wins, losses) = total_record_summary(c)
        .map(|s| parse_record(&s))
        .unwrap_or((0, 0));
    let display_record = playoff_series_record(c, competition)
        .or_else(|| str_at(c, &["record"]))
        .unwrap_or_else(|| total_record_summary(c).unwrap_or_else(|| format!("{wins}-{losses}")));
    Team {
        team_id: str_at(c, &["team", "id"])
            .or_else(|| str_at(c, &["id"]))
            .map(|s| i64_from_str(&s))
            .unwrap_or(0),
        team_name: str_at(c, &["team", "name"]).unwrap_or_default(),
        team_city: str_at(c, &["team", "location"]).unwrap_or_default(),
        team_tricode: str_at(c, &["team", "abbreviation"]).unwrap_or_default(),
        wins,
        losses,
        display_record,
        score: str_at(c, &["score"]).map(|s| i64_from_str(&s)).unwrap_or(0),
        shootout_score: None,
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

fn espn_soccer_competitor_to_team(c: &Value, _competition: &Value) -> Team {
    let display_record = total_record_summary(c).unwrap_or_default();
    let (wins, losses) = if display_record.is_empty() {
        (0, 0)
    } else {
        parse_record(&display_record)
    };
    let shootout_score = str_at(c, &["shootoutScore"])
        .filter(|s| !s.is_empty())
        .map(|s| i64_from_str(&s));
    Team {
        team_id: str_at(c, &["team", "id"])
            .or_else(|| str_at(c, &["id"]))
            .map(|s| i64_from_str(&s))
            .unwrap_or(0),
        team_name: str_at(c, &["team", "name"])
            .or_else(|| str_at(c, &["team", "displayName"]))
            .unwrap_or_default(),
        team_city: str_at(c, &["team", "location"]).unwrap_or_default(),
        team_tricode: str_at(c, &["team", "abbreviation"]).unwrap_or_default(),
        wins,
        losses,
        display_record,
        score: str_at(c, &["score"]).map(|s| i64_from_str(&s)).unwrap_or(0),
        shootout_score,
        hits: 0,
        errors: 0,
        periods: Vec::new(),
    }
}

fn summary_team(
    boxscore: &Value,
    comp: &Value,
    team_fn: fn(&Value, &Value) -> Team,
) -> BoxScoreTeam {
    let abbr = str_at(comp, &["team", "abbreviation"]).unwrap_or_default();
    let team = team_fn(comp, &Value::Null);
    let team_stats = array_at(boxscore, &["teams"])
        .into_iter()
        .find(|t| str_at(t, &["team", "abbreviation"]).as_deref() == Some(abbr.as_str()))
        .map(|t| array_at(&t, &["statistics"]))
        .unwrap_or_default();
    let players = array_at(boxscore, &["players"])
        .into_iter()
        .find(|t| str_at(t, &["team", "abbreviation"]).as_deref() == Some(abbr.as_str()))
        .map(|t| summary_players(&t))
        .unwrap_or_default();
    BoxScoreTeam {
        team: Team {
            score: str_at(comp, &["score"])
                .map(|s| i64_from_str(&s))
                .unwrap_or(0),
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
            ..team
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
        player_stats: player_stat_tables(&player_group, "/nfl/player"),
    }
}

fn nhl_summary_team(boxscore: &Value, comp: &Value, competition: &Value) -> NhlBoxScoreTeam {
    let team = espn_nhl_competitor_to_team(comp, competition);
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
    NhlBoxScoreTeam {
        team,
        team_stats: nfl_team_stats_table(&team_group),
        player_stats: player_stat_tables(&player_group, "/nhl/player"),
    }
}

fn soccer_summary_team(boxscore: &Value, comp: &Value) -> SoccerBoxScoreTeam {
    let team = espn_soccer_competitor_to_team(comp, &Value::Null);
    let team_id = team.team_id.to_string();
    let abbr = team.team_tricode.clone();
    let name = team.team_name.clone();
    let team_group = array_at(boxscore, &["teams"])
        .into_iter()
        .find(|team_box| {
            str_at(team_box, &["team", "id"]).as_deref() == Some(team_id.as_str())
                || str_at(team_box, &["team", "abbreviation"]).as_deref() == Some(abbr.as_str())
                || str_at(team_box, &["team", "displayName"]).as_deref() == Some(name.as_str())
        })
        .unwrap_or(Value::Null);
    SoccerBoxScoreTeam {
        team,
        team_stats: soccer_team_stats_table(&team_group),
    }
}

fn soccer_team_stats_table(group: &Value) -> Table {
    let rows = array_at(group, &["statistics"])
        .iter()
        .map(|stat| {
            vec![
                str_at(stat, &["label"])
                    .or_else(|| str_at(stat, &["displayName"]))
                    .or_else(|| str_at(stat, &["name"]))
                    .unwrap_or_default(),
                str_at(stat, &["displayValue"])
                    .or_else(|| stat.get("value").map(value_to_string))
                    .unwrap_or_default(),
            ]
        })
        .collect();
    Table {
        name: "Team Stats".to_string(),
        headers: vec!["Stat".to_string(), "Value".to_string()],
        rows,
        first_column_links: Vec::new(),
    }
}

fn soccer_events(events: &[Value], competitors: &[Value]) -> Vec<SoccerEvent> {
    events
        .iter()
        .filter_map(|event| soccer_event(event, competitors))
        .collect()
}

fn soccer_event(event: &Value, competitors: &[Value]) -> Option<SoccerEvent> {
    let scoring_play = bool_at(event, &["scoringPlay"]);
    let yellow_card = bool_at(event, &["yellowCard"])
        || str_at(event, &["type", "type"]).as_deref() == Some("yellow-card");
    let red_card = bool_at(event, &["redCard"])
        || str_at(event, &["type", "type"]).as_deref() == Some("red-card");
    if !scoring_play && !yellow_card && !red_card {
        return None;
    }

    let participants = array_at(event, &["participants"]);
    let player = participants
        .first()
        .and_then(|participant| str_at(participant, &["athlete", "displayName"]))
        .or_else(|| str_at(event, &["athletesInvolved", "0", "displayName"]))
        .unwrap_or_default();
    let assist = participants
        .get(1)
        .and_then(|participant| str_at(participant, &["athlete", "displayName"]))
        .unwrap_or_default();
    let kind = if scoring_play {
        "Goal"
    } else if red_card {
        "Red Card"
    } else {
        "Yellow Card"
    };
    Some(SoccerEvent {
        minute: str_at(event, &["clock", "displayValue"]).unwrap_or_default(),
        team_tricode: soccer_event_team_tricode(event, competitors),
        kind: kind.to_string(),
        player,
        assist,
        note: soccer_event_note(event),
    })
}

fn soccer_event_team_tricode(event: &Value, competitors: &[Value]) -> String {
    let team_id = str_at(event, &["team", "id"]).unwrap_or_default();
    competitors
        .iter()
        .find(|competitor| {
            str_at(competitor, &["team", "id"]).as_deref() == Some(team_id.as_str())
                || str_at(competitor, &["id"]).as_deref() == Some(team_id.as_str())
        })
        .and_then(|competitor| str_at(competitor, &["team", "abbreviation"]))
        .or_else(|| str_at(event, &["team", "abbreviation"]))
        .unwrap_or_default()
}

fn soccer_event_note(event: &Value) -> String {
    let mut notes = Vec::new();
    if bool_at(event, &["penaltyKick"]) {
        notes.push("Penalty");
    }
    if bool_at(event, &["ownGoal"]) {
        notes.push("Own goal");
    }
    notes.join(", ")
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
        first_column_links: Vec::new(),
    }
}

fn player_stat_tables(group: &Value, player_base_path: &str) -> Vec<Table> {
    array_at(group, &["statistics"])
        .iter()
        .map(|stat_group| {
            let mut headers = vec!["Name".to_string()];
            headers.extend(
                array_at(stat_group, &["labels"])
                    .iter()
                    .map(value_to_string),
            );
            let athletes = array_at(stat_group, &["athletes"]);
            let mut first_column_links = Vec::new();
            let mut rows: Vec<Vec<String>> = athletes
                .iter()
                .map(|athlete| {
                    first_column_links.push(player_link(player_base_path, athlete));
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
                first_column_links.push(String::new());
            }
            Table {
                name: str_at(stat_group, &["text"])
                    .or_else(|| str_at(stat_group, &["name"]))
                    .unwrap_or_else(|| "Stats".to_string()),
                headers,
                rows,
                first_column_links,
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
            first_column_links: Vec::new(),
        };
    };
    let mut headers = vec!["Name".to_string()];
    headers.extend(
        array_at(stat_group, &["labels"])
            .iter()
            .map(value_to_string),
    );
    let athletes = array_at(stat_group, &["athletes"]);
    let mut first_column_links = Vec::new();
    let rows = athletes
        .iter()
        .map(|athlete| {
            first_column_links.push(player_link("/mlb/player", athlete));
            let mut row = vec![str_at(athlete, &["athlete", "displayName"]).unwrap_or_default()];
            row.extend(array_at(athlete, &["stats"]).iter().map(value_to_string));
            row
        })
        .collect();
    Table {
        name: name.to_string(),
        headers,
        rows,
        first_column_links,
    }
}

fn player_link(base_path: &str, athlete: &Value) -> String {
    str_at(athlete, &["athlete", "id"])
        .filter(|id| !id.is_empty())
        .map(|id| format!("{base_path}/{id}"))
        .unwrap_or_default()
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
    let series = playoff_series(competition)?;
    let team_id = str_at(competitor, &["team", "id"]).or_else(|| str_at(competitor, &["id"]))?;
    let team_wins = array_at(series, &["competitors"])
        .iter()
        .find(|series_team| str_at(series_team, &["id"]).as_deref() == Some(team_id.as_str()))
        .map(|series_team| i64_at(series_team, &["wins"]))?;
    let opponent_wins = array_at(series, &["competitors"])
        .iter()
        .find(|series_team| str_at(series_team, &["id"]).as_deref() != Some(team_id.as_str()))
        .map(|series_team| i64_at(series_team, &["wins"]))
        .unwrap_or(0);
    Some(format!("{team_wins}-{opponent_wins}"))
}

fn playoff_series(competition: &Value) -> Option<&Value> {
    match competition.get("series")? {
        Value::Array(series) => series
            .iter()
            .find(|entry| str_at(entry, &["type"]).as_deref() == Some("playoff")),
        series if str_at(series, &["type"]).as_deref() == Some("playoff") => Some(series),
        _ => None,
    }
}

fn espn_status_to_game_status(status: &Value) -> i64 {
    if bool_at(status, &["type", "completed"]) {
        return 3;
    }
    if str_at(status, &["type", "state"]).as_deref() == Some("in") {
        return 2;
    }
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

fn nhl_empty_divisions() -> Vec<NhlStandingsDivision> {
    [
        ("Eastern", "Atlantic"),
        ("Eastern", "Metropolitan"),
        ("Western", "Central"),
        ("Western", "Pacific"),
    ]
    .iter()
    .map(|(conference, division)| NhlStandingsDivision {
        conference: (*conference).to_string(),
        division: (*division).to_string(),
        teams: Vec::new(),
    })
    .collect()
}

fn collect_nhl_standings(
    group: &Value,
    conference_hint: Option<&str>,
    division_hint: Option<&str>,
    divisions: &mut [NhlStandingsDivision],
) {
    let group_name = str_at(group, &["name"]).unwrap_or_default();
    let group_abbr = str_at(group, &["abbreviation"]).unwrap_or_default();
    let conference = if group_abbr == "East" || group_name.contains("Eastern") {
        Some("Eastern")
    } else if group_abbr == "West" || group_name.contains("Western") {
        Some("Western")
    } else {
        conference_hint
    };
    let division = nhl_division_name(&group_name).or(division_hint);
    for child in array_at(group, &["children"]) {
        collect_nhl_standings(&child, conference, division, divisions);
    }
    for entry in array_at(group, &["standings", "entries"]) {
        let stats = array_at(&entry, &["stats"]);
        let id = str_at(&entry, &["team", "id"])
            .map(|s| i64_from_str(&s))
            .unwrap_or(0);
        let abbr = str_at(&entry, &["team", "abbreviation"]).unwrap_or_default();
        let (fallback_conference, fallback_division) =
            nhl_division(&abbr).unwrap_or(("Eastern", "Atlantic"));
        let conference = conference.unwrap_or(fallback_conference);
        let division = division.unwrap_or(fallback_division);
        let row = NhlStandingsTeam {
            team_id: id,
            team_name: str_at(&entry, &["team", "displayName"])
                .or_else(|| str_at(&entry, &["team", "name"]))
                .unwrap_or_default(),
            team_tricode: abbr,
            conference: conference.to_string(),
            division: division.to_string(),
            playoff_rank: stat_value(&stats, "playoffSeed")
                .or_else(|| stat_value(&stats, "rank"))
                .unwrap_or(0.0) as i64,
            wins: stat_value(&stats, "wins").unwrap_or(0.0) as i64,
            losses: stat_value(&stats, "losses").unwrap_or(0.0) as i64,
            ot_losses: stat_value(&stats, "otLosses")
                .or_else(|| stat_value(&stats, "overtimeLosses"))
                .unwrap_or(0.0) as i64,
            points: stat_value(&stats, "points").unwrap_or(0.0) as i64,
            games_back: stat_display(&stats, "gamesBehind").unwrap_or_default(),
            goals_for: stat_value(&stats, "pointsFor").unwrap_or(0.0) as i64,
            goals_against: stat_value(&stats, "pointsAgainst").unwrap_or(0.0) as i64,
            goal_diff: stat_display(&stats, "pointDifferential")
                .or_else(|| stat_display(&stats, "pointsDiff"))
                .unwrap_or_default(),
            home: stat_display(&stats, "Home").unwrap_or_default(),
            road: stat_display(&stats, "Road").unwrap_or_default(),
            division_record: stat_display(&stats, "vs. Div.").unwrap_or_default(),
            last_ten: stat_display(&stats, "Last Ten Games").unwrap_or_default(),
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

fn soccer_standings_team(entry: &Value) -> SoccerStandingsTeam {
    let stats = array_at(entry, &["stats"]);
    SoccerStandingsTeam {
        team_id: str_at(entry, &["team", "id"])
            .map(|s| i64_from_str(&s))
            .unwrap_or(0),
        team_name: str_at(entry, &["team", "displayName"])
            .or_else(|| str_at(entry, &["team", "name"]))
            .unwrap_or_default(),
        team_tricode: str_at(entry, &["team", "abbreviation"]).unwrap_or_default(),
        rank: stat_num(&stats, "rank"),
        games_played: stat_num(&stats, "gamesPlayed"),
        wins: stat_num(&stats, "wins"),
        draws: stat_num(&stats, "ties"),
        losses: stat_num(&stats, "losses"),
        goals_for: stat_num(&stats, "pointsFor"),
        goals_against: stat_num(&stats, "pointsAgainst"),
        goal_diff: stat_display(&stats, "pointDifferential").unwrap_or_default(),
        points: stat_num(&stats, "points"),
        record: stat_display(&stats, "overall")
            .or_else(|| stat_display(&stats, "total"))
            .unwrap_or_default(),
    }
}

fn nhl_division_name(name: &str) -> Option<&'static str> {
    if name.contains("Atlantic") {
        Some("Atlantic")
    } else if name.contains("Metropolitan") {
        Some("Metropolitan")
    } else if name.contains("Central") {
        Some("Central")
    } else if name.contains("Pacific") {
        Some("Pacific")
    } else {
        None
    }
}

fn nhl_division(abbr: &str) -> Option<(&'static str, &'static str)> {
    Some(match abbr {
        "BOS" | "BUF" | "DET" | "FLA" | "MTL" | "OTT" | "TB" | "TOR" => ("Eastern", "Atlantic"),
        "CAR" | "CBJ" | "NJ" | "NYI" | "NYR" | "PHI" | "PIT" | "WSH" => ("Eastern", "Metropolitan"),
        "ARI" | "CHI" | "COL" | "DAL" | "MIN" | "NSH" | "STL" | "WPG" | "UTA" => {
            ("Western", "Central")
        }
        "ANA" | "CGY" | "EDM" | "LA" | "SEA" | "SJ" | "VAN" | "VGK" => ("Western", "Pacific"),
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

fn total_record_summary(c: &Value) -> Option<String> {
    array_at(c, &["records"])
        .into_iter()
        .chain(array_at(c, &["record"]))
        .find(|r| str_at(r, &["type"]).as_deref() == Some("total"))
        .and_then(|r| str_at(&r, &["summary"]))
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

    fn bracket_competitor(
        home_away: &str,
        team_id: Option<&str>,
        display_name: &str,
        is_active: bool,
        winner: Option<bool>,
    ) -> Value {
        let mut competitor = serde_json::json!({
            "homeAway": home_away,
            "score": "0",
            "team": {
                "displayName": display_name,
                "isActive": is_active,
                "logo": "",
            }
        });
        if let Some(team_id) = team_id {
            competitor["team"]["id"] = serde_json::json!(team_id);
        }
        if let Some(winner) = winner {
            competitor["winner"] = serde_json::json!(winner);
        }
        competitor
    }

    fn bracket_match_event(
        id: &str,
        slug: &str,
        date: &str,
        completed: bool,
        home: Value,
        away: Value,
    ) -> Value {
        serde_json::json!({
            "id": id,
            "date": date,
            "season": {"slug": slug},
            "competitions": [{
                "status": {"type": {"completed": completed, "state": if completed { "post" } else { "pre" }, "shortDetail": if completed { "FT" } else { "TBD" }}},
                "competitors": [home, away]
            }]
        })
    }

    fn synthetic_bracket_data(
        completed_r32_slot_one: bool,
    ) -> (EspnScoreboardDto, HashMap<String, i64>) {
        let mut events = Vec::new();
        let mut match_numbers = HashMap::new();

        let mut push_event = |id: String, match_number: i64, event: Value| {
            match_numbers.insert(id, match_number);
            events.push(event);
        };

        for slot in (1..=16).rev() {
            let id = format!("r32-{slot}");
            let base_id = 1000 + (slot as i64 - 1) * 2;
            let home_id = base_id + 1;
            let away_id = base_id + 2;
            let completed = completed_r32_slot_one && slot == 1;
            let home = bracket_competitor(
                "home",
                Some(&home_id.to_string()),
                &format!("R32 Team {slot} A"),
                true,
                if completed { Some(true) } else { None },
            );
            let away = bracket_competitor(
                "away",
                Some(&away_id.to_string()),
                &format!("R32 Team {slot} B"),
                true,
                if completed { Some(false) } else { None },
            );
            push_event(
                id.clone(),
                slot as i64,
                bracket_match_event(
                    &id,
                    "round-of-32",
                    &format!("2026-06-{slot:02}T16:00Z"),
                    completed,
                    home,
                    away,
                ),
            );
        }

        for slot in (1..=8).rev() {
            let completed = false;
            let home = if completed_r32_slot_one && slot == 1 {
                bracket_competitor("home", Some("1001"), "Team 1001", true, None)
            } else {
                bracket_competitor(
                    "home",
                    None,
                    &format!("Round of 32 {slot} Winner"),
                    false,
                    None,
                )
            };
            let away = bracket_competitor(
                "away",
                None,
                &format!("Round of 32 {} Winner", slot + 8),
                false,
                None,
            );
            let id = format!("r16-{slot}");
            push_event(
                id.clone(),
                100 + slot as i64,
                bracket_match_event(
                    &id,
                    "round-of-16",
                    &format!("2026-06-{slot:02}T18:00Z"),
                    completed,
                    home,
                    away,
                ),
            );
        }

        for slot in (1..=4).rev() {
            let home = bracket_competitor(
                "home",
                None,
                &format!("Round of 16 {slot} Winner"),
                false,
                None,
            );
            let away = bracket_competitor(
                "away",
                None,
                &format!("Round of 16 {} Winner", slot + 4),
                false,
                None,
            );
            let id = format!("qf-{slot}");
            push_event(
                id.clone(),
                200 + slot as i64,
                bracket_match_event(
                    &id,
                    "quarterfinals",
                    &format!("2026-07-{slot:02}T18:00Z"),
                    false,
                    home,
                    away,
                ),
            );
        }

        for slot in (1..=2).rev() {
            let home = bracket_competitor(
                "home",
                None,
                &format!("Quarterfinal {slot} Winner"),
                false,
                None,
            );
            let away = bracket_competitor(
                "away",
                None,
                &format!("Quarterfinal {} Winner", slot + 2),
                false,
                None,
            );
            let id = format!("sf-{slot}");
            push_event(
                id.clone(),
                300 + slot as i64,
                bracket_match_event(
                    &id,
                    "semifinals",
                    &format!("2026-07-{slot:02}T20:00Z"),
                    false,
                    home,
                    away,
                ),
            );
        }

        let final_id = "final-1";
        push_event(
            final_id.to_string(),
            400,
            bracket_match_event(
                final_id,
                "final",
                "2026-07-30T20:00Z",
                false,
                bracket_competitor("home", None, "Semifinal 1 Winner", false, None),
                bracket_competitor("away", None, "Semifinal 2 Winner", false, None),
            ),
        );

        let data = serde_json::from_value(serde_json::json!({"events": events})).unwrap();
        (data, match_numbers)
    }

    fn round_game_ids(bracket: &BracketTable, round_index: usize) -> Vec<String> {
        bracket.rounds[round_index]
            .matches
            .iter()
            .map(|game| game.game_id.clone())
            .collect()
    }

    #[test]
    fn player_gamelog_conversion_produces_tables() {
        let data: EspnPlayerGamelogDto = serde_json::from_value(serde_json::json!({
            "labels": ["MIN", "FG", "PTS", "REB", "AST", "TO"],
            "events": {"1": {"gameDate": "2026-01-01T00:00:00Z", "atVs": "@", "opponent": {"abbreviation": "BOS"}, "gameResult": "W", "score": "100-90"}},
            "seasonTypes": [{"displayName": "Season", "summary": {"stats": [
                {"displayName": "Totals", "stats": ["300", "80-160", "200", "50", "40", "20"]},
                {"displayName": "Averages", "stats": ["30", "8-16", "20", "5", "4", "2"]}
            ]}, "categories": [{"displayName": "january", "events": [{"eventId": "1", "stats": ["31", "9-17", "22", "6", "5", "3"]}]}]}]
        })).unwrap();
        let page = espn_player_gamelog("1", data);
        assert_eq!(page.tables.len(), 2);
        assert_eq!(page.tables[0].name, "Summary");
        assert_eq!(
            page.tables[0].headers,
            ["Split", "MIN", "FG", "PTS", "REB", "AST", "TO"]
        );
        assert_eq!(page.tables[0].rows.len(), 2);
        assert_eq!(
            page.tables[0].rows[0][1..7],
            ["300", "80-160", "200", "50", "40", "20"]
        );
        assert_eq!(
            page.tables[0].rows[1][1..7],
            ["30", "8-16", "20", "5", "4", "2"]
        );
        assert_eq!(
            page.tables[1].headers[5..11],
            ["MIN", "FG", "PTS", "REB", "AST", "TO"]
        );
        assert_eq!(page.tables[1].rows[0][1], "@ BOS");
        assert_eq!(
            page.tables[1].rows[0][5..11],
            ["31", "9-17", "22", "6", "5", "3"]
        );
    }

    #[test]
    fn player_gamelog_preserves_mlb_native_labels_and_totals() {
        let data: EspnPlayerGamelogDto = serde_json::from_value(serde_json::json!({
            "labels": ["AB", "R", "H", "HR", "RBI", "AVG"],
            "events": {"401815712": {"gameDate": "2026-06-01T00:00:00Z", "atVs": "vs", "opponent": {"abbreviation": "NYM"}, "gameResult": "W", "score": "5-2"}},
            "seasonTypes": [{"displayName": "Regular Season", "summary": {"stats": [
                {"displayName": "Totals", "stats": ["243", "48", "74", "13", "40", ".305"], "type": "total"}
            ]}, "categories": [{"displayName": "june", "events": [{"eventId": "401815712", "stats": ["2", "2", "2", "1", "1", ".305"]}]}]}]
        }))
        .unwrap();

        let page = espn_player_gamelog("39832", data);

        assert_eq!(
            page.tables[0].headers,
            ["Split", "AB", "R", "H", "HR", "RBI", "AVG"]
        );
        assert_eq!(page.tables[0].rows[0][0], "Regular Season Totals");
        assert_eq!(
            page.tables[0].rows[0][1..7],
            ["243", "48", "74", "13", "40", ".305"]
        );
        assert_eq!(
            page.tables[1].rows[0][5..11],
            ["2", "2", "2", "1", "1", ".305"]
        );
    }

    #[test]
    fn player_gamelog_preserves_nhl_native_labels_and_totals() {
        let data: EspnPlayerGamelogDto = serde_json::from_value(serde_json::json!({
            "labels": ["G", "A", "PTS", "+/-", "PIM", "S", "TOI/G"],
            "events": {"401869765": {"gameDate": "2026-04-20T00:00:00Z", "atVs": "@", "opponent": {"abbreviation": "BUF"}, "gameResult": "L", "score": "3-2"}},
            "seasonTypes": [{"displayName": "Postseason", "summary": {"stats": [
                {"displayName": "Totals", "stats": ["3", "4", "7", "-7", "8", "22", "20:57"], "type": "total"}
            ]}, "categories": [{"displayName": "Postseason", "events": [{"eventId": "401869765", "stats": ["1", "0", "1", "-3", "0", "5", "18:35"]}]}]}]
        }))
        .unwrap();

        let page = espn_player_gamelog("3114778", data);

        assert_eq!(
            page.tables[0].headers,
            ["Split", "G", "A", "PTS", "+/-", "PIM", "S", "TOI/G"]
        );
        assert_eq!(
            page.tables[0].rows[0][1..8],
            ["3", "4", "7", "-7", "8", "22", "20:57"]
        );
        assert_eq!(
            page.tables[1].rows[0][5..12],
            ["1", "0", "1", "-3", "0", "5", "18:35"]
        );
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
    fn espn_scoreboard_conversion_treats_halftime_as_live() {
        let data: EspnScoreboardDto = serde_json::from_value(serde_json::json!({
            "events": [{
                "id": "401869401",
                "date": "2026-04-27T23:30:00Z",
                "competitions": [{
                    "status": {
                        "period": 2,
                        "displayClock": "0.0",
                        "type": {
                            "name": "STATUS_HALFTIME",
                            "state": "in",
                            "completed": false,
                            "shortDetail": "Halftime"
                        }
                    },
                    "competitors": [
                        {"homeAway": "away", "id": "16", "score": "51", "team": {"id": "16", "abbreviation": "MIN"}, "records": [{"type": "total", "summary": "3-1"}], "linescores": [{"period": 1, "value": 29}, {"period": 2, "value": 22}]},
                        {"homeAway": "home", "id": "7", "score": "60", "team": {"id": "7", "abbreviation": "DEN"}, "records": [{"type": "total", "summary": "1-3"}], "linescores": [{"period": 1, "value": 34}, {"period": 2, "value": 26}]}
                    ]
                }]
            }]
        }))
        .unwrap();
        let scoreboard = espn_scoreboard("2026-04-27", data).unwrap();

        assert_eq!(scoreboard.games[0].game_status, 2);
        assert_eq!(scoreboard.games[0].game_status_text, "Halftime");
    }

    #[test]
    fn espn_wnba_standings_preserve_upstream_team_identity() {
        let data: EspnStandingsDto = serde_json::from_value(serde_json::json!({
            "children": [{
                "abbreviation": "East",
                "standings": {"entries": [{
                    "team": {"id": "999", "name": "Boston WNBA", "abbreviation": "BOS"},
                    "stats": [
                        {"name": "wins", "value": 11, "displayValue": "11"},
                        {"name": "losses", "value": 7, "displayValue": "7"}
                    ]
                }]}
            }]
        }))
        .unwrap();

        let standings = espn_wnba_standings(data);

        assert_eq!(standings.east[0].team_id, 999);
        assert_eq!(standings.east[0].team_name, "Boston WNBA");
        assert_eq!(standings.east[0].team_tricode, "BOS");
        assert_eq!(standings.east[0].wins, 11);
    }

    #[test]
    fn espn_wnba_standings_splits_short_conference_abbreviations() {
        let data: EspnStandingsDto = serde_json::from_value(serde_json::json!({
            "children": [
                {
                    "name": "Eastern Conference",
                    "abbreviation": "E",
                    "standings": {"entries": [{
                        "team": {"id": "1", "name": "Liberty", "abbreviation": "NY"},
                        "stats": [{"name": "wins", "value": 12, "displayValue": "12"}]
                    }]}
                },
                {
                    "name": "Western Conference",
                    "abbreviation": "W",
                    "standings": {"entries": [{
                        "team": {"id": "2", "name": "Lynx", "abbreviation": "MIN"},
                        "stats": [{"name": "wins", "value": 13, "displayValue": "13"}]
                    }]}
                }
            ]
        }))
        .unwrap();

        let standings = espn_wnba_standings(data);

        assert_eq!(standings.east.len(), 1);
        assert_eq!(standings.east[0].team_tricode, "NY");
        assert_eq!(standings.west.len(), 1);
        assert_eq!(standings.west[0].team_tricode, "MIN");
    }

    #[test]
    fn espn_basketball_standings_classifies_west_from_abbreviation_only() {
        let data: EspnStandingsDto = serde_json::from_value(serde_json::json!({
            "children": [
                {
                    "abbreviation": "East",
                    "standings": {"entries": [{
                        "team": {"id": "1", "name": "Celtics", "abbreviation": "BOS"},
                        "stats": [{"name": "wins", "value": 56, "displayValue": "56"}]
                    }]}
                },
                {
                    "abbreviation": "West",
                    "standings": {"entries": [{
                        "team": {"id": "2", "name": "Nuggets", "abbreviation": "DEN"},
                        "stats": [{"name": "wins", "value": 54, "displayValue": "54"}]
                    }]}
                }
            ]
        }))
        .unwrap();

        let standings = espn_standings(data);

        assert_eq!(standings.east.len(), 1);
        assert_eq!(standings.east[0].team_tricode, "BOS");
        assert_eq!(standings.west.len(), 1);
        assert_eq!(standings.west[0].team_tricode, "DEN");
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
    fn espn_mlb_scoreboard_tolerates_missing_optional_fields() {
        let data: EspnScoreboardDto = serde_json::from_value(serde_json::json!({
            "events": [{
                "id": "401815096",
                "date": "2026-04-26T17:35:00Z",
                "competitions": [{
                    "status": {"type": {"name": "STATUS_SCHEDULED", "shortDetail": "7:10 PM"}},
                    "competitors": [
                        {"homeAway": "away", "id": "2", "score": "0", "team": {"id": "2", "location": "Boston", "name": "Red Sox", "abbreviation": "BOS"}},
                        {"homeAway": "home", "id": "1", "score": "0", "team": {"id": "1", "location": "Baltimore", "name": "Orioles", "abbreviation": "BAL"}}
                    ]
                }]
            }]
        }))
        .unwrap();

        let scoreboard = espn_mlb_scoreboard("2026-04-26", data).unwrap();

        assert_eq!(scoreboard.games[0].away_team.display_record, "0-0");
        assert_eq!(scoreboard.games[0].away_team.hits, 0);
        assert!(scoreboard.games[0].away_team.periods.is_empty());
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
    fn espn_nfl_summary_tolerates_missing_optional_boxscore_groups() {
        let data: EspnSummaryDto = serde_json::from_value(serde_json::json!({
            "header": {
                "id": "401772846",
                "competitions": [{
                    "status": {"type": {"name": "STATUS_SCHEDULED"}},
                    "competitors": [
                        {"homeAway": "away", "id": "21", "score": "0", "team": {"id": "21"}},
                        {"homeAway": "home", "id": "27", "score": "0", "team": {"id": "27"}}
                    ]
                }]
            },
            "boxscore": {},
            "gameInfo": null
        }))
        .unwrap();

        let game = espn_nfl_summary(data).unwrap();

        assert!(game.away_team.team_stats.rows.is_empty());
        assert!(game.home_team.player_stats.is_empty());
    }

    #[test]
    fn espn_nhl_scoreboard_conversion_uses_hockey_records() {
        let data: EspnScoreboardDto = serde_json::from_value(serde_json::json!({
            "events": [{
                "id": "401900001",
                "date": "2026-04-26T23:00:00Z",
                "competitions": [{
                    "series": {"type": "playoff", "competitors": [{"id": "1", "wins": 3}, {"id": "13", "wins": 2}]},
                    "status": {"period": 3, "displayClock": "0:00", "type": {"name": "STATUS_FINAL", "completed": true, "shortDetail": "Final"}},
                    "competitors": [
                        {"homeAway": "away", "id": "1", "score": "3", "team": {"id": "1", "location": "Boston", "name": "Bruins", "abbreviation": "BOS"}, "record": "3-2", "records": [{"type": "total", "summary": "45-27-10"}], "linescores": [{"displayValue": "1"}, {"displayValue": "1"}, {"displayValue": "1"}]},
                        {"homeAway": "home", "id": "13", "score": "2", "team": {"id": "13", "location": "New York", "name": "Rangers", "abbreviation": "NYR"}, "record": "2-3", "records": [{"type": "total", "summary": "47-25-10"}], "linescores": [{"displayValue": "0"}, {"displayValue": "1"}, {"displayValue": "1"}]}
                    ]
                }]
            }]
        }))
        .unwrap();
        let scoreboard = espn_nhl_scoreboard("2026-04-26", data).unwrap();
        assert_eq!(scoreboard.games.len(), 1);
        assert_eq!(scoreboard.games[0].away_team.team_tricode, "BOS");
        assert_eq!(scoreboard.games[0].away_team.display_record, "3-2");
        assert_eq!(scoreboard.games[0].home_team.display_record, "2-3");
        assert_eq!(scoreboard.games[0].away_team.periods[2].score, 1);
        assert_eq!(scoreboard.games[0].game_status, 3);
    }

    #[test]
    fn espn_nhl_summary_conversion_produces_team_and_player_tables() {
        let data: EspnSummaryDto = serde_json::from_value(serde_json::json!({
            "header": {
                "id": "401900001",
                "competitions": [{
                    "series": [
                        {"type": "season", "competitors": [{"id": "1", "wins": 0}, {"id": "13", "wins": 2}]},
                        {"type": "playoff", "competitors": [{"id": "1", "wins": 3}, {"id": "13", "wins": 2}]}
                    ],
                    "status": {"type": {"name": "STATUS_FINAL"}},
                    "competitors": [
                        {"homeAway": "away", "id": "1", "score": "3", "team": {"id": "1", "location": "Boston", "name": "Bruins", "abbreviation": "BOS"}, "record": [{"type": "total", "summary": "45-27-10"}], "linescores": [{"displayValue": "1"}, {"displayValue": "1"}, {"displayValue": "1"}]},
                        {"homeAway": "home", "id": "13", "score": "2", "team": {"id": "13", "location": "New York", "name": "Rangers", "abbreviation": "NYR"}, "record": [{"type": "total", "summary": "47-25-10"}], "linescores": [{"displayValue": "0"}, {"displayValue": "1"}, {"displayValue": "1"}]}
                    ]
                }]
            },
            "boxscore": {
                "teams": [
                    {"homeAway": "away", "team": {"abbreviation": "BOS"}, "statistics": [{"label": "Shots", "displayValue": "31"}]},
                    {"homeAway": "home", "team": {"abbreviation": "NYR"}, "statistics": [{"label": "Shots", "displayValue": "29"}]}
                ],
                "players": [
                    {"team": {"abbreviation": "BOS"}, "statistics": [{"text": "Boston Skaters", "labels": ["G", "A"], "athletes": [{"athlete": {"displayName": "David Pastrnak"}, "stats": ["1", "1"]}]}]},
                    {"team": {"abbreviation": "NYR"}, "statistics": [{"text": "New York Skaters", "labels": ["G", "A"], "athletes": [{"athlete": {"displayName": "Artemi Panarin"}, "stats": ["0", "1"]}]}]}
                ]
            },
            "gameInfo": null
        }))
        .unwrap();
        let game = espn_nhl_summary(data).unwrap();
        assert_eq!(game.away_team.team.team_tricode, "BOS");
        assert_eq!(game.away_team.team.display_record, "3-2");
        assert_eq!(game.home_team.team.display_record, "2-3");
        assert_eq!(game.away_team.team_stats.rows[0][0], "Shots");
        assert_eq!(game.away_team.player_stats[0].name, "Boston Skaters");
        assert_eq!(game.away_team.player_stats[0].rows[0][0], "David Pastrnak");
    }

    #[test]
    fn espn_nhl_summary_tolerates_missing_optional_boxscore_groups() {
        let data: EspnSummaryDto = serde_json::from_value(serde_json::json!({
            "header": {
                "id": "401900002",
                "competitions": [{
                    "status": {"type": {"name": "STATUS_SCHEDULED"}},
                    "competitors": [
                        {"homeAway": "away", "id": "1", "score": "0", "team": {"id": "1", "location": "Boston", "name": "Bruins", "abbreviation": "BOS"}},
                        {"homeAway": "home", "id": "13", "score": "0", "team": {"id": "13", "location": "New York", "name": "Rangers", "abbreviation": "NYR"}}
                    ]
                }]
            },
            "boxscore": {},
            "gameInfo": null
        }))
        .unwrap();

        let game = espn_nhl_summary(data).unwrap();

        assert!(game.away_team.team_stats.rows.is_empty());
        assert!(game.home_team.player_stats.is_empty());
    }

    #[test]
    fn espn_soccer_scoreboard_conversion_produces_match_cards() {
        let data: EspnScoreboardDto = serde_json::from_value(serde_json::json!({
            "events": [{
                "id": "633790",
                "date": "2022-11-20T16:00Z",
                "competitions": [{
                    "status": {"period": 2, "displayClock": "90'+6'", "type": {"name": "STATUS_FULL_TIME", "completed": true, "shortDetail": "FT"}},
                    "competitors": [
                        {"homeAway": "home", "id": "4398", "score": "0", "team": {"id": "4398", "location": "Qatar", "name": "Qatar", "abbreviation": "QAT"}, "records": [{"type": "total", "summary": "0-0-1"}]},
                        {"homeAway": "away", "id": "209", "score": "2", "team": {"id": "209", "location": "Ecuador", "name": "Ecuador", "abbreviation": "ECU"}, "records": [{"type": "total", "summary": "1-0-0"}]}
                    ]
                }]
            }]
        }))
        .unwrap();

        let scoreboard = espn_soccer_scoreboard("2022-11-20", data).unwrap();

        assert_eq!(scoreboard.games[0].away_team.team_tricode, "ECU");
        assert_eq!(scoreboard.games[0].away_team.display_record, "1-0-0");
        assert!(scoreboard.games[0].away_team.periods.is_empty());
        assert_eq!(scoreboard.games[0].game_status_text, "FT");
    }

    #[test]
    fn espn_soccer_bracket_orders_by_match_number_and_flags_placeholders() {
        let data: EspnScoreboardDto = serde_json::from_value(serde_json::json!({
            "events": [
                {
                    "id": "200",
                    "date": "2026-06-29T16:00Z",
                    "season": {"slug": "round-of-16"},
                    "competitions": [{
                        "status": {"type": {"completed": true, "state": "post", "shortDetail": "FT"}},
                        "competitors": [
                            {"homeAway": "home", "winner": true, "score": "3", "team": {"id": "83", "displayName": "Brazil", "shortDisplayName": "BRA", "abbreviation": "BRA", "isActive": true, "logo": "https://logos/bra.png"}},
                            {"homeAway": "away", "winner": false, "score": "1", "team": {"id": "85", "displayName": "Chile", "shortDisplayName": "CHI", "abbreviation": "CHI", "isActive": true, "logo": "https://logos/chi.png"}}
                        ]
                    }]
                },
                {
                    "id": "201",
                    "date": "2026-06-28T16:00Z",
                    "season": {"slug": "round-of-16"},
                    "competitions": [{
                        "status": {"type": {"completed": false, "state": "pre", "shortDetail": "6/28 - 12:00 PM"}},
                        "competitors": [
                            {"homeAway": "home", "score": "0", "team": {"displayName": "Group A Winner", "isActive": false, "logo": ""}},
                            {"homeAway": "away", "score": "0", "team": {"displayName": "Group B Runner-Up", "isActive": false, "logo": ""}}
                        ]
                    }]
                },
                {
                    "id": "300",
                    "date": "2026-07-18T16:00Z",
                    "season": {"slug": "3rd-place-match"},
                    "competitions": [{
                        "status": {"type": {"completed": false, "state": "pre", "shortDetail": "7/18 - 12:00 PM"}},
                        "competitors": [
                            {"homeAway": "home", "score": "0", "team": {"displayName": "Semifinal 1 Loser", "isActive": false, "logo": ""}},
                            {"homeAway": "away", "score": "0", "team": {"displayName": "Semifinal 2 Loser", "isActive": false, "logo": ""}}
                        ]
                    }]
                }
            ]
        }))
        .unwrap();

        let match_numbers = HashMap::from([("200".to_string(), 90), ("201".to_string(), 89)]);
        let bracket = espn_soccer_bracket(data, &match_numbers).unwrap();

        assert_eq!(bracket.rounds.len(), 1);
        let round = &bracket.rounds[0];
        assert_eq!(round.name, "Round of 16");
        // Lower match number sorts first even though it is later in the feed.
        assert_eq!(round.matches[0].game_id, "201");
        assert_eq!(round.matches[1].game_id, "200");

        let placeholder_match = &round.matches[0];
        assert!(placeholder_match.home.placeholder);
        assert_eq!(placeholder_match.home.name, "Group A Winner");

        let played_match = &round.matches[1];
        assert!(!played_match.home.placeholder);
        assert!(played_match.home.winner);
        assert_eq!(played_match.home.score, "3");
        assert_eq!(played_match.game_status, 3);

        let third = bracket.third_place.expect("third place match");
        assert_eq!(third.game_id, "300");
        assert_eq!(third.away.name, "Semifinal 2 Loser");
    }

    #[test]
    fn espn_soccer_bracket_tree_orders_partially_played_bracket() {
        let (data, match_numbers) = synthetic_bracket_data(false);
        let bracket = espn_soccer_bracket(data, &match_numbers).unwrap();

        assert_eq!(bracket.rounds.len(), 5);
        assert_eq!(
            round_game_ids(&bracket, 0),
            [
                "r32-1", "r32-9", "r32-5", "r32-13", "r32-3", "r32-11", "r32-7", "r32-15", "r32-2",
                "r32-10", "r32-6", "r32-14", "r32-4", "r32-12", "r32-8", "r32-16",
            ]
        );
        assert_eq!(
            round_game_ids(&bracket, 1),
            [
                "r16-1", "r16-5", "r16-3", "r16-7", "r16-2", "r16-6", "r16-4", "r16-8",
            ]
        );
        assert_eq!(
            round_game_ids(&bracket, 2),
            ["qf-1", "qf-3", "qf-2", "qf-4"]
        );
        assert_eq!(round_game_ids(&bracket, 3), ["sf-1", "sf-2"]);
        assert_eq!(round_game_ids(&bracket, 4), ["final-1"]);

        let (data, match_numbers) = synthetic_bracket_data(true);
        let bracket = espn_soccer_bracket(data, &match_numbers).unwrap();

        assert_eq!(
            round_game_ids(&bracket, 0),
            [
                "r32-1", "r32-9", "r32-5", "r32-13", "r32-3", "r32-11", "r32-7", "r32-15", "r32-2",
                "r32-10", "r32-6", "r32-14", "r32-4", "r32-12", "r32-8", "r32-16",
            ]
        );
        assert_eq!(
            round_game_ids(&bracket, 1),
            [
                "r16-1", "r16-5", "r16-3", "r16-7", "r16-2", "r16-6", "r16-4", "r16-8",
            ]
        );
        assert_eq!(bracket.rounds[0].matches[0].home.name, "R32 Team 1 A");
        assert_eq!(bracket.rounds[1].matches[0].home.name, "Team 1001");
        assert!(!bracket.rounds[1].matches[0].home.placeholder);
    }

    #[test]
    fn parse_bracket_feeder_reads_round_and_slot() {
        assert_eq!(
            parse_bracket_feeder("Round of 32 3 Winner"),
            Some(("round-of-32", 3))
        );
        assert_eq!(
            parse_bracket_feeder("Quarterfinal 2 Winner"),
            Some(("quarterfinals", 2))
        );
        assert_eq!(parse_bracket_feeder("Semifinal 1 Loser"), None);
        assert_eq!(parse_bracket_feeder("Brazil"), None);
    }

    #[test]
    fn espn_soccer_summary_conversion_produces_team_stats() {
        let data: EspnSummaryDto = serde_json::from_value(serde_json::json!({
            "header": {
                "id": "633790",
                "competitions": [{
                    "status": {"type": {"name": "STATUS_FULL_TIME", "completed": true, "shortDetail": "FT"}},
                    "competitors": [
                        {"homeAway": "home", "id": "4398", "score": "0", "team": {"id": "4398", "location": "Qatar", "name": "Qatar", "abbreviation": "QAT"}, "record": [{"type": "total", "summary": "0-0-1"}]},
                        {"homeAway": "away", "id": "209", "score": "2", "team": {"id": "209", "location": "Ecuador", "name": "Ecuador", "abbreviation": "ECU"}, "record": [{"type": "total", "summary": "1-0-0"}]}
                    ]
                }]
            },
            "boxscore": {
                "teams": [
                    {"team": {"id": "4398", "displayName": "Qatar"}, "statistics": [{"label": "Possession", "displayValue": "47.1"}]},
                    {"team": {"id": "209", "displayName": "Ecuador"}, "statistics": [{"label": "Possession", "displayValue": "52.9"}]}
                ]
            },
            "keyEvents": [
                {"type": {"type": "penalty---scored"}, "clock": {"displayValue": "16'"}, "scoringPlay": true, "penaltyKick": true, "team": {"id": "209"}, "participants": [{"athlete": {"displayName": "Enner Valencia"}}]},
                {"type": {"type": "goal---header"}, "clock": {"displayValue": "31'"}, "scoringPlay": true, "team": {"id": "209"}, "participants": [{"athlete": {"displayName": "Enner Valencia"}}, {"athlete": {"displayName": "Ángelo Preciado"}}]},
                {"type": {"type": "yellow-card"}, "clock": {"displayValue": "29'"}, "scoringPlay": false, "team": {"id": "209"}, "participants": [{"athlete": {"displayName": "Moisés Caicedo"}}]}
            ],
            "gameInfo": null
        }))
        .unwrap();

        let game = espn_soccer_summary(data).unwrap();

        assert_eq!(game.away_team.team.team_tricode, "ECU");
        assert_eq!(game.away_team.team_stats.rows[0], ["Possession", "52.9"]);
        assert_eq!(game.home_team.team_stats.rows[0], ["Possession", "47.1"]);
        assert_eq!(game.events[0].player, "Enner Valencia");
        assert_eq!(game.events[0].note, "Penalty");
        assert_eq!(game.events[1].assist, "Ángelo Preciado");
        assert_eq!(game.events[2].kind, "Yellow Card");
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

    #[test]
    fn espn_nhl_standings_conversion_groups_by_division() {
        let data: EspnStandingsDto = serde_json::from_value(serde_json::json!({
            "children": [
                {"abbreviation": "East", "standings": {"entries": [
                    {"team": {"id": "1", "displayName": "Boston Bruins", "abbreviation": "BOS"}, "stats": [
                        {"name": "playoffSeed", "value": 2, "displayValue": "2"},
                        {"name": "wins", "value": 45, "displayValue": "45"},
                        {"name": "losses", "value": 27, "displayValue": "27"},
                        {"name": "otLosses", "value": 10, "displayValue": "10"},
                        {"name": "points", "value": 100, "displayValue": "100"},
                        {"name": "gamesBehind", "value": 0, "displayValue": "-"},
                        {"name": "pointsFor", "value": 263, "displayValue": "263"},
                        {"name": "pointsAgainst", "value": 241, "displayValue": "241"},
                        {"name": "pointDifferential", "value": 22, "displayValue": "+22"},
                        {"name": "Home", "displayValue": "24-12-5"},
                        {"name": "Road", "displayValue": "21-15-5"},
                        {"name": "vs. Div.", "displayValue": "14-8-4"},
                        {"name": "Last Ten Games", "displayValue": "6-3-1"},
                        {"name": "streak", "displayValue": "W2"}
                    ]}
                ]}},
                {"abbreviation": "West", "standings": {"entries": [
                    {"team": {"id": "37", "displayName": "Vegas Golden Knights", "abbreviation": "VGK"}, "stats": [
                        {"name": "playoffSeed", "value": 4, "displayValue": "4"},
                        {"name": "wins", "value": 39, "displayValue": "39"},
                        {"name": "losses", "value": 26, "displayValue": "26"},
                        {"name": "otLosses", "value": 17, "displayValue": "17"},
                        {"name": "points", "value": 95, "displayValue": "95"},
                        {"name": "pointsFor", "value": 265, "displayValue": "265"},
                        {"name": "pointsAgainst", "value": 250, "displayValue": "250"},
                        {"name": "pointsDiff", "value": 15, "displayValue": "+15"},
                        {"name": "streak", "displayValue": "W3"}
                    ]}
                ]}}
            ]
        }))
        .unwrap();
        let standings = espn_nhl_standings(data);
        let atlantic = standings
            .divisions
            .iter()
            .find(|division| division.conference == "Eastern" && division.division == "Atlantic")
            .unwrap();
        let pacific = standings
            .divisions
            .iter()
            .find(|division| division.conference == "Western" && division.division == "Pacific")
            .unwrap();
        assert_eq!(atlantic.teams[0].team_name, "Boston Bruins");
        assert_eq!(atlantic.teams[0].ot_losses, 10);
        assert_eq!(atlantic.teams[0].division_record, "14-8-4");
        assert_eq!(pacific.teams[0].team_tricode, "VGK");
        assert_eq!(pacific.teams[0].goal_diff, "+15");
    }
}
