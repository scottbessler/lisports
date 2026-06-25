§G
LiSports = Axum HTML sports dashboard for NBA, WNBA, MLB, NFL, NHL, World Cup scoreboards, games, standings, plus player stats where supported.

§C
C1 Rust 2024, axum 0.8, tokio, reqwest rustls, serde_json, chrono.
C2 `unsafe_code` forbidden; rust warnings + clippy all denied.
C3 Server listens on `PORT` default `8080`; cache root `DATA_PATH` default `data`.
C4 Public assets served from `/public`; HTML is server-rendered string templates.
C5 Upstream data is ESPN site APIs for all leagues; NBA today + fallback player stats may use NBA endpoints.
C6 Cache only stable-ish JSON: completed games, daily standings snapshots, player pages; live today memory cache is 30s.
C7 Invalid route params return 400; upstream/cache/parse failures render error response.
C8 Routes and nav use league prefixes; no API JSON surface.
C9 Player pages exist for NBA, WNBA, MLB, NFL, NHL via league prefixes; World Cup player pages unsupported.
C10 No `FORMAT.md` found at repo root; this spec uses required § shape directly.

§I
I.http.home `/` -> temp redirect `/worldcup/scoreboard`.
I.http.health `/healthcheck` -> `OK`.
I.http.assets `/public/*` serves static app shell files.
I.http.nba `/nba/scoreboard`, `/nba/scoreboard/today`, `/nba/scoreboard/{YYYY-MM-DD}`, `/nba/scoreboard/{YYYY-MM-DD}/game/{game_id}`, `/nba/standings`, `/nba/player/{player_id}`.
I.http.wnba `/wnba/scoreboard`, `/wnba/scoreboard/today`, `/wnba/scoreboard/{YYYY-MM-DD}`, `/wnba/scoreboard/{YYYY-MM-DD}/game/{game_id}`, `/wnba/standings`, `/wnba/player/{player_id}`.
I.http.mlb `/mlb/scoreboard`, `/mlb/scoreboard/today`, `/mlb/scoreboard/{YYYY-MM-DD}`, `/mlb/scoreboard/{YYYY-MM-DD}/game/{game_id}`, `/mlb/standings`, `/mlb/player/{player_id}`.
I.http.nfl `/nfl/scoreboard`, `/nfl/scoreboard/today`, `/nfl/scoreboard/{1..23}`, `/nfl/scoreboard/{1..23}/game/{game_id}`, `/nfl/standings`, `/nfl/player/{player_id}`.
I.http.nhl `/nhl/scoreboard`, `/nhl/scoreboard/today`, `/nhl/scoreboard/{YYYY-MM-DD}`, `/nhl/scoreboard/{YYYY-MM-DD}/game/{game_id}`, `/nhl/standings`, `/nhl/player/{player_id}`.
I.http.worldcup `/worldcup/scoreboard`, `/worldcup/scoreboard/today`, `/worldcup/scoreboard/{YYYY-MM-DD}`, `/worldcup/scoreboard/{YYYY-MM-DD}/game/{game_id}`, `/worldcup/standings`, `/worldcup/bracket`.
I.cfg `PORT`, `DATA_PATH`, `RUST_LOG?`, `WATCH_PATHS`, `POLL_INTERVAL`.
I.up.nba.today `https://nba-prod-us-east-1-mediaops-stats.s3.amazonaws.com/NBA/liveData/scoreboard/todaysScoreboard_00.json`.
I.up.espn.scoreboard `https://site.api.espn.com/apis/site/v2/sports/{sport}/{league}/scoreboard`.
I.up.espn.summary `https://site.api.espn.com/apis/site/v2/sports/{sport}/{league}/summary?event={game_id}`.
I.up.espn.standings `https://site.api.espn.com/apis/v2/sports/{sport}/{league}/standings`.
I.up.espn.player `https://site.web.api.espn.com/apis/common/v3/sports/{sport}/{league}/athletes/{player_id}/gamelog`.
I.up.nba.player_fallback `https://stats.nba.com/stats/playerdashboardbyyearoveryearcombined?...PlayerID={player_id}...Season={current}`.
I.cli `cargo run`, `cargo fmt --check`, `cargo check`, `./dev.sh`.

§V
V1 Router exposes every route in §I.http and dayless scoreboard routes temp-redirect to `/league/scoreboard/today`.
V2 `today` scoreboard URLs render 200 directly; date/week nav links use concrete date/week URLs, not `/today`.
V3 Date leagues accept only `YYYY-MM-DD`; NFL accepts only weeks `1..23`; ids are numeric; invalid params produce 400.
V4 NBA/WNBA/NHL/World Cup date scoreboards show prev/current/next date window with calendar-today `*`; NFL shows bounded 7-week window and playoff labels 19 Wild Card, 20 Divisional, 21 Conf Champ, 22 Pro Bowl, 23 Super Bowl.
V5 Scoreboard cards link to selected game view; selected game view hides game list, shows detail nav, and marks page `scoreboard has-game`.
V6 Live scoreboard or selected live game (`game_status == 2`) sets `<body data-refresh-at=...>` about `LIVE_DATA_CACHE_SECONDS`; completed pages omit it.
V7 Empty scoreboard renders `No Games Scheduled`; completed scoreboard with games may be file-cached.
V8 Basketball NBA/WNBA box score renders sortable player + team-stat tables; player names link only when destination exists.
V9 MLB scoreboard uses R/H/E card columns and no inning columns on list cards; MLB game renders line score plus batting and pitching tables.
V10 NFL game renders team stats and player stat tables; NFL scoreboard periods collapse to total-only card.
V11 NHL game renders team stats and player stat tables; NHL scoreboard renders 3 regulation periods plus total, no 4th-period header.
V12 Standings render sortable tables: NBA/WNBA East+West, MLB AL/NL divisions, NFL AFC/NFC divisions, NHL conference+division groups, World Cup groups.
V23 World Cup scoreboard renders soccer score-only cards; match view renders team-stat comparison plus goals/assists/cards; no fake period columns; mobile stat labels + values remain visible.
V13 Normalizers convert ESPN status to domain status: completed -> 3, in-progress/halftime -> 2, otherwise 1.
V14 Team records preserve sport display rules: basketball/NHL playoff series may replace season record; NHL may include OT losses; MLB/NFL use ESPN total summaries.
V15 Cache keys allow only ASCII alnum plus `:`, `-`, `_`; invalid/stale cache JSON is treated as miss and removed.
V16 Normalizers tolerate missing optional upstream fields with defaults but fail parse when required competition/home/away/header data is absent.
V17 Public layout includes nav, favicon, manifest, CSS, table-sort script; nav labels remain readable on mobile; manifest name is `LiSports` and starts at `/worldcup/scoreboard/today`.
V18 Shared abstractions must preserve sport-specific rules for schedule bucket, periods, standings grouping, records, stat tables, logos, and player-link policy.
V19 League parity target: each league should declare supported surfaces (`scoreboard`, `game`, `standings`, `player?`) from one registry so missing features are explicit.
V20 Player pages target: all player-capable leagues expose player pages or a documented unsupported state; box-score names link only when destination exists.
V21 Player gamelog pages preserve ESPN sport-native labels + stat order; no cross-sport NBA stat remap.
V22 Box-score player stat rows link to same-league player pages when ESPN athlete id exists; totals/unidentified rows stay plain text.

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
T9|x|add NFL player pages from ESPN athlete/gamelog or mark unsupported in registry and UI|V19,V20,I.http.nfl
T10|x|add NHL player pages from ESPN athlete/gamelog or mark unsupported in registry and UI|V19,V20,I.http.nhl
T11|x|replace hard-coded NBA fallback season `2023-24` with current/configured season or remove fallback if stale|C5,C6,I.up.nba.player_fallback,V20
T12|x|add tests for cache write policy on empty completed scoreboards per league|C6,V7,V15
T13|x|add route/render parity matrix test over all league registry entries|V1,V2,V3,V18,V19
T14|x|add normalizer tests for WNBA standings/team identity and NHL/NFL/MLB missing optional upstream fields|V12,V14,V16
T15|x|document feature matrix in README: scoreboard, game, standings, player, date/week bucket, upstream source|C8,C9,V19,V20
T16|x|set root `/` to redirect via configured default league; current default World Cup|I.http.home,V17,V19
T17|x|add World Cup soccer scoreboard/game/standings surfaces; player unsupported in registry|I.http.worldcup,V1,V3,V4,V12,V19,V20,V23

§B
id|date|cause|fix
B1|2026-06-12|NHL summary ignored `competition` in record normalization|V14
B2|2026-06-12|ESPN gamelog normalizer remapped all sports through NBA stat labels|V21
B3|2026-06-12|MLB/NFL/NHL box-score stat tables discarded athlete ids|V22
B4|2026-06-12|NHL summary `series[]` ignored; only `series{}` parsed|V14
B5|2026-06-12|NHL playoff `record` string + stale cache showed season records|V14,V15
B6|2026-06-24|World Cup match view omitted key events + mobile stats forced wide table|V23
B7|2026-06-24|World Cup nav label wrapped in narrow mobile label column|V17
