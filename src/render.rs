use chrono::{Datelike, Days, NaiveDate, Utc};

use crate::{
    models::{
        BoxScore, BoxScoreTeam, Game, MlbBoxScore, MlbBoxScoreTeam, MlbStandingsTable,
        MlbStandingsTeam, NflBoxScore, NflBoxScoreTeam, NflStandingsTable, NflStandingsTeam,
        PlayerStatsPage, Scoreboard, StandingsTable, StandingsTeam, Table, Team,
    },
    stats,
};

#[derive(Clone, Copy)]
enum League {
    Nba,
    Mlb,
    Nfl,
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
  <link rel="manifest" href="/public/manifest.webmanifest">
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
  <div class="nav-section">
    <span class="nav-label">NBA</span>
    <a href="/nba/scoreboard" aria-label="NBA Scoreboard">Scoreboard</a>
    <a href="/nba/standings" aria-label="NBA Standings">Standings</a>
  </div>
  <div class="nav-section">
    <span class="nav-label">MLB</span>
    <a href="/mlb/scoreboard" aria-label="MLB Scoreboard">Scoreboard</a>
    <a href="/mlb/standings" aria-label="MLB Standings">Standings</a>
  </div>
  <div class="nav-section">
    <span class="nav-label">NFL</span>
    <a href="/nfl/scoreboard" aria-label="NFL Scoreboard">Scoreboard</a>
    <a href="/nfl/standings" aria-label="NFL Standings">Standings</a>
  </div>
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
        if let Some(game) = selected {
            html.push_str(&detail_nav(
                "/nba/scoreboard",
                &day.to_string(),
                &scoreboard.games,
                &game.game_id,
            ));
            html.push_str(&game_details(game));
        } else {
            html.push_str(&game_list(
                "/nba/scoreboard",
                &day.to_string(),
                &scoreboard.games,
                League::Nba,
            ));
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
        if let Some(game) = selected {
            let scoreboard_game = scoreboard
                .games
                .iter()
                .find(|scoreboard_game| scoreboard_game.game_id == game.game_id);
            html.push_str(&detail_nav(
                "/mlb/scoreboard",
                &day.to_string(),
                &scoreboard.games,
                &game.game_id,
            ));
            html.push_str(&mlb_game_details(game, scoreboard_game));
        } else {
            html.push_str(&game_list(
                "/mlb/scoreboard",
                &day.to_string(),
                &scoreboard.games,
                League::Mlb,
            ));
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

pub fn nfl_scoreboard_page(
    week: i64,
    scoreboard: &Scoreboard,
    selected: Option<&NflBoxScore>,
) -> String {
    let mut html = String::from(r#"<main class="page">"#);
    html.push_str(&week_nav(week, "/nfl/scoreboard"));
    if scoreboard.games.is_empty() {
        html.push_str(r#"<section class="center"><h1>No Games Scheduled</h1></section>"#);
    } else {
        let class = if selected.is_some() {
            "scoreboard has-game"
        } else {
            "scoreboard"
        };
        html.push_str(&format!(r#"<section class="{class}">"#));
        if let Some(game) = selected {
            html.push_str(&detail_nav(
                "/nfl/scoreboard",
                &week.to_string(),
                &scoreboard.games,
                &game.game_id,
            ));
            html.push_str(&nfl_game_details(game));
        } else {
            html.push_str(&game_list(
                "/nfl/scoreboard",
                &week.to_string(),
                &scoreboard.games,
                League::Nfl,
            ));
        }
        html.push_str("</section>");
    }
    html.push_str("</main>");
    layout(
        if selected.is_some() {
            "NFL Game"
        } else {
            "NFL Scoreboard"
        },
        &html,
    )
}

fn week_nav(week: i64, base_path: &str) -> String {
    let mut html = String::from(r#"<div class="date-nav week-nav">"#);
    let prev = (week - 1).max(1);
    let next = (week + 1).min(23);
    let start = (week - 3).clamp(1, 17);
    let end = (start + 6).min(23);
    html.push_str(&format!(
        r#"<a class="button" href="{base_path}/{prev}">Prev</a>"#
    ));
    for w in start..=end {
        let class = if w == week { "button active" } else { "button" };
        html.push_str(&format!(
            r#"<a class="{class}" href="{base_path}/{w}">{}</a>"#,
            nfl_week_label(w)
        ));
    }
    html.push_str(&format!(
        r#"<a class="button" href="{base_path}/{next}">Next</a>"#
    ));
    html.push_str("</div>");
    html
}

fn nfl_week_label(week: i64) -> String {
    match week {
        1..=18 => format!("Week {week}"),
        19 => "Wild Card".to_string(),
        20 => "Divisional".to_string(),
        21 => "Conf Champ".to_string(),
        22 => "Pro Bowl".to_string(),
        23 => "Super Bowl".to_string(),
        _ => format!("Week {week}"),
    }
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
        let visibility_class = match offset.abs() {
            0 => "date-current",
            1 => "date-near",
            _ => "date-wide",
        };
        let class = if d == day {
            format!("button active {visibility_class}")
        } else {
            format!("button {visibility_class}")
        };
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

fn game_list(base_path: &str, bucket: &str, games: &[Game], league: League) -> String {
    let mut html = String::from(r#"<div class="game-list">"#);
    let all_completed = games.iter().all(|game| game.game_status == 3);
    for game in games {
        html.push_str(&format!(
            r#"<a class="game-link" href="{}/{}/game/{}">{}</a>"#,
            base_path,
            escape_attr(bucket),
            escape_attr(&game.game_id),
            game_summary(game, !all_completed, league)
        ));
    }
    html.push_str("</div>");
    html
}

fn detail_nav(base_path: &str, bucket: &str, games: &[Game], selected_game_id: &str) -> String {
    let selected_index = games
        .iter()
        .position(|game| game.game_id == selected_game_id);
    let prev = selected_index
        .and_then(|index| index.checked_sub(1))
        .and_then(|index| games.get(index));
    let current = selected_index.and_then(|index| games.get(index));
    let next = selected_index.and_then(|index| games.get(index + 1));

    let mut html = String::from(r#"<div class="detail-actions">"#);
    if let Some(prev) = prev {
        html.push_str(&game_step_link(base_path, bucket, prev, "prev"));
    }
    if let Some(current) = current {
        html.push_str(&game_step_link(base_path, bucket, current, "current"));
    }
    if let Some(next) = next {
        html.push_str(&game_step_link(base_path, bucket, next, "next"));
    }
    html.push_str("</div>");
    html
}

fn game_step_link(base_path: &str, bucket: &str, game: &Game, direction: &str) -> String {
    let status = compact_game_status(game);
    let arrow = if direction == "prev" { "&lt;" } else { "&gt;" };
    let arrow_first = direction == "prev";
    let has_arrow = direction != "current";
    let arrow_html = format!(r#"<span class="step-arrow">{arrow}</span>"#);
    let status_html = if status.is_empty() {
        String::new()
    } else {
        format!(r#"<span class="step-status">{}</span>"#, escape(&status))
    };
    let away_winner = game_winner_marker(game, false);
    let home_winner = game_winner_marker(game, true);
    let away_record = team_record(&game.away_team);
    let home_record = team_record(&game.home_team);
    let content = format!(
        r#"{}{status_html}<span class="step-team"><span>{} <small>({})</small>{}</span><strong>{}</strong></span><span class="step-team"><span>{} <small>({})</small>{}</span><strong>{}</strong></span>{}"#,
        if arrow_first && has_arrow {
            arrow_html.as_str()
        } else {
            ""
        },
        escape(&game.away_team.team_tricode),
        escape(&away_record),
        away_winner,
        game.away_team.score,
        escape(&game.home_team.team_tricode),
        escape(&home_record),
        home_winner,
        game.home_team.score,
        if !arrow_first && has_arrow {
            arrow_html.as_str()
        } else {
            ""
        }
    );
    if direction == "current" {
        format!(r#"<div class="game-step current" aria-current="true">{content}</div>"#)
    } else {
        format!(
            r#"<a class="button game-step {direction}" href="{base_path}/{}/game/{}">{content}</a>"#,
            escape_attr(bucket),
            escape_attr(&game.game_id)
        )
    }
}

fn game_winner_marker(game: &Game, is_home: bool) -> &'static str {
    if game.game_status != 3 {
        return "";
    }
    if (is_home && game.home_team.score > game.away_team.score)
        || (!is_home && game.away_team.score > game.home_team.score)
    {
        " W"
    } else {
        ""
    }
}

fn compact_game_status(game: &Game) -> String {
    if game.game_status == 2 && game.period > 0 {
        let period = match game.period {
            1 => "1Q".to_string(),
            2 => "2Q".to_string(),
            3 => "3Q".to_string(),
            4 => "4Q".to_string(),
            period => format!("{}OT", period - 4),
        };
        let clock = game.game_clock.trim();
        if clock.is_empty() || matches!(clock, "0.0" | "0:00") {
            return period;
        }
        return format!("{period} {clock}");
    }
    game.game_status_text.trim().to_string()
}

fn game_summary(game: &Game, show_status: bool, league: League) -> String {
    let class = match league {
        League::Mlb => "game-card mlb-game-card",
        League::Nba | League::Nfl => "game-card period-game-card",
    };
    let mut html = format!(r#"<table class="{class}"><thead><tr><th></th>"#);
    match league {
        League::Nba | League::Nfl => {
            for period in 1..=period_column_count(game) {
                html.push_str(&format!("<th>{period}</th>"));
            }
            html.push_str("<th>T</th></tr></thead><tbody>");
        }
        League::Mlb => html.push_str("<th>R</th><th>H</th><th>E</th></tr></thead><tbody>"),
    }
    html.push_str(&team_summary_row(game, &game.away_team, false, league));
    html.push_str(&team_summary_row(game, &game.home_team, true, league));
    if show_status {
        let colspan = match league {
            League::Nba | League::Nfl => period_column_count(game) + 2,
            League::Mlb => 4,
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
    html.push_str(&format!(
        r#"<span class="team-label">{}<span class="team-name" title="{}"><span>{}</span> <small>({})</small></span>{}</span>"#,
        team_logo(team, "mini-logo", league),
        escape_attr(&format!("{} {}", team.team_city, team.team_name)),
        escape(&team.team_tricode),
        escape(&team_record(team)),
        winner(game, is_home)
    ));
    html.push_str("</th>");
    match league {
        League::Nba | League::Nfl => {
            for period in 1..=period_column_count(game) {
                html.push_str(&format!("<td>{}</td>", period_score(team, period)));
            }
            html.push_str(&format!(
                r#"<td class="score-total">{}</td></tr>"#,
                team.score
            ));
        }
        League::Mlb if game.game_status == 1 => {
            html.push_str(r#"<td class="score-total">-</td><td>-</td><td>-</td></tr>"#);
        }
        League::Mlb => {
            html.push_str(&format!(
                r#"<td class="score-total">{}</td><td>{}</td><td>{}</td></tr>"#,
                team.score, team.hits, team.errors
            ));
        }
    }
    html
}

fn period_column_count(game: &Game) -> i64 {
    game.away_team
        .periods
        .iter()
        .chain(game.home_team.periods.iter())
        .map(|period| period.period)
        .max()
        .unwrap_or(0)
        .max(4)
}

fn period_score(team: &Team, period: i64) -> String {
    team.periods
        .iter()
        .find(|score| score.period == period)
        .map(|period| {
            if period.score == 0 {
                "-".to_string()
            } else {
                period.score.to_string()
            }
        })
        .unwrap_or_default()
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
    html.push_str("</section>");
    html
}

fn nfl_game_details(game: &NflBoxScore) -> String {
    let mut html = String::from(r#"<section class="details">"#);
    html.push_str(&nfl_team_stats_comparison(game));
    html.push_str(&nfl_team_game_details(game, &game.away_team, false));
    html.push_str(&nfl_team_game_details(game, &game.home_team, true));
    html.push_str("</section>");
    html
}

fn nfl_team_game_details(game: &NflBoxScore, team: &NflBoxScoreTeam, is_home: bool) -> String {
    let mut html = String::from(r#"<article class="team-details">"#);
    html.push_str("<h1>");
    html.push_str(&team_logo(&team.team, "logo", League::Nfl));
    html.push_str(&format!(
        "{} {} <strong>{}</strong> {}",
        escape(&team.team.team_city),
        escape(&team.team.team_name),
        team.team.score,
        nfl_box_winner(game, is_home)
    ));
    html.push_str("</h1>");
    for table in &team.player_stats {
        html.push_str(&format!(
            r#"<section class="box-score-group"><h2>{}</h2>{}</section>"#,
            escape(&table.name),
            render_table(table)
        ));
    }
    html.push_str("</article>");
    html
}

fn nfl_team_stats_comparison(game: &NflBoxScore) -> String {
    let away_label = &game.away_team.team.team_tricode;
    let home_label = &game.home_team.team.team_tricode;
    let mut rows = Vec::new();
    for row in &game.away_team.team_stats.rows {
        let Some(stat) = row.first() else {
            continue;
        };
        let away_value = row.get(1).cloned().unwrap_or_default();
        let home_value = game
            .home_team
            .team_stats
            .rows
            .iter()
            .find(|home_row| home_row.first() == Some(stat))
            .and_then(|home_row| home_row.get(1))
            .cloned()
            .unwrap_or_default();
        rows.push((stat.clone(), away_value, home_value));
    }
    for row in &game.home_team.team_stats.rows {
        let Some(stat) = row.first() else {
            continue;
        };
        if rows.iter().any(|(existing, _, _)| existing == stat) {
            continue;
        }
        rows.push((
            stat.clone(),
            String::new(),
            row.get(1).cloned().unwrap_or_default(),
        ));
    }

    let mut table_rows = Vec::new();
    for (stat, away_value, home_value) in rows {
        let (away_class, home_class) = nfl_stat_classes(&stat, &away_value, &home_value);
        table_rows.push(vec![
            table_cell(escape(&stat)),
            table_cell_with_class(
                escape(&away_value),
                away_class.strip_prefix("num ").unwrap_or(""),
            ),
            table_cell_with_class(
                escape(&home_value),
                home_class.strip_prefix("num ").unwrap_or(""),
            ),
        ]);
    }
    format!(
        r#"<article class="team-details"><h1>Team Stats</h1>{}</article>"#,
        sortable_table_cells(
            &["Stat", away_label, home_label],
            &table_rows,
            TableOptions::default()
        )
    )
}

fn nfl_stat_classes(
    stat: &str,
    away_value: &str,
    home_value: &str,
) -> (&'static str, &'static str) {
    let Some(away_num) = nfl_stat_value(away_value) else {
        return ("num", "num");
    };
    let Some(home_num) = nfl_stat_value(home_value) else {
        return ("num", "num");
    };
    if (away_num - home_num).abs() < f64::EPSILON {
        return ("num", "num");
    }
    let away_better = if nfl_lower_is_better(stat) {
        away_num < home_num
    } else {
        away_num > home_num
    };
    if away_better {
        ("num good", "num")
    } else {
        ("num", "num good")
    }
}

fn nfl_lower_is_better(stat: &str) -> bool {
    let stat = stat.to_ascii_lowercase();
    stat.contains("turnover")
        || stat.contains("interception")
        || stat.contains("fumble")
        || stat.contains("penalt")
        || stat.contains("sacks-yards lost")
}

fn nfl_stat_value(value: &str) -> Option<f64> {
    let value = value.trim();
    if value.is_empty() || value == "-" {
        return None;
    }
    if let Some((made, attempted)) = value.split_once(['-', '/']) {
        let made = made.parse::<f64>().ok()?;
        let attempted = attempted.parse::<f64>().ok()?;
        if attempted == 0.0 {
            return Some(0.0);
        }
        return Some(made / attempted);
    }
    if let Some((minutes, seconds)) = value.split_once(':') {
        let minutes = minutes.parse::<f64>().ok()?;
        let seconds = seconds.parse::<f64>().ok()?;
        return Some(minutes * 60.0 + seconds);
    }
    value.trim_end_matches('%').parse::<f64>().ok()
}

fn mlb_game_details(game: &MlbBoxScore, scoreboard_game: Option<&Game>) -> String {
    let mut html = String::from(r#"<section class="details">"#);
    html.push_str(&mlb_line_score(game, scoreboard_game));
    html.push_str(&mlb_team_game_details(game, &game.away_team, false));
    html.push_str(&mlb_team_game_details(game, &game.home_team, true));
    html.push_str("</section>");
    html
}

fn mlb_line_score(game: &MlbBoxScore, scoreboard_game: Option<&Game>) -> String {
    let away_team = if !has_usable_line_score(&game.away_team.team) {
        scoreboard_game.map(|game| &game.away_team)
    } else {
        Some(&game.away_team.team)
    };
    let home_team = if !has_usable_line_score(&game.home_team.team) {
        scoreboard_game.map(|game| &game.home_team)
    } else {
        Some(&game.home_team.team)
    };
    let Some(away_team) = away_team else {
        return String::new();
    };
    let Some(home_team) = home_team else {
        return String::new();
    };

    let mut html = String::from(
        r#"<article class="team-details line-score"><h1>Line Score</h1><div class="table-wrap"><table class="game-card"><thead><tr><th></th>"#,
    );
    for period in &away_team.periods {
        html.push_str(&format!("<th>{}</th>", period.period));
    }
    html.push_str("<th>R</th><th>H</th><th>E</th></tr></thead><tbody>");
    html.push_str(&mlb_line_score_row(away_team));
    html.push_str(&mlb_line_score_row(home_team));
    html.push_str("</tbody></table></div></article>");
    html
}

fn has_usable_line_score(team: &Team) -> bool {
    !team.periods.is_empty()
        && team
            .periods
            .iter()
            .enumerate()
            .all(|(index, period)| period.period == index as i64 + 1)
}

fn mlb_line_score_row(team: &Team) -> String {
    let mut html = String::from("<tr><th>");
    html.push_str(&team_logo(team, "mini-logo", League::Mlb));
    html.push_str(&format!(
        r#"<span title="{}">{}</span>"#,
        escape_attr(&format!("{} {}", team.team_city, team.team_name)),
        escape(&team.team_tricode)
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
    html.push_str(&format!(
        r#"<td class="score-total">{}</td><td>{}</td><td>{}</td></tr>"#,
        team.score, team.hits, team.errors
    ));
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
    let mut rows = Vec::new();
    for player in team.players.iter().filter(|p| p.played) {
        let s = &player.statistics;
        rows.push(vec![
            table_cell(format!(
                r#"<a href="/nba/player/{}">{}{}</a>"#,
                player.person_id,
                escape(&player.name),
                if player.starter { "*" } else { "" }
            )),
            table_cell(s.minutes.to_string()),
            stat_cell(s.points, 20, true),
            stat_cell(s.rebounds_total, 10, true),
            stat_cell(s.assists, 8, true),
            table_cell(format_number(
                stats::pie(s, &team.statistics, &other.statistics) as f64,
                2,
            )),
            table_cell(format!(
                "{}-{}",
                s.field_goals_made, s.field_goals_attempted
            )),
            table_cell(format!(
                "{}-{}",
                s.three_pointers_made, s.three_pointers_attempted
            )),
            table_cell(format!(
                "{}-{}",
                s.free_throws_made, s.free_throws_attempted
            )),
            table_cell(
                stats::points_per_shot(s)
                    .map(|v| format!("{v:.2}"))
                    .unwrap_or_default(),
            ),
            stat_cell(s.turnovers, 3, false),
            stat_cell(s.steals, 3, true),
            stat_cell(s.blocks, 3, true),
            stat_cell(s.fouls_personal, 5, false),
            table_cell(s.plus_minus_points.to_string()),
            table_cell(
                stats::usage_rate(s, &team.statistics)
                    .map(|v| format_number(v as f64, 2))
                    .unwrap_or_default(),
            ),
        ]);
    }
    sortable_table_cells(
        &[
            "Name", "MIN", "PTS", "RB", "AS", "PIE", "FG", "3P", "FT", "PPS", "TO", "ST", "BK",
            "PF", "+/-", "USG",
        ],
        &rows,
        TableOptions {
            class: Some("box-score-table"),
            sort_group: Some("box-score"),
            default_sort_index: Some(5),
            default_sort_dir: Some("desc"),
            ..TableOptions::default()
        },
    )
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

pub fn nfl_standings_page(standings: &NflStandingsTable) -> String {
    layout(
        "NFL Standings",
        &format!(
            r#"<main class="page standings nfl-standings">{}</main>"#,
            standings
                .divisions
                .iter()
                .map(|division| nfl_standings_table(
                    &format!("{} {}", division.conference, division.division),
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
                ..TableOptions::default()
            },
        )
    )
}

fn nfl_standings_table(title: &str, rows: &[NflStandingsTeam]) -> String {
    let table_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            vec![
                row.playoff_rank.to_string(),
                format!(
                    "{}{}",
                    nfl_team_logo(&row.team_tricode, &row.team_name, "mini-logo"),
                    escape(&row.team_name)
                ),
                row.wins.to_string(),
                row.losses.to_string(),
                row.ties.to_string(),
                row.win_pct.clone(),
                row.games_back.clone(),
                row.points_for.to_string(),
                row.points_against.to_string(),
                row.point_diff.clone(),
                row.home.clone(),
                row.road.clone(),
                row.division_record.clone(),
                row.conference_record.clone(),
                row.streak.clone(),
            ]
        })
        .collect();
    format!(
        r#"<article class="panel"><h1>{}</h1>{}</article>"#,
        escape(title),
        sortable_table_with_options(
            &[
                "#", "Team", "W", "L", "T", "PCT", "GB", "PF", "PA", "DIFF", "HM", "RD", "DIV",
                "CONF", "STR"
            ],
            &table_rows,
            TableOptions {
                default_sort_index: Some(0),
                default_sort_dir: Some("asc"),
                ..TableOptions::default()
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
                ..TableOptions::default()
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
    class: Option<&'static str>,
    sort_group: Option<&'static str>,
    default_sort_index: Option<usize>,
    default_sort_dir: Option<&'static str>,
}

struct TableCell {
    html: String,
    class: &'static str,
}

fn table_cell(html: impl Into<String>) -> TableCell {
    TableCell {
        html: html.into(),
        class: "",
    }
}

fn table_cell_with_class(html: impl Into<String>, class: &'static str) -> TableCell {
    TableCell {
        html: html.into(),
        class,
    }
}

fn sortable_table_with_options(
    headers: &[&str],
    rows: &[Vec<String>],
    options: TableOptions,
) -> String {
    let cells: Vec<Vec<TableCell>> = rows
        .iter()
        .map(|row| row.iter().map(|cell| table_cell(cell.clone())).collect())
        .collect();
    sortable_table_cells(headers, &cells, options)
}

fn sortable_table_cells(
    headers: &[&str],
    rows: &[Vec<TableCell>],
    options: TableOptions,
) -> String {
    let mut attrs = String::new();
    let class = match options.class {
        Some(class) => format!("sortable {class}"),
        None => "sortable".to_string(),
    };
    if let Some(group) = options.sort_group {
        attrs.push_str(&format!(r#" data-sort-group="{group}""#));
    }
    if let Some(index) = options.default_sort_index {
        attrs.push_str(&format!(r#" data-default-sort-index="{index}""#));
    }
    if let Some(dir) = options.default_sort_dir {
        attrs.push_str(&format!(r#" data-default-sort-dir="{dir}""#));
    }
    let column_classes = table_column_classes(headers, rows);
    let mut html = format!(r#"<div class="table-wrap"><table class="{class}"{attrs}><thead><tr>"#);
    for (index, header) in headers.iter().enumerate() {
        html.push_str(&format!(
            r#"<th class="{}">{}</th>"#,
            column_classes.get(index).copied().unwrap_or("text"),
            escape(header)
        ));
    }
    html.push_str("</tr></thead><tbody>");
    for row in rows {
        html.push_str("<tr>");
        for (index, cell) in row.iter().enumerate() {
            let base_class = column_classes.get(index).copied().unwrap_or("text");
            let class = table_cell_class(base_class, cell.class);
            html.push_str(&format!(r#"<td class="{class}">{}</td>"#, cell.html));
        }
        html.push_str("</tr>");
    }
    html.push_str("</tbody></table></div>");
    html
}

fn table_column_classes(headers: &[&str], rows: &[Vec<TableCell>]) -> Vec<&'static str> {
    headers
        .iter()
        .enumerate()
        .map(|(index, header)| column_class(header, rows.iter().filter_map(|row| row.get(index))))
        .collect()
}

fn column_class<'a>(header: &str, cells: impl Iterator<Item = &'a TableCell>) -> &'static str {
    if header_implies_text(header) {
        return "text";
    }
    let mut saw_value = false;
    for cell in cells {
        let value = cell.html.trim();
        if value.is_empty() || value == "-" {
            continue;
        }
        saw_value = true;
        if value.contains('<') || !looks_numeric(value) {
            return "text";
        }
    }
    if saw_value { "num" } else { cell_class(header) }
}

fn header_implies_text(header: &str) -> bool {
    matches!(
        header.to_ascii_lowercase().as_str(),
        "name" | "team" | "player" | "stat" | "date" | "split"
    )
}

fn table_cell_class(base: &str, extra: &str) -> String {
    if extra.is_empty() {
        base.to_string()
    } else {
        format!("{base} {extra}")
    }
}

fn game_status(game: &Game) -> String {
    if game.game_status == 1 && !game_has_started(game) && !game.game_time_utc.is_empty() {
        return format!(
            r#"<time data-local-game-time datetime="{}">{}</time>"#,
            escape_attr(&game.game_time_utc),
            escape(&game.game_status_text)
        );
    }
    escape(&game.game_status_text)
}

fn game_has_started(game: &Game) -> bool {
    game.period > 0
        || game.home_team.score > 0
        || game.away_team.score > 0
        || !game.home_team.periods.is_empty()
        || !game.away_team.periods.is_empty()
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
        League::Nfl => nfl_team_logo(&team.team_tricode, &team.team_name, class),
    }
}

fn nfl_box_winner(game: &NflBoxScore, is_home: bool) -> &'static str {
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

fn nfl_team_logo(team_tricode: &str, team_name: &str, class: &str) -> String {
    format!(
        r#"<img class="{class}" src="https://a.espncdn.com/i/teamlogos/nfl/500/{}.png" alt="{}">"#,
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

fn stat_cell(value: i64, threshold: i64, good_when_high: bool) -> TableCell {
    let class = if (good_when_high && value >= threshold) || (!good_when_high && value < threshold)
    {
        "good"
    } else if !good_when_high && value >= threshold {
        "bad"
    } else {
        ""
    };
    table_cell_with_class(value.to_string(), class)
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

    #[test]
    fn table_renderer_aligns_headers_from_column_values() {
        let table = sortable_table(
            &["Name", "PTS", "Date"],
            &[vec![
                "Jaylen Brown".to_string(),
                "31".to_string(),
                "2026-04-26".to_string(),
            ]],
        );

        assert!(table.contains(
            r#"<th class="text">Name</th><th class="num">PTS</th><th class="text">Date</th>"#
        ));
        assert!(table.contains(
            r#"<td class="text">Jaylen Brown</td><td class="num">31</td><td class="text">2026-04-26</td>"#
        ));
    }

    #[test]
    fn started_games_do_not_localize_status_as_tip_time() {
        let game = Game {
            game_id: "1".to_string(),
            game_status: 1,
            game_status_text: "Halftime".to_string(),
            period: 2,
            game_clock: "0.0".to_string(),
            game_time_utc: "2026-04-28T02:30Z".to_string(),
            home_team: Team {
                team_id: 1,
                team_name: "Nuggets".to_string(),
                team_city: "Denver".to_string(),
                team_tricode: "DEN".to_string(),
                wins: 0,
                losses: 0,
                display_record: String::new(),
                score: 60,
                hits: 0,
                errors: 0,
                periods: Vec::new(),
            },
            away_team: Team {
                team_id: 2,
                team_name: "Timberwolves".to_string(),
                team_city: "Minnesota".to_string(),
                team_tricode: "MIN".to_string(),
                wins: 0,
                losses: 0,
                display_record: String::new(),
                score: 51,
                hits: 0,
                errors: 0,
                periods: Vec::new(),
            },
            home_leaders: Default::default(),
            away_leaders: Default::default(),
        };

        assert_eq!(game_status(&game), "Halftime");
    }
}
