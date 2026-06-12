use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Scoreboard {
    pub game_date: String,
    pub games: Vec<Game>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Game {
    pub game_id: String,
    pub game_status: i64,
    pub game_status_text: String,
    pub period: i64,
    pub game_clock: String,
    pub game_time_utc: String,
    pub home_team: Team,
    pub away_team: Team,
    pub home_leaders: Leaders,
    pub away_leaders: Leaders,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Team {
    pub team_id: i64,
    pub team_name: String,
    pub team_city: String,
    pub team_tricode: String,
    pub wins: i64,
    pub losses: i64,
    #[serde(default)]
    pub display_record: String,
    pub score: i64,
    #[serde(default)]
    pub hits: i64,
    #[serde(default)]
    pub errors: i64,
    pub periods: Vec<Period>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Period {
    pub period: i64,
    pub score: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Leaders {
    pub person_id: i64,
    pub name: String,
    pub team_tricode: String,
    pub points: f64,
    pub rebounds: f64,
    pub assists: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoxScore {
    pub game_id: String,
    pub game_status: i64,
    pub home_team: BoxScoreTeam,
    pub away_team: BoxScoreTeam,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoxScoreTeam {
    pub team: Team,
    pub players: Vec<Player>,
    pub statistics: TeamStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    pub person_id: i64,
    pub name: String,
    pub starter: bool,
    pub played: bool,
    pub statistics: Statistics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Statistics {
    pub assists: i64,
    pub blocks: i64,
    pub blocks_received: i64,
    pub field_goals_attempted: i64,
    pub field_goals_made: i64,
    pub fouls_personal: i64,
    pub free_throws_attempted: i64,
    pub free_throws_made: i64,
    pub minutes: i64,
    pub plus_minus_points: i64,
    pub points: i64,
    pub rebounds_defensive: i64,
    pub rebounds_offensive: i64,
    pub rebounds_total: i64,
    pub steals: i64,
    pub three_pointers_attempted: i64,
    pub three_pointers_made: i64,
    pub turnovers: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TeamStatistics {
    pub assists: i64,
    pub blocks: i64,
    pub field_goals_attempted: i64,
    pub field_goals_made: i64,
    pub fouls_personal: i64,
    pub free_throws_attempted: i64,
    pub free_throws_made: i64,
    pub minutes: i64,
    pub points: i64,
    pub rebounds_defensive: i64,
    pub rebounds_offensive: i64,
    pub rebounds_total: i64,
    pub steals: i64,
    pub three_pointers_attempted: i64,
    pub three_pointers_made: i64,
    pub turnovers: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StandingsTable {
    pub east: Vec<StandingsTeam>,
    pub west: Vec<StandingsTeam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StandingsTeam {
    pub team_id: i64,
    pub team_name: String,
    #[serde(default)]
    pub team_tricode: String,
    pub conference: String,
    pub playoff_rank: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_pct: f64,
    pub conference_games_back: f64,
    pub points_pg: f64,
    pub opp_points_pg: f64,
    pub diff_points_pg: f64,
    pub home: String,
    pub road: String,
    pub last_ten: String,
    pub current_streak: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerStatsPage {
    pub tables: Vec<Table>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MlbBoxScore {
    pub game_id: String,
    pub game_status: i64,
    pub away_team: MlbBoxScoreTeam,
    pub home_team: MlbBoxScoreTeam,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MlbBoxScoreTeam {
    pub team: Team,
    pub batting: Table,
    pub pitching: Table,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MlbStandingsTable {
    pub divisions: Vec<MlbStandingsDivision>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MlbStandingsDivision {
    pub league: String,
    pub division: String,
    pub teams: Vec<MlbStandingsTeam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MlbStandingsTeam {
    pub team_id: i64,
    pub team_name: String,
    pub team_tricode: String,
    pub league: String,
    pub division: String,
    pub playoff_rank: i64,
    pub wins: i64,
    pub losses: i64,
    pub win_pct: String,
    pub games_back: String,
    pub runs_scored: i64,
    pub runs_allowed: i64,
    pub run_diff: String,
    pub streak: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NflBoxScore {
    pub game_id: String,
    pub game_status: i64,
    pub away_team: NflBoxScoreTeam,
    pub home_team: NflBoxScoreTeam,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NflBoxScoreTeam {
    pub team: Team,
    pub team_stats: Table,
    pub player_stats: Vec<Table>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NflStandingsTable {
    pub divisions: Vec<NflStandingsDivision>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NflStandingsDivision {
    pub conference: String,
    pub division: String,
    pub teams: Vec<NflStandingsTeam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NflStandingsTeam {
    pub team_id: i64,
    pub team_name: String,
    pub team_tricode: String,
    pub conference: String,
    pub division: String,
    pub playoff_rank: i64,
    pub wins: i64,
    pub losses: i64,
    pub ties: i64,
    pub win_pct: String,
    pub games_back: String,
    pub points_for: i64,
    pub points_against: i64,
    pub point_diff: String,
    pub home: String,
    pub road: String,
    pub division_record: String,
    pub conference_record: String,
    pub streak: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NhlBoxScore {
    pub game_id: String,
    pub game_status: i64,
    pub away_team: NhlBoxScoreTeam,
    pub home_team: NhlBoxScoreTeam,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NhlBoxScoreTeam {
    pub team: Team,
    pub team_stats: Table,
    pub player_stats: Vec<Table>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NhlStandingsTable {
    pub divisions: Vec<NhlStandingsDivision>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NhlStandingsDivision {
    pub conference: String,
    pub division: String,
    pub teams: Vec<NhlStandingsTeam>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NhlStandingsTeam {
    pub team_id: i64,
    pub team_name: String,
    pub team_tricode: String,
    pub conference: String,
    pub division: String,
    pub playoff_rank: i64,
    pub wins: i64,
    pub losses: i64,
    pub ot_losses: i64,
    pub points: i64,
    pub games_back: String,
    pub goals_for: i64,
    pub goals_against: i64,
    pub goal_diff: String,
    pub home: String,
    pub road: String,
    pub division_record: String,
    pub last_ten: String,
    pub streak: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Table {
    pub name: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    #[serde(default)]
    pub first_column_links: Vec<String>,
}
