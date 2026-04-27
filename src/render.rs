use chrono::{Datelike, Days, NaiveDate, Utc};

use crate::{
    models::{
        BoxScore, BoxScoreTeam, Game, PlayerStatsPage, Scoreboard, StandingsTable, StandingsTeam,
        Table, Team,
    },
    stats,
};

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
  <a href="/mlb/scoreboard">MLB</a>
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
    let mut html = String::from(r#"<main class="page">"#);
    html.push_str(&date_nav(day));
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
                game_summary(game, !all_completed)
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

fn date_nav(day: NaiveDate) -> String {
    let mut html = String::from(r#"<div class="date-nav">"#);
    html.push_str(&format!(
        r#"<a class="button" href="/nba/scoreboard/{}">Prev</a>"#,
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
            r#"<a class="{class}" href="/nba/scoreboard/{d}">{}</a>"#,
            escape(&label)
        ));
    }
    html.push_str(&format!(
        r#"<a class="button" href="/nba/scoreboard/{}">Next</a>"#,
        day.checked_add_days(Days::new(1)).unwrap_or(day)
    ));
    html.push_str("</div>");
    html
}

fn game_summary(game: &Game, show_status: bool) -> String {
    let mut html = String::from(r#"<table class="game-card"><thead><tr><th></th>"#);
    for period in &game.away_team.periods {
        html.push_str(&format!("<th>{}</th>", period.period));
    }
    html.push_str("<th>T</th></tr></thead><tbody>");
    html.push_str(&team_summary_row(game, &game.away_team, false));
    html.push_str(&team_summary_row(game, &game.home_team, true));
    if show_status {
        let colspan = game.away_team.periods.len() + 2;
        html.push_str(&format!(
            r#"<tr><th class="status" colspan="{colspan}">{}</th></tr>"#,
            escape(&game.game_status_text)
        ));
    }
    html.push_str("</tbody></table>");
    html
}

fn team_summary_row(game: &Game, team: &Team, is_home: bool) -> String {
    let mut html = String::from("<tr><th>");
    html.push_str(&team_logo(team, "mini-logo"));
    html.push_str(&format!(
        r#"<span title="{}">{}</span> <small>({}-{})</small> {}"#,
        escape_attr(&format!("{} {}", team.team_city, team.team_name)),
        escape(&team.team_tricode),
        team.wins,
        team.losses,
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
    html.push_str(&format!("<td>{}</td></tr>", team.score));
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

fn team_game_details(
    game: &BoxScore,
    team: &BoxScoreTeam,
    other: &BoxScoreTeam,
    is_home: bool,
) -> String {
    let mut html = String::from(r#"<article class="team-details">"#);
    html.push_str("<h1>");
    html.push_str(&team_logo(&team.team, "logo"));
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
        r#"<div class="table-wrap"><table class="sortable"><thead><tr><th>Name</th><th>MIN</th><th>PTS</th><th>RB</th><th>AS</th><th>PIE</th><th>FG</th><th>3P</th><th>FT</th><th>PPS</th><th>TO</th><th>ST</th><th>BK</th><th>PF</th><th>+/-</th><th>USG</th></tr></thead><tbody>"#,
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
            stats::pie(s, &team.statistics, &other.statistics)
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
                .map(|v| v.to_string())
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
                row.win_pct.to_string(),
                if row.conference_games_back == 0.0 {
                    "-".to_string()
                } else {
                    row.conference_games_back.to_string()
                },
                row.points_pg.to_string(),
                row.opp_points_pg.to_string(),
                row.diff_points_pg.to_string(),
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
        sortable_table(
            &[
                "#", "Team", "W", "L", "%", "GB", "PPG", "OPPG", "DIFF", "HM", "RD", "L10", "STR"
            ],
            &table_rows
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
    let mut html = String::from(r#"<div class="table-wrap"><table class="sortable"><thead><tr>"#);
    for header in headers {
        html.push_str(&format!("<th>{}</th>", escape(header)));
    }
    html.push_str("</tr></thead><tbody>");
    for row in rows {
        html.push_str("<tr>");
        for cell in row {
            html.push_str(&format!("<td>{}</td>", cell));
        }
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");
    html
}

fn team_logo(team: &Team, class: &str) -> String {
    team_logo_id(team.team_id, &team.team_name, class)
}

fn team_logo_id(team_id: i64, team_name: &str, class: &str) -> String {
    format!(
        r#"<img class="{class}" src="https://cdn.nba.com/logos/nba/{}/primary/L/logo.svg" alt="{}">"#,
        team_id,
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
        "good"
    } else if !good_when_high && value >= threshold {
        "bad"
    } else {
        ""
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
