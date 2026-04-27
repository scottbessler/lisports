use chrono::{Datelike, Days, NaiveDate, Utc};

use crate::{
    models::{
        BoxScore, BoxScoreTeam, Game, MlbBoxScore, MlbBoxScoreTeam, MlbStandingsTable,
        MlbStandingsTeam, PlayerStatsPage, Scoreboard, StandingsTable, StandingsTeam, Table, Team,
    },
    stats,
};

#[derive(Clone, Copy)]
enum League {
    Nba,
    Mlb,
}

pub fn layout(title: &str, body: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en" data-theme="light">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <title>{}</title>
  <link rel="icon" href="/public/favicon.ico">
  <link rel="stylesheet" href="/public/app.css">
</head>
<body>
  {}
  {}
  <script src="/public/table-sort.js"></script>
</body>
</html>"#,
        escape(title),
        nav(),
        body
    )
}

pub fn nav() -> &'static str {
    r#"<nav class="nav">
  <div class="brand">LiSports</div>
  <a href="/nba/scoreboard">NBA Scoreboard</a>
  <a href="/nba/standings">NBA Standings</a>
  <a href="/mlb/scoreboard">MLB Scoreboard</a>
  <a href="/mlb/standings">MLB Standings</a>
  <a href="/nfl/scoreboard">NFL</a>
</nav>"#
}

pub fn error_page(title: &str, message: &str) -> String {
    layout(
        title,
        &format!(
            r#"<main class="center"><h1>{}</h1><p>{}</p></main>"#,
            escape(title),
            escape(message)
        ),
    )
}

pub fn coming_soon_page() -> String {
    layout(
        "Coming Soon",
        r#"<main class="center"><h1>Coming eventually?</h1></main>"#,
    )
}

pub fn scoreboard_page(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&BoxScore>,
) -> String {
    basketball_scoreboard_page(day, scoreboard, selected)
}

fn basketball_scoreboard_page(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&BoxScore>,
) -> String {
    let mut html = String::from(r#"<main class="page">"#);
    html.push_str(&date_nav(day, "/nba/scoreboard"));
    if scoreboard.games.is_empty() {
        html.push_str(r#"<section class="center"><h1>No Games Scheduled</h1></section>"#);
    } else {
        let class = if selected.is_some() {
            "scoreboard has-game"
        } else {
            "scoreboard"
        };
        html.push_str(&format!(r#"<section class="{class}">"#));
        html.push_str(r#"<div class="game-list">"#);
        let all_completed = scoreboard.games.iter().all(|g| g.game_status == 3);
        for game in &scoreboard.games {
            html.push_str(&format!(
                r#"<a class="game-link" href="/nba/scoreboard/{}/game/{}">{}</a>"#,
                day,
                escape_attr(&game.game_id),
                game_summary(game, !all_completed, League::Nba)
            ));
        }
        html.push_str("</div>");
        if let Some(game) = selected {
            html.push_str(&game_details(game));
        }
        html.push_str("</section>");
    }
    html.push_str("</main>");
    layout(
        if selected.is_some() {
            "NBA Game"
        } else {
            "NBA Scoreboard"
        },
        &html,
    )
}

pub fn mlb_scoreboard_page(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&MlbBoxScore>,
) -> String {
    let mut html = String::from(r#"<main class="page">"#);
    html.push_str(&date_nav(day, "/mlb/scoreboard"));
    if scoreboard.games.is_empty() {
        html.push_str(r#"<section class="center"><h1>No Games Scheduled</h1></section>"#);
    } else {
        let class = if selected.is_some() {
            "scoreboard has-game"
        } else {
            "scoreboard"
        };
        html.push_str(&format!(r#"<section class="{class}">"#));
        html.push_str(r#"<div class="game-list">"#);
        let all_completed = scoreboard.games.iter().all(|g| g.game_status == 3);
        for game in &scoreboard.games {
            html.push_str(&format!(
                r#"<a class="game-link" href="/mlb/scoreboard/{}/game/{}">{}</a>"#,
                day,
                escape_attr(&game.game_id),
                game_summary(game, !all_completed, League::Mlb)
            ));
        }
        html.push_str("</div>");
        if let Some(game) = selected {
            html.push_str(&mlb_game_details(game));
        }
        html.push_str("</section>");
    }
    html.push_str("</main>");
    layout(
        if selected.is_some() {
            "MLB Game"
        } else {
            "MLB Scoreboard"
        },
        &html,
    )
}

fn date_nav(day: NaiveDate, base_path: &str) -> String {
    let mut html = String::from(r#"<div class="date-nav">"#);
    html.push_str(&format!(
        r#"<a class="button" href="{}/{}">Prev</a>"#,
        base_path,
        day.checked_sub_days(Days::new(1)).unwrap_or(day)
    ));
    for offset in -3..=3 {
        let d = day + chrono::Duration::days(offset);
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
        let class = if d == day { "button active" } else { "button" };
        html.push_str(&format!(
            r#"<a class="{class}" href="{base_path}/{d}">{}</a>"#,
            escape(&label)
        ));
    }
    html.push_str(&format!(
        r#"<a class="button" href="{}/{}">Next</a>"#,
        base_path,
        day.checked_add_days(Days::new(1)).unwrap_or(day)
    ));
    html.push_str("</div>");
    html
}

fn game_summary(game: &Game, show_status: bool, league: League) -> String {
    let mut html = String::from(r#"<table class="game-card"><thead><tr><th></th>"#);
    for period in &game.away_team.periods {
        html.push_str(&format!("<th>{}</th>", period.period));
    }
    match league {
        League::Nba => html.push_str("<th>T</th></tr></thead><tbody>"),
        League::Mlb => html.push_str("<th>R</th><th>H</th><th>E</th></tr></thead><tbody>"),
    }
    html.push_str(&team_summary_row(game, &game.away_team, false, league));
    html.push_str(&team_summary_row(game, &game.home_team, true, league));
    if show_status {
        let colspan = match league {
            League::Nba => game.away_team.periods.len() + 2,
            League::Mlb => game.away_team.periods.len() + 4,
        };
        html.push_str(&format!(
            r#"<tr><th class="status" colspan="{colspan}">{}</th></tr>"#,
            game_status(game)
        ));
    }
    html.push_str("</tbody></table>");
    html
}

fn team_summary_row(game: &Game, team: &Team, is_home: bool, league: League) -> String {
    let mut html = String::from("<tr><th>");
    html.push_str(&team_logo(team, "mini-logo", league));
    html.push_str(&format!(
        r#"<span title="{}">{}</span> <small>({})</small> {}"#,
        escape_attr(&format!("{} {}", team.team_city, team.team_name)),
        escape(&team.team_tricode),
        escape(&team_record(team)),
        winner(game, is_home)
    ));
    html.push_str("</th>");
    for period in &team.periods {
        html.push_str(&format!(
            "<td>{}</td>",
            if period.score == 0 {
                "-".to_string()
            } else {
                period.score.to_string()
            }
        ));
    }
    match league {
        League::Nba => html.push_str(&format!("<td>{}</td></tr>", team.score)),
        League::Mlb => html.push_str(&format!(
            "<td>{}</td><td>{}</td><td>{}</td></tr>",
            team.score, team.hits, team.errors
        )),
    }
    html
}

fn game_details(game: &BoxScore) -> String {
    let mut html = String::from(r#"<section class="details">"#);
    html.push_str(&team_game_details(
        game,
        &game.away_team,
        &game.home_team,
        false,
    ));
    html.push_str(&team_game_details(
        game,
        &game.home_team,
        &game.away_team,
        true,
    ));
    html.push_str(&format!(
        r#"<p><a href="https://www.nba.com/game/{}-vs-{}-{}?watchFullGame">Watch on League Pass</a></p>"#,
        escape_attr(&game.away_team.team.team_tricode),
        escape_attr(&game.home_team.team.team_tricode),
        escape_attr(&game.game_id)
    ));
    html.push_str("</section>");
    html
}

fn mlb_game_details(game: &MlbBoxScore) -> String {
    let mut html = String::from(r#"<section class="details">"#);
    html.push_str(&mlb_team_game_details(game, &game.away_team, false));
    html.push_str(&mlb_team_game_details(game, &game.home_team, true));
    html.push_str("</section>");
    html
}

fn mlb_team_game_details(game: &MlbBoxScore, team: &MlbBoxScoreTeam, is_home: bool) -> String {
    let mut html = String::from(r#"<article class="team-details">"#);
    html.push_str("<h1>");
    html.push_str(&team_logo(&team.team, "logo", League::Mlb));
    html.push_str(&format!(
        "{} {} <strong>{}</strong> {}",
        escape(&team.team.team_city),
        escape(&team.team.team_name),
        team.team.score,
        mlb_box_winner(game, is_home)
    ));
    html.push_str("</h1>");
    html.push_str(&format!(
        r#"<section class="box-score-group"><h2>{}</h2>{}</section>"#,
        escape(&team.batting.name),
        render_table(&team.batting)
    ));
    html.push_str(&format!(
        r#"<section class="box-score-group"><h2>{}</h2>{}</section>"#,
        escape(&team.pitching.name),
        render_table(&team.pitching)
    ));
    html.push_str("</article>");
    html
}

fn team_game_details(
    game: &BoxScore,
    team: &BoxScoreTeam,
    other: &BoxScoreTeam,
    is_home: bool,
) -> String {
    let mut html = String::from(r#"<article class="team-details">"#);
    html.push_str("<h1>");
    html.push_str(&team_logo(&team.team, "logo", League::Nba));
    html.push_str(&format!(
        "{} {} <strong>{}</strong> {}",
        escape(&team.team.team_city),
        escape(&team.team.team_name),
        team.team.score,
        box_winner(game, is_home)
    ));
    html.push_str("</h1>");
    html.push_str(&team_box(team, other));
    html.push_str("</article>");
    html
}

fn team_box(team: &BoxScoreTeam, other: &BoxScoreTeam) -> String {
    let mut html = String::from(
        r#"<div class="table-wrap"><table class="sortable box-score-table" data-sort-group="box-score" data-default-sort-index="5" data-default-sort-dir="desc"><thead><tr><th class="text">Name</th><th>MIN</th><th>PTS</th><th>RB</th><th>AS</th><th>PIE</th><th>FG</th><th>3P</th><th>FT</th><th>PPS</th><th>TO</th><th>ST</th><th>BK</th><th>PF</th><th>+/-</th><th>USG</th></tr></thead><tbody>"#,
    );
    for player in team.players.iter().filter(|p| p.played) {
        let s = &player.statistics;
        html.push_str("<tr>");
        html.push_str(&format!(
            r#"<th><a href="/nba/player/{}">{}{}</a></th>"#,
            player.person_id,
            escape(&player.name),
            if player.starter { "*" } else { "" }
        ));
        html.push_str(&format!("<td>{}</td>", s.minutes));
        html.push_str(&stat_cell(s.points, 20, true));
        html.push_str(&stat_cell(s.rebounds_total, 10, true));
        html.push_str(&stat_cell(s.assists, 8, true));
        html.push_str(&format!(
            "<td>{}</td>",
            format_number(stats::pie(s, &team.statistics, &other.statistics) as f64, 2)
        ));
        html.push_str(&format!(
            "<td>{}-{}</td>",
            s.field_goals_made, s.field_goals_attempted
        ));
        html.push_str(&format!(
            "<td>{}-{}</td>",
            s.three_pointers_made, s.three_pointers_attempted
        ));
        html.push_str(&format!(
            "<td>{}-{}</td>",
            s.free_throws_made, s.free_throws_attempted
        ));
        html.push_str(&format!(
            "<td>{}</td>",
            stats::points_per_shot(s)
                .map(|v| format!("{v:.2}"))
                .unwrap_or_default()
        ));
        html.push_str(&stat_cell(s.turnovers, 3, false));
        html.push_str(&stat_cell(s.steals, 3, true));
        html.push_str(&stat_cell(s.blocks, 3, true));
        html.push_str(&stat_cell(s.fouls_personal, 5, false));
        html.push_str(&format!("<td>{}</td>", s.plus_minus_points));
        html.push_str(&format!(
            "<td>{}</td>",
            stats::usage_rate(s, &team.statistics)
                .map(|v| format_number(v as f64, 2))
                .unwrap_or_default()
        ));
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");
    html
}

pub fn standings_page(standings: &StandingsTable) -> String {
    layout(
        "NBA Standings",
        &format!(
            r#"<main class="page standings"><section>{}</section><section>{}</section></main>"#,
            standings_table("East", &standings.east),
            standings_table("West", &standings.west)
        ),
    )
}

pub fn mlb_standings_page(standings: &MlbStandingsTable) -> String {
    layout(
        "MLB Standings",
        &format!(
            r#"<main class="page standings mlb-standings">{}</main>"#,
            standings
                .divisions
                .iter()
                .map(|division| mlb_standings_table(
                    &format!("{} {}", division.league, division.division),
                    &division.teams
                ))
                .collect::<String>()
        ),
    )
}

fn standings_table(title: &str, rows: &[StandingsTeam]) -> String {
    let table_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            vec![
                row.playoff_rank.to_string(),
                format!(
                    "{}{}",
                    team_logo_id(row.team_id, &row.team_name, "mini-logo"),
                    escape(&row.team_name)
                ),
                row.wins.to_string(),
                row.losses.to_string(),
                format_number(row.win_pct, 3),
                if row.conference_games_back == 0.0 {
                    "-".to_string()
                } else {
                    format_number(row.conference_games_back, 2)
                },
                format_number(row.points_pg, 2),
                format_number(row.opp_points_pg, 2),
                format_number(row.diff_points_pg, 2),
                row.home.clone(),
                row.road.clone(),
                row.last_ten.clone(),
                if row.current_streak < 0 {
                    format!("L{}", row.current_streak.abs())
                } else {
                    format!("W{}", row.current_streak.abs())
                },
            ]
        })
        .collect();
    format!(
        r#"<article class="panel"><h1>{}</h1>{}</article>"#,
        escape(title),
        sortable_table_with_options(
            &[
                "#", "Team", "W", "L", "%", "GB", "PPG", "OPPG", "DIFF", "HM", "RD", "L10", "STR"
            ],
            &table_rows,
            TableOptions {
                default_sort_index: Some(0),
                default_sort_dir: Some("asc"),
            },
        )
    )
}

fn mlb_standings_table(title: &str, rows: &[MlbStandingsTeam]) -> String {
    let table_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            vec![
                row.playoff_rank.to_string(),
                format!(
                    "{}{}",
                    mlb_team_logo(&row.team_tricode, &row.team_name, "mini-logo"),
                    escape(&row.team_name)
                ),
                row.wins.to_string(),
                row.losses.to_string(),
                row.win_pct.clone(),
                row.games_back.clone(),
                row.runs_scored.to_string(),
                row.runs_allowed.to_string(),
                row.run_diff.clone(),
                row.streak.clone(),
            ]
        })
        .collect();
    format!(
        r#"<article class="panel"><h1>{}</h1>{}</article>"#,
        escape(title),
        sortable_table_with_options(
            &[
                "#", "Team", "W", "L", "PCT", "GB", "RS", "RA", "DIFF", "STR"
            ],
            &table_rows,
            TableOptions {
                default_sort_index: Some(0),
                default_sort_dir: Some("asc"),
            },
        )
    )
}

pub fn player_page(stats: &PlayerStatsPage) -> String {
    let mut body = String::from(r#"<main class="page player">"#);
    if stats.tables.is_empty() {
        body.push_str(r#"<section class="center"><h1>No player stats available.</h1></section>"#);
    }
    for table in &stats.tables {
        body.push_str(&format!(
            r#"<article class="panel"><h1>{}</h1>{}</article>"#,
            escape(&table.name),
            render_table(table)
        ));
    }
    body.push_str("</main>");
    layout("NBA Player", &body)
}

fn render_table(table: &Table) -> String {
    let headers: Vec<&str> = table.headers.iter().map(String::as_str).collect();
    sortable_table(&headers, &table.rows)
}

pub fn sortable_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    sortable_table_with_options(headers, rows, TableOptions::default())
}

#[derive(Default)]
struct TableOptions {
    default_sort_index: Option<usize>,
    default_sort_dir: Option<&'static str>,
}

fn sortable_table_with_options(
    headers: &[&str],
    rows: &[Vec<String>],
    options: TableOptions,
) -> String {
    let mut attrs = String::new();
    if let Some(index) = options.default_sort_index {
        attrs.push_str(&format!(r#" data-default-sort-index="{index}""#));
    }
    if let Some(dir) = options.default_sort_dir {
        attrs.push_str(&format!(r#" data-default-sort-dir="{dir}""#));
    }
    let mut html = format!(r#"<div class="table-wrap"><table class="sortable"{attrs}><thead><tr>"#);
    for header in headers {
        html.push_str(&format!(
            r#"<th class="{}">{}</th>"#,
            cell_class(header),
            escape(header)
        ));
    }
    html.push_str("</tr></thead><tbody>");
    for row in rows {
        html.push_str("<tr>");
        for cell in row {
            html.push_str(&format!(
                r#"<td class="{}">{}</td>"#,
                cell_class(cell),
                cell
            ));
        }
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");
    html
}

fn game_status(game: &Game) -> String {
    if game.game_status == 1 && !game.game_time_utc.is_empty() {
        return format!(
            r#"<time data-local-game-time datetime="{}">{}</time>"#,
            escape_attr(&game.game_time_utc),
            escape(&game.game_status_text)
        );
    }
    escape(&game.game_status_text)
}

fn team_record(team: &Team) -> String {
    if team.display_record.is_empty() {
        format!("{}-{}", team.wins, team.losses)
    } else {
        team.display_record.clone()
    }
}

fn cell_class(value: &str) -> &'static str {
    if value.contains("<img") || value.chars().any(|c| c.is_alphabetic()) && !looks_numeric(value) {
        "text"
    } else {
        "num"
    }
}

fn looks_numeric(value: &str) -> bool {
    let text = value.trim();
    if text.is_empty() || text == "-" {
        return false;
    }
    text.parse::<f64>().is_ok()
        || text
            .strip_prefix(['+', '-'])
            .unwrap_or(text)
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, '.' | '%' | ','))
        || text.contains('-') && text.chars().all(|c| c.is_ascii_digit() || c == '-')
}

fn format_number(value: f64, decimals: usize) -> String {
    let formatted = format!("{value:.decimals$}");
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

fn team_logo(team: &Team, class: &str, league: League) -> String {
    match league {
        League::Nba => team_logo_id(team.team_id, &team.team_name, class),
        League::Mlb => mlb_team_logo(&team.team_tricode, &team.team_name, class),
    }
}

fn mlb_box_winner(game: &MlbBoxScore, is_home: bool) -> &'static str {
    if game.game_status != 3 {
        return "";
    }
    if (is_home && game.home_team.team.score > game.away_team.team.score)
        || (!is_home && game.away_team.team.score > game.home_team.team.score)
    {
        "<strong>W</strong>"
    } else {
        ""
    }
}

fn team_logo_id(team_id: i64, team_name: &str, class: &str) -> String {
    format!(
        r#"<img class="{class}" src="https://cdn.nba.com/logos/nba/{}/primary/L/logo.svg" alt="{}">"#,
        team_id,
        escape_attr(team_name)
    )
}

fn mlb_team_logo(team_tricode: &str, team_name: &str, class: &str) -> String {
    format!(
        r#"<img class="{class}" src="https://a.espncdn.com/i/teamlogos/mlb/500/{}.png" alt="{}">"#,
        escape_attr(&team_tricode.to_lowercase()),
        escape_attr(team_name)
    )
}

fn winner(game: &Game, is_home: bool) -> &'static str {
    if game.game_status != 3 {
        return "";
    }
    if (is_home && game.home_team.score > game.away_team.score)
        || (!is_home && game.away_team.score > game.home_team.score)
    {
        "<strong>W</strong>"
    } else {
        ""
    }
}

fn box_winner(game: &BoxScore, is_home: bool) -> &'static str {
    if game.game_status != 3 {
        return "";
    }
    if (is_home && game.home_team.team.score > game.away_team.team.score)
        || (!is_home && game.away_team.team.score > game.home_team.team.score)
    {
        "<strong>W</strong>"
    } else {
        ""
    }
}

fn stat_cell(value: i64, threshold: i64, good_when_high: bool) -> String {
    let class = if (good_when_high && value >= threshold) || (!good_when_high && value < threshold)
    {
        "num good"
    } else if !good_when_high && value >= threshold {
        "num bad"
    } else {
        "num"
    };
    format!(r#"<td class="{class}">{value}</td>"#)
}

fn weekday(n: u32) -> &'static str {
    ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"][n as usize]
}

pub fn escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn escape_attr(input: &str) -> String {
    escape(input).replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escaping_handles_html() {
        assert_eq!(escape("<b>&"), "&lt;b&gt;&amp;");
        assert_eq!(escape_attr("\"x\""), "&quot;x&quot;");
    }

    #[test]
    fn table_renderer_marks_tables_sortable() {
        let table = sortable_table(&["A"], &[vec!["1".to_string()]]);
        assert!(table.contains("table class=\"sortable\""));
    }
}
