§G
LiSports = Axum HTML sports dashboard for NBA, WNBA, MLB, NFL, NHL scoreboards, games, standings, plus NBA player stats.

§C
C1 Rust 2024, axum 0.8, tokio, reqwest rustls, serde_json, chrono.
C2 `unsafe_code` forbidden; rust warnings + clippy all denied.
C3 Server listens on `PORT` default `8080`; cache root `DATA_PATH` default `data`.
C4 Public assets served from `/public`; HTML is server-rendered string templates.
C5 Upstream data is ESPN site APIs for all leagues; NBA today + fallback player stats may use NBA endpoints.
C6 Cache only stable-ish JSON: completed games, daily standings snapshots, player pages; live today memory cache is 30s.
C7 Invalid route params return 400; upstream/cache/parse failures render error response.
C8 Routes and nav use league prefixes; no API JSON surface.
C9 Known asymmetry: only NBA has `/nba/player/{player_id}`.
C10 No `FORMAT.md` found at repo root; this spec uses required § shape directly.

§I
I.http.home `/` -> temp redirect `/nba/scoreboard`.
I.http.health `/healthcheck` -> `OK`.
I.http.assets `/public/*` serves static app shell files.
I.http.nba `/nba/scoreboard`, `/nba/scoreboard/today`, `/nba/scoreboard/{YYYY-MM-DD}`, `/nba/scoreboard/{YYYY-MM-DD}/game/{game_id}`, `/nba/standings`, `/nba/player/{player_id}`.
I.http.wnba `/wnba/scoreboard`, `/wnba/scoreboard/today`, `/wnba/scoreboard/{YYYY-MM-DD}`, `/wnba/scoreboard/{YYYY-MM-DD}/game/{game_id}`, `/wnba/standings`.
I.http.mlb `/mlb/scoreboard`, `/mlb/scoreboard/today`, `/mlb/scoreboard/{YYYY-MM-DD}`, `/mlb/scoreboard/{YYYY-MM-DD}/game/{game_id}`, `/mlb/standings`.
I.http.nfl `/nfl/scoreboard`, `/nfl/scoreboard/today`, `/nfl/scoreboard/{1..23}`, `/nfl/scoreboard/{1..23}/game/{game_id}`, `/nfl/standings`.
I.http.nhl `/nhl/scoreboard`, `/nhl/scoreboard/today`, `/nhl/scoreboard/{YYYY-MM-DD}`, `/nhl/scoreboard/{YYYY-MM-DD}/game/{game_id}`, `/nhl/standings`.
I.cfg `PORT`, `DATA_PATH`, `RUST_LOG?`, `WATCH_PATHS`, `POLL_INTERVAL`.
I.up.nba.today `https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json`.
I.up.espn.scoreboard `https://site.api.espn.com/apis/site/v2/sports/{sport}/{league}/scoreboard`.
I.up.espn.summary `https://site.api.espn.com/apis/site/v2/sports/{sport}/{league}/summary?event={game_id}`.
I.up.espn.standings `https://site.api.espn.com/apis/v2/sports/{sport}/{league}/standings`.
I.up.espn.nba_player `https://site.web.api.espn.com/apis/common/v3/sports/basketball/nba/athletes/{player_id}/gamelog`.
I.up.nba.player_fallback `https://stats.nba.com/stats/playerdashboardbyyearoveryearcombined?...PlayerID={player_id}...Season=2023-24`.
I.cli `cargo run`, `cargo fmt --check`, `cargo check`, `./dev.sh`.

§V
V1 Router exposes every route in §I.http and dayless scoreboard routes temp-redirect to `/league/scoreboard/today`.
V2 `today` scoreboard URLs render 200 directly; date/week nav links use concrete date/week URLs, not `/today`.
V3 Date leagues accept only `YYYY-MM-DD`; NFL accepts only weeks `1..23`; ids are numeric; invalid params produce 400.
V4 NBA/WNBA/NHL date scoreboards show prev/current/next date window with calendar-today `*`; NFL shows bounded 7-week window and playoff labels 19 Wild Card, 20 Divisional, 21 Conf Champ, 22 Pro Bowl, 23 Super Bowl.
V5 Scoreboard cards link to selected game view; selected game view hides game list, shows detail nav, and marks page `scoreboard has-game`.
V6 Live scoreboard or selected live game (`game_status == 2`) sets `<body data-refresh-at=...>` about `LIVE_DATA_CACHE_SECONDS`; completed pages omit it.
V7 Empty scoreboard renders `No Games Scheduled`; completed scoreboard with games may be file-cached.
V8 Basketball NBA/WNBA box score renders sortable player + team-stat tables; NBA player links are allowed only for NBA, not WNBA.
V9 MLB scoreboard uses R/H/E card columns and no inning columns on list cards; MLB game renders line score plus batting and pitching tables.
V10 NFL game renders team stats and player stat tables; NFL scoreboard periods collapse to total-only card.
V11 NHL game renders team stats and player stat tables; NHL scoreboard renders 3 regulation periods plus total, no 4th-period header.
V12 Standings render sortable tables: NBA/WNBA East+West, MLB AL/NL divisions, NFL AFC/NFC divisions, NHL conference+division groups.
V13 Normalizers convert ESPN status to domain status: completed -> 3, in-progress/halftime -> 2, otherwise 1.
V14 Team records preserve sport display rules: basketball playoff series may replace season record; NHL may include OT losses; MLB/NFL use ESPN total summaries.
V15 Cache keys allow only ASCII alnum plus `:`, `-`, `_`; invalid/stale cache JSON is treated as miss and removed.
V16 Normalizers tolerate missing optional upstream fields with defaults but fail parse when required competition/home/away/header data is absent.
V17 Public layout includes nav, favicon, manifest, CSS, table-sort script; manifest name is `LiSports` and starts at `/nba/scoreboard/today`.
V18 Shared abstractions must preserve sport-specific rules for schedule bucket, periods, standings grouping, records, stat tables, logos, and player-link policy.
V19 League parity target: each league should declare supported surfaces (`scoreboard`, `game`, `standings`, `player?`) from one registry so missing features are explicit.
V20 Player pages target: all player-capable leagues expose player pages or a documented unsupported state; box-score names link only when destination exists.

§T
id|status|task|cites
T1|x|add league registry/config for route base, sport path, date-vs-week bucket, nav label, logo path, feature flags|V18,V19,I.http.nba,I.http.wnba,I.http.mlb,I.http.nfl,I.http.nhl
T2|x|collapse duplicated route handlers into generic league handlers without changing URLs/status/render output|V1,V2,V3,V18,I.http.nba
T3|x|unify today lookup policy: date leagues use same lookback semantics; NFL uses latest week with live/completed games|V2,V4,V7,V18
T4|x|unify ESPN client methods behind parameterized league endpoints and typed sport adapters|V13,V16,V18,I.up.espn.scoreboard,I.up.espn.summary,I.up.espn.standings
T5|x|extract common scoreboard renderer shell; keep sport adapters for card columns, periods, detail tables, refresh logic|V4,V5,V6,V8,V9,V10,V11,V18
T6|x|extract common standings renderer shell; keep sport grouping/columns/ranking adapters|V12,V18
T7|x|add WNBA player page support or explicit unsupported UX; prevent WNBA box-score links to NBA players|V8,V19,V20,I.http.wnba
T8|x|add MLB player pages from ESPN athlete/gamelog or mark unsupported in registry and UI|V19,V20,I.http.mlb
T9|.|add NFL player pages from ESPN athlete/gamelog or mark unsupported in registry and UI|V19,V20,I.http.nfl
T10|.|add NHL player pages from ESPN athlete/gamelog or mark unsupported in registry and UI|V19,V20,I.http.nhl
T11|.|replace hard-coded NBA fallback season `2023-24` with current/configured season or remove fallback if stale|C5,C6,I.up.nba.player_fallback,V20
T12|.|add tests for cache write policy on empty completed scoreboards per league|C6,V7,V15
T13|.|add route/render parity matrix test over all league registry entries|V1,V2,V3,V18,V19
T14|.|add normalizer tests for WNBA standings/team identity and NHL/NFL/MLB missing optional upstream fields|V12,V14,V16
T15|.|document feature matrix in README: scoreboard, game, standings, player, date/week bucket, upstream source|C8,C9,V19,V20
T16|.|decide whether root `/` should remain NBA-first or redirect via configured default league|I.http.home,V19

§B
id|date|cause|fix
