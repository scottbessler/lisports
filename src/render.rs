use chrono::{Datelike, Days, NaiveDate, SecondsFormat, Utc};

use crate::{
    clients::LIVE_DATA_CACHE_SECONDS,
    models::{
        BoxScore, BoxScoreTeam, BracketMatch, BracketSlot, BracketTable, Game, MlbBoxScore,
        MlbBoxScoreTeam, MlbStandingsTable, MlbStandingsTeam, NflBoxScore, NflBoxScoreTeam,
        NflStandingsTable, NflStandingsTeam, NhlBoxScore, NhlBoxScoreTeam, NhlStandingsTable,
        NhlStandingsTeam, Player, PlayerStatsPage, Scoreboard, SoccerBoxScore, SoccerBoxScoreTeam,
        SoccerEvent, SoccerStandingsTable, SoccerStandingsTeam, StandingsTable, StandingsTeam,
        Statistics, Table, Team, TeamPage, TeamStatistics,
    },
    stats,
};

#[derive(Clone, Copy)]
enum League {
    Nba,
    Wnba,
    Mlb,
    Nfl,
    Nhl,
    Soccer,
    Nwsl,
}

pub fn layout(title: &str, body: &str) -> String {
    layout_with_refresh(title, body, None)
}

fn layout_with_refresh(title: &str, body: &str, refresh_at: Option<&str>) -> String {
    let refresh_attr = refresh_at
        .map(|value| format!(r#" data-refresh-at="{}""#, escape_attr(value)))
        .unwrap_or_default();
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
<body{}>
  {}
  {}
  <script src="/public/table-sort.js"></script>
  <script>
    document.querySelectorAll('[data-nav]').forEach((nav) => {{
      const toggle = nav.querySelector('.nav-toggle');
      const links = nav.querySelector('.nav-links');
      if (!toggle || !links) {{
        return;
      }}
      toggle.addEventListener('click', () => {{
        const isOpen = nav.classList.toggle('open');
        toggle.setAttribute('aria-expanded', String(isOpen));
      }});
    }});
  </script>
</body>
</html>"#,
        escape(title),
        refresh_attr,
        nav(),
        body
    )
}

pub fn nav() -> &'static str {
    r#"<nav class="nav" data-nav>
  <div class="brand">LiSports</div>
  <button class="nav-toggle" type="button" aria-controls="primary-nav" aria-expanded="false">
    <span class="menu-icon" aria-hidden="true"></span>
    <span>LiSports</span>
  </button>
  <div class="nav-links" id="primary-nav">
    <div class="nav-section">
      <span class="nav-label">NBA</span>
      <a href="/nba/scoreboard" aria-label="NBA Scoreboard">Scoreboard</a>
      <a href="/nba/standings" aria-label="NBA Standings">Standings</a>
    </div>
    <div class="nav-section">
      <span class="nav-label">WNBA</span>
      <a href="/wnba/scoreboard" aria-label="WNBA Scoreboard">Scoreboard</a>
      <a href="/wnba/standings" aria-label="WNBA Standings">Standings</a>
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
    <div class="nav-section">
      <span class="nav-label">NHL</span>
      <a href="/nhl/scoreboard" aria-label="NHL Scoreboard">Scoreboard</a>
      <a href="/nhl/standings" aria-label="NHL Standings">Standings</a>
    </div>
    <div class="nav-section nav-section-wide">
      <span class="nav-label">World&nbsp;Cup</span>
      <a href="/worldcup/scoreboard" aria-label="World Cup Scoreboard">Scoreboard</a>
      <a href="/worldcup/standings" aria-label="World Cup Standings">Standings</a>
      <a href="/worldcup/bracket" aria-label="World Cup Bracket">Bracket</a>
    </div>
    <div class="nav-section">
      <span class="nav-label">NWSL</span>
      <a href="/nwsl/scoreboard" aria-label="NWSL Scoreboard">Scoreboard</a>
      <a href="/nwsl/standings" aria-label="NWSL Standings">Standings</a>
    </div>
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
    basketball_scoreboard_page(day, scoreboard, selected, None)
}

pub fn scoreboard_page_with_today(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&BoxScore>,
    today_day: NaiveDate,
) -> String {
    basketball_scoreboard_page(day, scoreboard, selected, Some(today_day))
}

pub fn wnba_scoreboard_page_with_today(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&BoxScore>,
    today_day: NaiveDate,
) -> String {
    basketball_scoreboard_page_with_league(
        day,
        scoreboard,
        selected,
        Some(today_day),
        BasketballPage {
            league: League::Wnba,
            base_path: "/wnba/scoreboard",
            title: "WNBA",
        },
    )
}

fn basketball_scoreboard_page(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&BoxScore>,
    today_day: Option<NaiveDate>,
) -> String {
    basketball_scoreboard_page_with_league(
        day,
        scoreboard,
        selected,
        today_day,
        BasketballPage {
            league: League::Nba,
            base_path: "/nba/scoreboard",
            title: "NBA",
        },
    )
}

struct BasketballPage {
    league: League,
    base_path: &'static str,
    title: &'static str,
}

fn basketball_scoreboard_page_with_league(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&BoxScore>,
    today_day: Option<NaiveDate>,
    page: BasketballPage,
) -> String {
    let content = if scoreboard.games.is_empty() {
        None
    } else {
        let mut content = String::new();
        if let Some(game) = selected {
            content.push_str(&detail_nav(
                page.base_path,
                &day.to_string(),
                &scoreboard.games,
                &game.game_id,
                page.league,
            ));
            content.push_str(&game_details(game, page.league));
        } else {
            content.push_str(&game_list(
                page.base_path,
                &day.to_string(),
                &scoreboard.games,
                page.league,
            ));
        }
        Some(content)
    };
    let refresh_at = basketball_refresh_at(scoreboard, selected);
    let title = if selected.is_some() {
        format!("{} Game", page.title)
    } else {
        format!("{} Scoreboard", page.title)
    };
    scoreboard_shell(
        date_nav(day, page.base_path, today_day),
        selected.is_some(),
        content,
        &title,
        refresh_at.as_deref(),
    )
}

pub fn mlb_scoreboard_page(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&MlbBoxScore>,
) -> String {
    mlb_scoreboard_page_with_today_marker(day, scoreboard, selected, None)
}

pub fn mlb_scoreboard_page_with_today(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&MlbBoxScore>,
    today_day: NaiveDate,
) -> String {
    mlb_scoreboard_page_with_today_marker(day, scoreboard, selected, Some(today_day))
}

fn mlb_scoreboard_page_with_today_marker(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&MlbBoxScore>,
    today_day: Option<NaiveDate>,
) -> String {
    let content = if scoreboard.games.is_empty() {
        None
    } else {
        let mut content = String::new();
        if let Some(game) = selected {
            let scoreboard_game = scoreboard
                .games
                .iter()
                .find(|scoreboard_game| scoreboard_game.game_id == game.game_id);
            content.push_str(&detail_nav(
                "/mlb/scoreboard",
                &day.to_string(),
                &scoreboard.games,
                &game.game_id,
                League::Mlb,
            ));
            content.push_str(&mlb_game_details(game, scoreboard_game));
        } else {
            content.push_str(&game_list(
                "/mlb/scoreboard",
                &day.to_string(),
                &scoreboard.games,
                League::Mlb,
            ));
        }
        Some(content)
    };
    let refresh_at = mlb_refresh_at(scoreboard, selected);
    let title = if selected.is_some() {
        "MLB Game"
    } else {
        "MLB Scoreboard"
    };
    scoreboard_shell(
        date_nav(day, "/mlb/scoreboard", today_day),
        selected.is_some(),
        content,
        title,
        refresh_at.as_deref(),
    )
}

pub fn nfl_scoreboard_page(
    week: i64,
    scoreboard: &Scoreboard,
    selected: Option<&NflBoxScore>,
) -> String {
    let content = if scoreboard.games.is_empty() {
        None
    } else {
        let mut content = String::new();
        if let Some(game) = selected {
            content.push_str(&detail_nav(
                "/nfl/scoreboard",
                &week.to_string(),
                &scoreboard.games,
                &game.game_id,
                League::Nfl,
            ));
            content.push_str(&nfl_game_details(game));
        } else {
            content.push_str(&game_list(
                "/nfl/scoreboard",
                &week.to_string(),
                &scoreboard.games,
                League::Nfl,
            ));
        }
        Some(content)
    };
    let refresh_at = nfl_refresh_at(scoreboard, selected);
    let title = if selected.is_some() {
        "NFL Game"
    } else {
        "NFL Scoreboard"
    };
    scoreboard_shell(
        week_nav(week, "/nfl/scoreboard"),
        selected.is_some(),
        content,
        title,
        refresh_at.as_deref(),
    )
}

pub fn nhl_scoreboard_page_with_today(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&NhlBoxScore>,
    today_day: NaiveDate,
) -> String {
    let content = if scoreboard.games.is_empty() {
        None
    } else {
        let mut content = String::new();
        if let Some(game) = selected {
            content.push_str(&detail_nav(
                "/nhl/scoreboard",
                &day.to_string(),
                &scoreboard.games,
                &game.game_id,
                League::Nhl,
            ));
            content.push_str(&nhl_game_details(game));
        } else {
            content.push_str(&game_list(
                "/nhl/scoreboard",
                &day.to_string(),
                &scoreboard.games,
                League::Nhl,
            ));
        }
        Some(content)
    };
    let refresh_at = nhl_refresh_at(scoreboard, selected);
    let title = if selected.is_some() {
        "NHL Game"
    } else {
        "NHL Scoreboard"
    };
    scoreboard_shell(
        date_nav(day, "/nhl/scoreboard", Some(today_day)),
        selected.is_some(),
        content,
        title,
        refresh_at.as_deref(),
    )
}

pub fn soccer_scoreboard_page_with_today(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&SoccerBoxScore>,
    today_day: NaiveDate,
) -> String {
    soccer_scoreboard_page_for_league(
        "World Cup",
        "/worldcup/scoreboard",
        League::Soccer,
        day,
        scoreboard,
        selected,
        today_day,
    )
}

pub fn nwsl_scoreboard_page_with_today(
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&SoccerBoxScore>,
    today_day: NaiveDate,
) -> String {
    soccer_scoreboard_page_for_league(
        "NWSL",
        "/nwsl/scoreboard",
        League::Nwsl,
        day,
        scoreboard,
        selected,
        today_day,
    )
}

fn soccer_scoreboard_page_for_league(
    title_prefix: &str,
    scoreboard_base: &str,
    league: League,
    day: NaiveDate,
    scoreboard: &Scoreboard,
    selected: Option<&SoccerBoxScore>,
    today_day: NaiveDate,
) -> String {
    let content = if scoreboard.games.is_empty() {
        None
    } else {
        let mut content = String::new();
        if let Some(game) = selected {
            content.push_str(&detail_nav(
                scoreboard_base,
                &day.to_string(),
                &scoreboard.games,
                &game.game_id,
                league,
            ));
            content.push_str(&soccer_game_details(game));
        } else {
            content.push_str(&game_list(
                scoreboard_base,
                &day.to_string(),
                &scoreboard.games,
                league,
            ));
        }
        Some(content)
    };
    let refresh_at = soccer_refresh_at(scoreboard, selected);
    let title = if selected.is_some() {
        format!("{title_prefix} Match")
    } else {
        format!("{title_prefix} Scoreboard")
    };
    scoreboard_shell(
        date_nav(day, scoreboard_base, Some(today_day)),
        selected.is_some(),
        content,
        &title,
        refresh_at.as_deref(),
    )
}

fn scoreboard_shell(
    nav_html: String,
    has_selected_game: bool,
    content: Option<String>,
    title: &str,
    refresh_at: Option<&str>,
) -> String {
    let mut html = String::from(r#"<main class="page">"#);
    html.push_str(&nav_html);
    if let Some(content) = content {
        let class = if has_selected_game {
            "scoreboard has-game"
        } else {
            "scoreboard"
        };
        html.push_str(&format!(r#"<section class="{class}">"#));
        html.push_str(&content);
        html.push_str("</section>");
    } else {
        html.push_str(r#"<section class="center"><h1>No Games Scheduled</h1></section>"#);
    }
    html.push_str("</main>");
    layout_with_refresh(title, &html, refresh_at)
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

fn basketball_refresh_at(scoreboard: &Scoreboard, selected: Option<&BoxScore>) -> Option<String> {
    let selected_live = selected.is_some_and(|game| game.game_status == 2);
    live_refresh_at(scoreboard.games.iter().any(|game| game.game_status == 2) || selected_live)
}

fn mlb_refresh_at(scoreboard: &Scoreboard, selected: Option<&MlbBoxScore>) -> Option<String> {
    let selected_live = selected.is_some_and(|game| game.game_status == 2);
    live_refresh_at(scoreboard.games.iter().any(|game| game.game_status == 2) || selected_live)
}

fn nfl_refresh_at(scoreboard: &Scoreboard, selected: Option<&NflBoxScore>) -> Option<String> {
    let selected_live = selected.is_some_and(|game| game.game_status == 2);
    live_refresh_at(scoreboard.games.iter().any(|game| game.game_status == 2) || selected_live)
}

fn nhl_refresh_at(scoreboard: &Scoreboard, selected: Option<&NhlBoxScore>) -> Option<String> {
    let selected_live = selected.is_some_and(|game| game.game_status == 2);
    live_refresh_at(scoreboard.games.iter().any(|game| game.game_status == 2) || selected_live)
}

fn soccer_refresh_at(scoreboard: &Scoreboard, selected: Option<&SoccerBoxScore>) -> Option<String> {
    let selected_live = selected.is_some_and(|game| game.game_status == 2);
    live_refresh_at(scoreboard.games.iter().any(|game| game.game_status == 2) || selected_live)
}

fn live_refresh_at(should_refresh: bool) -> Option<String> {
    should_refresh.then(|| {
        (Utc::now() + chrono::Duration::seconds(LIVE_DATA_CACHE_SECONDS as i64))
            .to_rfc3339_opts(SecondsFormat::Secs, true)
    })
}

fn date_nav(day: NaiveDate, base_path: &str, today_day: Option<NaiveDate>) -> String {
    let mut html = String::from(r#"<div class="date-nav">"#);
    html.push_str(&format!(
        r#"<a class="button" href="{}/{}">Prev</a>"#,
        base_path,
        day.checked_sub_days(Days::new(1)).unwrap_or(day)
    ));
    for offset in -3..=3 {
        let d = day + chrono::Duration::days(offset);
        let is_today = today_day == Some(d);
        let mut label = format!(
            "{} {}/{}",
            weekday(d.weekday().num_days_from_sunday()),
            d.month(),
            d.day()
        );
        if is_today {
            label.push_str(" *");
        }
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

fn detail_nav(
    base_path: &str,
    bucket: &str,
    games: &[Game],
    selected_game_id: &str,
    league: League,
) -> String {
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
        html.push_str(&game_step_link(base_path, bucket, prev, "prev", league));
    }
    if let Some(current) = current {
        html.push_str(&game_step_link(
            base_path, bucket, current, "current", league,
        ));
    }
    if let Some(next) = next {
        html.push_str(&game_step_link(base_path, bucket, next, "next", league));
    }
    html.push_str("</div>");
    html
}

fn game_step_link(
    base_path: &str,
    bucket: &str,
    game: &Game,
    direction: &str,
    league: League,
) -> String {
    let status = compact_game_status(game, league);
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

fn compact_game_status(game: &Game, league: League) -> String {
    if game.game_status == 2 && game.period > 0 {
        let period = live_period_label(game.period, league);
        let clock = game.game_clock.trim();
        if clock.is_empty() || matches!(clock, "0.0" | "0:00") {
            return period;
        }
        return format!("{period} {clock}");
    }
    game.game_status_text.trim().to_string()
}

fn live_period_label(period: i64, league: League) -> String {
    match league {
        League::Nhl => match period {
            1 => "1st".to_string(),
            2 => "2nd".to_string(),
            3 => "3rd".to_string(),
            period => format!("{}OT", period - 3),
        },
        League::Soccer | League::Nwsl => {
            if period <= 1 {
                "1H".to_string()
            } else {
                "2H".to_string()
            }
        }
        _ => match period {
            1 => "1Q".to_string(),
            2 => "2Q".to_string(),
            3 => "3Q".to_string(),
            4 => "4Q".to_string(),
            period => format!("{}OT", period - 4),
        },
    }
}

fn game_summary(game: &Game, show_status: bool, league: League) -> String {
    let class = match league {
        League::Mlb => "game-card mlb-game-card",
        League::Soccer | League::Nwsl => "game-card soccer-game-card",
        League::Nba | League::Wnba | League::Nfl | League::Nhl => "game-card period-game-card",
    };
    let mut html = format!(r#"<table class="{class}"><thead><tr><th></th>"#);
    match league {
        League::Nba | League::Wnba | League::Nfl | League::Nhl => {
            for period in 1..=period_column_count(game, league) {
                html.push_str(&format!("<th>{}</th>", period_header(period, league)));
            }
            html.push_str("<th>T</th></tr></thead><tbody>");
        }
        League::Mlb => html.push_str("<th>R</th><th>H</th><th>E</th></tr></thead><tbody>"),
        League::Soccer | League::Nwsl => html.push_str("<th>Score</th></tr></thead><tbody>"),
    }
    html.push_str(&team_summary_row(game, &game.away_team, false, league));
    html.push_str(&team_summary_row(game, &game.home_team, true, league));
    if show_status {
        let colspan = match league {
            League::Nba | League::Wnba | League::Nfl | League::Nhl => {
                period_column_count(game, league) + 2
            }
            League::Mlb => 4,
            League::Soccer | League::Nwsl => 2,
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
        League::Nba | League::Wnba | League::Nfl | League::Nhl => {
            for period in 1..=period_column_count(game, league) {
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
        League::Soccer | League::Nwsl if game.game_status == 1 => {
            html.push_str(r#"<td class="score-total">-</td></tr>"#);
        }
        League::Soccer | League::Nwsl => {
            let pens = team
                .shootout_score
                .map(|s| format!(r#" <span class="pens">({s})</span>"#))
                .unwrap_or_default();
            html.push_str(&format!(
                r#"<td class="score-total">{}{pens}</td></tr>"#,
                team.score
            ));
        }
    }
    html
}

fn period_column_count(game: &Game, league: League) -> i64 {
    let regulation_periods = match league {
        League::Nhl => 3,
        League::Soccer | League::Nwsl => 0,
        _ => 4,
    };
    game.away_team
        .periods
        .iter()
        .chain(game.home_team.periods.iter())
        .map(|period| period.period)
        .max()
        .unwrap_or(0)
        .max(regulation_periods)
}

fn period_header(period: i64, league: League) -> String {
    match league {
        League::Nhl if period > 3 => format!("{}OT", period - 3),
        League::Soccer | League::Nwsl => String::new(),
        _ => period.to_string(),
    }
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

fn game_details(game: &BoxScore, league: League) -> String {
    let mut html = String::from(r#"<section class="details">"#);
    html.push_str(&team_game_details(
        game,
        &game.away_team,
        &game.home_team,
        false,
        league,
    ));
    html.push_str(&team_game_details(
        game,
        &game.home_team,
        &game.away_team,
        true,
        league,
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

fn nhl_game_details(game: &NhlBoxScore) -> String {
    let mut html = String::from(r#"<section class="details">"#);
    html.push_str(&nhl_team_stats_comparison(game));
    html.push_str(&nhl_team_game_details(game, &game.away_team, false));
    html.push_str(&nhl_team_game_details(game, &game.home_team, true));
    html.push_str("</section>");
    html
}

fn soccer_game_details(game: &SoccerBoxScore) -> String {
    let mut html = String::from(r#"<section class="details">"#);
    html.push_str(&soccer_events_panel(game));
    html.push_str(&soccer_team_stats_comparison(game));
    html.push_str(&soccer_team_game_details(game, &game.away_team, false));
    html.push_str(&soccer_team_game_details(game, &game.home_team, true));
    html.push_str("</section>");
    html
}

fn soccer_events_panel(game: &SoccerBoxScore) -> String {
    let goals: Vec<&SoccerEvent> = game
        .events
        .iter()
        .filter(|event| event.kind == "Goal")
        .collect();
    let cards: Vec<&SoccerEvent> = game
        .events
        .iter()
        .filter(|event| event.kind.contains("Card"))
        .collect();
    if goals.is_empty() && cards.is_empty() {
        return String::new();
    }

    let mut html = String::from(r#"<article class="team-details soccer-events">"#);
    if !goals.is_empty() {
        html.push_str("<h1>Goals</h1>");
        html.push_str(&soccer_event_table(&goals, true));
    }
    if !cards.is_empty() {
        html.push_str("<h1>Cards</h1>");
        html.push_str(&soccer_event_table(&cards, false));
    }
    html.push_str("</article>");
    html
}

fn soccer_event_table(events: &[&SoccerEvent], show_assist: bool) -> String {
    let mut headers = vec!["Min", "Team", "Player"];
    if show_assist {
        headers.push("Assist");
    } else {
        headers.push("Card");
    }
    headers.push("Note");
    let rows: Vec<Vec<String>> = events
        .iter()
        .map(|event| {
            let mut row = vec![
                event.minute.clone(),
                event.team_tricode.clone(),
                event.player.clone(),
            ];
            if show_assist {
                row.push(event.assist.clone());
            } else {
                row.push(event.kind.clone());
            }
            row.push(event.note.clone());
            row
        })
        .collect();
    sortable_table(&headers, &rows)
}

fn soccer_team_game_details(
    game: &SoccerBoxScore,
    team: &SoccerBoxScoreTeam,
    is_home: bool,
) -> String {
    let mut html = String::from(r#"<article class="team-details">"#);
    html.push_str("<h1>");
    html.push_str(&team_logo(&team.team, "logo", League::Soccer));
    html.push_str(&format!(
        "{} <strong>{}</strong> {}",
        escape(&team.team.team_name),
        team.team.score,
        soccer_box_winner(game, is_home)
    ));
    html.push_str("</h1>");
    html.push_str("</article>");
    html
}

fn soccer_team_stats_comparison(game: &SoccerBoxScore) -> String {
    team_stats_comparison_with_class(
        &game.away_team.team.team_tricode,
        &game.home_team.team.team_tricode,
        &game.away_team.team_stats,
        &game.home_team.team_stats,
        " soccer-match-stats",
    )
}

fn nhl_team_game_details(game: &NhlBoxScore, team: &NhlBoxScoreTeam, is_home: bool) -> String {
    let mut html = String::from(r#"<article class="team-details">"#);
    html.push_str("<h1>");
    html.push_str(&team_logo(&team.team, "logo", League::Nhl));
    html.push_str(&format!(
        "{} {} <strong>{}</strong> {}",
        escape(&team.team.team_city),
        escape(&team.team.team_name),
        team.team.score,
        nhl_box_winner(game, is_home)
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

fn nhl_team_stats_comparison(game: &NhlBoxScore) -> String {
    team_stats_comparison(
        &game.away_team.team.team_tricode,
        &game.home_team.team.team_tricode,
        &game.away_team.team_stats,
        &game.home_team.team_stats,
    )
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
    team_stats_comparison(
        &game.away_team.team.team_tricode,
        &game.home_team.team.team_tricode,
        &game.away_team.team_stats,
        &game.home_team.team_stats,
    )
}

fn team_stats_comparison(
    away_label: &str,
    home_label: &str,
    away_stats: &Table,
    home_stats: &Table,
) -> String {
    team_stats_comparison_with_class(away_label, home_label, away_stats, home_stats, "")
}

fn team_stats_comparison_with_class(
    away_label: &str,
    home_label: &str,
    away_stats: &Table,
    home_stats: &Table,
    article_class_suffix: &str,
) -> String {
    let mut rows = Vec::new();
    for row in &away_stats.rows {
        let Some(stat) = row.first() else {
            continue;
        };
        let away_value = row.get(1).cloned().unwrap_or_default();
        let home_value = home_stats
            .rows
            .iter()
            .find(|home_row| home_row.first() == Some(stat))
            .and_then(|home_row| home_row.get(1))
            .cloned()
            .unwrap_or_default();
        rows.push((stat.clone(), away_value, home_value));
    }
    for row in &home_stats.rows {
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
        r#"<article class="team-details{article_class_suffix}"><h1>Team Stats</h1>{}</article>"#,
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
    let inning_count = away_team
        .periods
        .iter()
        .chain(home_team.periods.iter())
        .map(|period| period.period)
        .max()
        .unwrap_or(0);
    for period in 1..=inning_count {
        html.push_str(&format!("<th>{period}</th>"));
    }
    html.push_str("<th>R</th><th>H</th><th>E</th></tr></thead><tbody>");
    html.push_str(&mlb_line_score_row(away_team, inning_count));
    html.push_str(&mlb_line_score_row(home_team, inning_count));
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

fn mlb_line_score_row(team: &Team, inning_count: i64) -> String {
    let mut html = String::from("<tr><th>");
    html.push_str(&team_logo(team, "mini-logo", League::Mlb));
    html.push_str(&format!(
        r#"<span title="{}">{}</span>"#,
        escape_attr(&format!("{} {}", team.team_city, team.team_name)),
        escape(&team.team_tricode)
    ));
    html.push_str("</th>");
    for period in 1..=inning_count {
        let cell = team
            .periods
            .iter()
            .find(|score| score.period == period)
            .map(|score| {
                if score.score == 0 {
                    "-".to_string()
                } else {
                    score.score.to_string()
                }
            })
            .unwrap_or_default();
        html.push_str(&format!("<td>{cell}</td>"));
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
    league: League,
) -> String {
    let mut html = String::from(r#"<article class="team-details">"#);
    html.push_str("<h1>");
    html.push_str(&team_logo(&team.team, "logo", league));
    html.push_str(&format!(
        "{} {} <strong>{}</strong> {}",
        escape(&team.team.team_city),
        escape(&team.team.team_name),
        team.team.score,
        box_winner(game, is_home)
    ));
    html.push_str("</h1>");
    html.push_str(&team_box(team, other, league));
    html.push_str("</article>");
    html
}

fn team_box(team: &BoxScoreTeam, other: &BoxScoreTeam, league: League) -> String {
    let mut rows = Vec::new();
    for player in team.players.iter().filter(|p| p.played) {
        let s = &player.statistics;
        let player_name = format!(
            "{}{}",
            escape(&player.name),
            if player.starter { "*" } else { "" }
        );
        let player_cell = match league {
            League::Nba => format!(
                r#"<a href="/nba/player/{}">{}</a>"#,
                player.person_id, player_name
            ),
            League::Wnba => format!(
                r#"<a href="/wnba/player/{}">{}</a>"#,
                player.person_id, player_name
            ),
            _ => player_name,
        };
        rows.push(vec![
            table_cell(player_cell),
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
    rows.push(box_score_team_summary_row(&team.players, &team.statistics));
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
        },
    )
}

fn box_score_team_summary_row(players: &[Player], fallback: &TeamStatistics) -> Vec<TableCell> {
    let team_stats = players.iter().filter(|player| player.played).fold(
        Statistics::default(),
        |mut total, player| {
            let stats = &player.statistics;
            total.assists += stats.assists;
            total.blocks += stats.blocks;
            total.field_goals_attempted += stats.field_goals_attempted;
            total.field_goals_made += stats.field_goals_made;
            total.fouls_personal += stats.fouls_personal;
            total.free_throws_attempted += stats.free_throws_attempted;
            total.free_throws_made += stats.free_throws_made;
            total.minutes += stats.minutes;
            total.points += stats.points;
            total.rebounds_defensive += stats.rebounds_defensive;
            total.rebounds_offensive += stats.rebounds_offensive;
            total.rebounds_total += stats.rebounds_total;
            total.steals += stats.steals;
            total.three_pointers_attempted += stats.three_pointers_attempted;
            total.three_pointers_made += stats.three_pointers_made;
            total.turnovers += stats.turnovers;
            total
        },
    );
    let stats = merge_team_summary(team_stats, fallback);
    vec![
        table_cell("<strong>Team</strong>"),
        table_cell(stats.minutes.to_string()),
        table_cell(stats.points.to_string()),
        table_cell(stats.rebounds_total.to_string()),
        table_cell(stats.assists.to_string()),
        table_cell(String::new()),
        table_cell(format!(
            "{}-{}",
            stats.field_goals_made, stats.field_goals_attempted
        )),
        table_cell(format!(
            "{}-{}",
            stats.three_pointers_made, stats.three_pointers_attempted
        )),
        table_cell(format!(
            "{}-{}",
            stats.free_throws_made, stats.free_throws_attempted
        )),
        table_cell(
            stats::points_per_shot(&stats)
                .map(|value| format!("{value:.2}"))
                .unwrap_or_default(),
        ),
        table_cell(stats.turnovers.to_string()),
        table_cell(stats.steals.to_string()),
        table_cell(stats.blocks.to_string()),
        table_cell(stats.fouls_personal.to_string()),
        table_cell(String::new()),
        table_cell(String::new()),
    ]
}

fn merge_team_summary(mut stats: Statistics, fallback: &TeamStatistics) -> Statistics {
    stats.minutes = stats.minutes.max(fallback.minutes);
    stats.points = stats.points.max(fallback.points);
    if fallback.field_goals_attempted > stats.field_goals_attempted {
        stats.field_goals_attempted = fallback.field_goals_attempted;
        stats.field_goals_made = fallback.field_goals_made;
    }
    if fallback.three_pointers_attempted > stats.three_pointers_attempted {
        stats.three_pointers_attempted = fallback.three_pointers_attempted;
        stats.three_pointers_made = fallback.three_pointers_made;
    }
    if fallback.free_throws_attempted > stats.free_throws_attempted {
        stats.free_throws_attempted = fallback.free_throws_attempted;
        stats.free_throws_made = fallback.free_throws_made;
    }
    stats.rebounds_total = stats.rebounds_total.max(fallback.rebounds_total);
    stats.assists = stats.assists.max(fallback.assists);
    stats.turnovers = stats.turnovers.max(fallback.turnovers);
    stats.steals = stats.steals.max(fallback.steals);
    stats.blocks = stats.blocks.max(fallback.blocks);
    stats.fouls_personal = stats.fouls_personal.max(fallback.fouls_personal);
    stats
}

pub fn standings_page(standings: &StandingsTable) -> String {
    let legend = playoff_legend(&[
        ("playoff", "Clinched playoff seed (top 6)"),
        ("playin", "Play-In tournament (seeds 7\u{2013}10)"),
    ]);
    standings_shell(
        "NBA Standings",
        "",
        format!(
            "{legend}{}{}",
            standings_table(
                "East",
                &standings.east,
                League::Nba,
                false,
                Some("cut-playoff-6 cut-playin-10"),
            ),
            standings_table(
                "West",
                &standings.west,
                League::Nba,
                false,
                Some("cut-playoff-6 cut-playin-10"),
            ),
        ),
    )
}

pub fn wnba_standings_page(standings: &StandingsTable) -> String {
    let teams = league_wide_standings(standings);
    let legend = playoff_legend(&[("playoff", "Playoff cutoff (top 8)")]);
    standings_shell(
        "WNBA Standings",
        "",
        format!(
            "{legend}{}",
            standings_table("League", &teams, League::Wnba, true, Some("cut-playoff-8")),
        ),
    )
}

/// Merge the conference tables into a single league-wide table sorted by
/// record, recomputing games-back relative to the overall leader.
fn league_wide_standings(standings: &StandingsTable) -> Vec<StandingsTeam> {
    let mut teams: Vec<StandingsTeam> = standings
        .east
        .iter()
        .chain(standings.west.iter())
        .cloned()
        .collect();
    teams.sort_by(|a, b| {
        b.win_pct
            .partial_cmp(&a.win_pct)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(b.wins.cmp(&a.wins))
            .then(a.losses.cmp(&b.losses))
            .then(a.team_name.cmp(&b.team_name))
    });
    if let Some(leader) = teams.first().cloned() {
        for team in &mut teams {
            team.conference_games_back =
                ((leader.wins - team.wins) + (team.losses - leader.losses)) as f64 / 2.0;
        }
    }
    teams
}

fn playoff_legend(items: &[(&str, &str)]) -> String {
    let entries: String = items
        .iter()
        .map(|(swatch, label)| {
            format!(
                r#"<span class="legend-item"><span class="legend-swatch {swatch}"></span>{}</span>"#,
                escape(label)
            )
        })
        .collect();
    format!(r#"<p class="standings-legend">{entries}</p>"#)
}

pub fn mlb_standings_page(standings: &MlbStandingsTable) -> String {
    standings_shell(
        "MLB Standings",
        "mlb-standings",
        standings
            .divisions
            .iter()
            .map(|division| {
                mlb_standings_table(
                    &format!("{} {}", division.league, division.division),
                    &division.teams,
                )
            })
            .collect::<String>(),
    )
}

pub fn nfl_standings_page(standings: &NflStandingsTable) -> String {
    standings_shell(
        "NFL Standings",
        "nfl-standings",
        standings
            .divisions
            .iter()
            .map(|division| {
                nfl_standings_table(
                    &format!("{} {}", division.conference, division.division),
                    &division.teams,
                )
            })
            .collect::<String>(),
    )
}

pub fn nhl_standings_page(standings: &NhlStandingsTable) -> String {
    standings_shell(
        "NHL Standings",
        "nhl-standings",
        standings
            .divisions
            .iter()
            .map(|division| {
                nhl_standings_table(
                    &format!("{} {}", division.conference, division.division),
                    &division.teams,
                )
            })
            .collect::<String>(),
    )
}

pub fn bracket_page(title_prefix: &str, bracket: &BracketTable) -> String {
    let title = format!("{title_prefix} Bracket");
    let body = if bracket.rounds.is_empty() {
        r#"<main class="page bracket"><section class="center"><h1>No Bracket Available</h1></section></main>"#.to_string()
    } else {
        let rounds = bracket
            .rounds
            .iter()
            .map(|round| {
                let matches = round
                    .matches
                    .iter()
                    .map(bracket_match_html)
                    .collect::<String>();
                format!(
                    r#"<section class="bracket-round"><h2 class="bracket-round-title">{}</h2><div class="bracket-matches">{matches}</div></section>"#,
                    escape(&round.name)
                )
            })
            .collect::<String>();
        let third_place = bracket
            .third_place
            .as_ref()
            .map(|third| {
                format!(
                    r#"<section class="bracket-third"><h2 class="bracket-round-title">Third Place</h2><div class="bracket-matches">{}</div></section>"#,
                    bracket_match_html(third)
                )
            })
            .unwrap_or_default();
        format!(
            r#"<main class="page bracket"><h1>{}</h1><div class="bracket-scroll"><div class="bracket-rounds">{rounds}</div></div>{third_place}</main>"#,
            escape(&title)
        )
    };
    let refresh_at = bracket_refresh_at(bracket);
    layout_with_refresh(&title, &body, refresh_at.as_deref())
}

fn bracket_refresh_at(bracket: &BracketTable) -> Option<String> {
    let should_refresh = bracket
        .rounds
        .iter()
        .any(|round| round.matches.iter().any(|game| game.game_status == 2))
        || bracket
            .third_place
            .as_ref()
            .is_some_and(|game| game.game_status == 2);
    live_refresh_at(should_refresh)
}

fn bracket_match_html(game: &BracketMatch) -> String {
    let played = game.game_status >= 2;
    let completed = if game.game_status == 3 {
        " completed"
    } else if game.game_status == 2 {
        " live"
    } else {
        ""
    };
    let caption_html = if game.game_status < 2 && !game.game_time_utc.is_empty() {
        format!(
            r#"<div class="bracket-caption"><time data-local-game-time data-show-date datetime="{}">{}</time></div>"#,
            escape_attr(&game.game_time_utc),
            escape(game.game_status_text.trim()),
        )
    } else {
        let caption = game.game_status_text.trim();
        if caption.is_empty() {
            String::new()
        } else {
            format!(r#"<div class="bracket-caption">{}</div>"#, escape(caption))
        }
    };
    let day = game.game_time_utc.get(..10).unwrap_or_default();
    let inner = format!(
        r#"<article class="bracket-match{completed}">{}{}{caption_html}</article>"#,
        bracket_slot_html(&game.home, played),
        bracket_slot_html(&game.away, played),
    );
    if !game.game_id.is_empty() && !day.is_empty() && game.game_status >= 2 {
        format!(
            r#"<div class="bracket-cell"><a class="bracket-link" href="/worldcup/scoreboard/{}/game/{}">{inner}</a></div>"#,
            escape_attr(day),
            escape_attr(&game.game_id),
        )
    } else {
        format!(r#"<div class="bracket-cell">{inner}</div>"#)
    }
}

fn bracket_slot_html(slot: &BracketSlot, played: bool) -> String {
    let mut classes = String::from("bracket-slot");
    if slot.winner {
        classes.push_str(" winner");
    }
    if slot.placeholder {
        classes.push_str(" placeholder");
    }
    let flag = if !slot.placeholder && !slot.logo.is_empty() {
        format!(
            r#"<img class="bracket-flag" src="{}" alt="{}">"#,
            escape_attr(&slot.logo),
            escape_attr(&slot.name)
        )
    } else {
        r#"<span class="bracket-flag empty" aria-hidden="true"></span>"#.to_string()
    };
    let score = if played && !slot.score.is_empty() {
        let pens = if slot.shootout_score.is_empty() {
            String::new()
        } else {
            format!(
                r#" <span class="bracket-pens">({})</span>"#,
                escape(&slot.shootout_score)
            )
        };
        format!(
            r#"<span class="bracket-score">{}{pens}</span>"#,
            escape(&slot.score)
        )
    } else {
        r#"<span class="bracket-score"></span>"#.to_string()
    };
    format!(
        r#"<div class="{classes}">{flag}<span class="bracket-team">{}</span>{score}</div>"#,
        escape(&slot.name)
    )
}

pub fn soccer_standings_page(title_prefix: &str, standings: &SoccerStandingsTable) -> String {
    soccer_standings_page_for_league(title_prefix, standings, League::Soccer)
}

pub fn nwsl_standings_page(standings: &SoccerStandingsTable) -> String {
    soccer_standings_page_for_league("NWSL", standings, League::Nwsl)
}

fn soccer_standings_page_for_league(
    title_prefix: &str,
    standings: &SoccerStandingsTable,
    league: League,
) -> String {
    standings_shell(
        &format!("{title_prefix} Standings"),
        "soccer-standings",
        standings
            .groups
            .iter()
            .map(|group| soccer_standings_table(&group.group, &group.teams, league))
            .collect::<String>(),
    )
}

fn standings_shell(title: &str, class_suffix: &str, content: String) -> String {
    let class_suffix = if class_suffix.is_empty() {
        String::new()
    } else {
        format!(" {class_suffix}")
    };
    layout(
        title,
        &format!(r#"<main class="page standings{class_suffix}">{content}</main>"#),
    )
}

fn standings_table(
    title: &str,
    rows: &[StandingsTeam],
    league: League,
    rerank: bool,
    cutoff_class: Option<&'static str>,
) -> String {
    let table_rows: Vec<Vec<String>> = rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let rank = if rerank {
                index as i64 + 1
            } else {
                row.playoff_rank
            };
            let logo = team_logo_id_for_league(
                row.team_id,
                &row.team_tricode,
                &row.team_name,
                "mini-logo",
                league,
            );
            let team_name = escape(&row.team_name);
            let team_cell = match league {
                League::Nba => format!(
                    r#"{logo}<a href="/nba/team/{}">{team_name}</a>"#,
                    escape_attr(&row.team_tricode.to_ascii_lowercase())
                ),
                League::Wnba => format!(
                    r#"{logo}<a href="/wnba/team/{}">{team_name}</a>"#,
                    escape_attr(&row.team_tricode.to_ascii_lowercase())
                ),
                _ => format!("{logo}{team_name}"),
            };
            vec![
                rank.to_string(),
                team_cell,
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
                class: cutoff_class,
                default_sort_index: Some(0),
                default_sort_dir: Some("asc"),
                ..TableOptions::default()
            },
        )
    )
}

/// Standings team cell linking the team name to its detail page.
fn linked_team_cell(slug: &str, logo: &str, tricode: &str, team_name: &str) -> String {
    format!(
        r#"{logo}<a href="/{slug}/team/{}">{}</a>"#,
        escape_attr(&tricode.to_ascii_lowercase()),
        escape(team_name)
    )
}

fn nfl_standings_table(title: &str, rows: &[NflStandingsTeam]) -> String {
    let table_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            vec![
                row.playoff_rank.to_string(),
                linked_team_cell(
                    "nfl",
                    &nfl_team_logo(&row.team_tricode, &row.team_name, "mini-logo"),
                    &row.team_tricode,
                    &row.team_name,
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

fn nhl_standings_table(title: &str, rows: &[NhlStandingsTeam]) -> String {
    let table_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            vec![
                row.playoff_rank.to_string(),
                linked_team_cell(
                    "nhl",
                    &nhl_team_logo(&row.team_tricode, &row.team_name, "mini-logo"),
                    &row.team_tricode,
                    &row.team_name,
                ),
                row.wins.to_string(),
                row.losses.to_string(),
                row.ot_losses.to_string(),
                row.points.to_string(),
                row.games_back.clone(),
                row.goals_for.to_string(),
                row.goals_against.to_string(),
                row.goal_diff.clone(),
                row.home.clone(),
                row.road.clone(),
                row.division_record.clone(),
                row.last_ten.clone(),
                row.streak.clone(),
            ]
        })
        .collect();
    format!(
        r#"<article class="panel"><h1>{}</h1>{}</article>"#,
        escape(title),
        sortable_table_with_options(
            &[
                "#", "Team", "W", "L", "OTL", "PTS", "GB", "GF", "GA", "DIFF", "HM", "RD", "DIV",
                "L10", "STR"
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

fn soccer_standings_table(title: &str, rows: &[SoccerStandingsTeam], league: League) -> String {
    let table_rows: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            vec![
                row.rank.to_string(),
                format!(
                    "{}{}",
                    team_logo(
                        &Team {
                            team_id: row.team_id,
                            team_name: row.team_name.clone(),
                            team_city: String::new(),
                            team_tricode: row.team_tricode.clone(),
                            wins: 0,
                            losses: 0,
                            display_record: String::new(),
                            score: 0,
                            shootout_score: None,
                            hits: 0,
                            errors: 0,
                            periods: Vec::new(),
                        },
                        "mini-logo",
                        league,
                    ),
                    escape(&row.team_name)
                ),
                row.games_played.to_string(),
                row.wins.to_string(),
                row.draws.to_string(),
                row.losses.to_string(),
                row.goals_for.to_string(),
                row.goals_against.to_string(),
                row.goal_diff.clone(),
                row.points.to_string(),
                row.record.clone(),
            ]
        })
        .collect();
    format!(
        r#"<article class="panel"><h1>{}</h1>{}</article>"#,
        escape(title),
        sortable_table_with_options(
            &[
                "#", "Team", "MP", "W", "D", "L", "GF", "GA", "GD", "PTS", "REC"
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
                linked_team_cell(
                    "mlb",
                    &mlb_team_logo(&row.team_tricode, &row.team_name, "mini-logo"),
                    &row.team_tricode,
                    &row.team_name,
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
    player_page_for_league("NBA", stats)
}

pub fn player_page_for_league(league_label: &str, stats: &PlayerStatsPage) -> String {
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
    layout(&format!("{league_label} Player"), &body)
}

pub fn team_page(league: &crate::leagues::League, page: &TeamPage) -> String {
    let mut heading = escape(&page.team_name);
    if !page.record.is_empty() {
        heading.push_str(&format!(
            r#" <span class="team-record">{}</span>"#,
            escape(&page.record)
        ));
    }
    let body = format!(
        r#"<main class="page team"><header class="team-header"><h1>{heading}</h1></header><article class="panel"><h1>{}</h1>{}</article><article class="panel"><h1>{}</h1>{}</article><article class="panel"><h1>{}</h1>{}</article></main>"#,
        escape(&page.games.name),
        render_table(&page.games),
        escape(&page.next_games.name),
        render_table(&page.next_games),
        escape(&page.players.name),
        render_table(&page.players),
    );
    layout(&format!("{} {}", league.nav_label, page.team_name), &body)
}

fn render_table(table: &Table) -> String {
    let headers: Vec<&str> = table.headers.iter().map(String::as_str).collect();
    sortable_table_with_first_column_links(&headers, &table.rows, &table.first_column_links)
}

pub fn sortable_table(headers: &[&str], rows: &[Vec<String>]) -> String {
    sortable_table_with_options(headers, rows, TableOptions::default())
}

fn sortable_table_with_first_column_links(
    headers: &[&str],
    rows: &[Vec<String>],
    first_column_links: &[String],
) -> String {
    if first_column_links.is_empty() {
        return sortable_table(headers, rows);
    }
    let cells: Vec<Vec<TableCell>> = rows
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .map(|(cell_index, cell)| {
                    if cell_index == 0
                        && let Some(link) = first_column_links.get(row_index)
                        && !link.is_empty()
                    {
                        return table_cell(format!(
                            r#"<a href="{}">{}</a>"#,
                            escape_attr(link),
                            escape(cell)
                        ));
                    }
                    table_cell(cell.clone())
                })
                .collect()
        })
        .collect();
    sortable_table_cells(headers, &cells, TableOptions::default())
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
        League::Wnba => wnba_team_logo(&team.team_tricode, &team.team_name, class),
        League::Mlb => mlb_team_logo(&team.team_tricode, &team.team_name, class),
        League::Nfl => nfl_team_logo(&team.team_tricode, &team.team_name, class),
        League::Nhl => nhl_team_logo(&team.team_tricode, &team.team_name, class),
        League::Soccer => soccer_team_logo(&team.team_tricode, &team.team_name, class),
        League::Nwsl => nwsl_team_logo(team.team_id, &team.team_tricode, &team.team_name, class),
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

fn nhl_box_winner(game: &NhlBoxScore, is_home: bool) -> &'static str {
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

fn soccer_box_winner(game: &SoccerBoxScore, is_home: bool) -> &'static str {
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
    team_logo_id_for_league(team_id, "", team_name, class, League::Nba)
}

fn team_logo_id_for_league(
    team_id: i64,
    team_tricode: &str,
    team_name: &str,
    class: &str,
    league: League,
) -> String {
    match league {
        League::Wnba => wnba_team_logo(team_tricode, team_name, class),
        _ => format!(
            r#"<img class="{class}" src="https://cdn.nba.com/logos/nba/{}/primary/L/logo.svg" alt="{}">"#,
            team_id,
            escape_attr(team_name)
        ),
    }
}

fn wnba_team_logo(team_tricode: &str, team_name: &str, class: &str) -> String {
    format!(
        r#"<img class="{class}" src="https://a.espncdn.com/i/teamlogos/wnba/500/{}.png" alt="{}">"#,
        escape_attr(&team_tricode.to_lowercase()),
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

fn nhl_team_logo(team_tricode: &str, team_name: &str, class: &str) -> String {
    format!(
        r#"<img class="{class}" src="https://a.espncdn.com/i/teamlogos/nhl/500/{}.png" alt="{}">"#,
        escape_attr(&team_tricode.to_lowercase()),
        escape_attr(team_name)
    )
}

fn soccer_team_logo(team_tricode: &str, team_name: &str, class: &str) -> String {
    format!(
        r#"<img class="{class}" src="https://a.espncdn.com/i/teamlogos/countries/500/{}.png" alt="{}">"#,
        escape_attr(&team_tricode.to_lowercase()),
        escape_attr(team_name)
    )
}

fn nwsl_team_logo(team_id: i64, team_tricode: &str, team_name: &str, class: &str) -> String {
    let logo_id = if team_id > 0 {
        team_id.to_string()
    } else {
        team_tricode.to_lowercase()
    };
    format!(
        r#"<img class="{class}" src="https://a.espncdn.com/i/teamlogos/soccer/500/{}.png" alt="{}">"#,
        escape_attr(&logo_id),
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
    use crate::models::Period;

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
    fn mlb_line_score_row_pads_missing_innings_to_align_totals() {
        let home_team = Team {
            team_id: 1,
            team_name: "Marlins".to_string(),
            team_city: "Miami".to_string(),
            team_tricode: "MIA".to_string(),
            wins: 0,
            losses: 0,
            display_record: String::new(),
            score: 4,
            shootout_score: None,
            hits: 6,
            errors: 0,
            periods: vec![
                Period {
                    period: 1,
                    score: 0,
                },
                Period {
                    period: 2,
                    score: 0,
                },
                Period {
                    period: 3,
                    score: 0,
                },
                Period {
                    period: 4,
                    score: 1,
                },
                Period {
                    period: 5,
                    score: 1,
                },
                Period {
                    period: 6,
                    score: 0,
                },
                Period {
                    period: 7,
                    score: 0,
                },
                Period {
                    period: 8,
                    score: 2,
                },
            ],
        };

        let row = mlb_line_score_row(&home_team, 9);

        assert!(row.contains(r#"<td></td><td class="score-total">4</td><td>6</td><td>0</td>"#));
        assert_eq!(row.matches("<td").count(), 12);
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
                shootout_score: None,
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
                shootout_score: None,
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
