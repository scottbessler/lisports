# LiSports

LiSports is a Rust web server that renders static HTML for a small sports dashboard.

## Routes

- `/` redirects to `/nba/scoreboard`
- `/nba/scoreboard` redirects to `/nba/scoreboard/today`
- `/nba/scoreboard/today` renders today's NBA scoreboard
- `/nba/scoreboard/:day` renders the NBA games for a date in `YYYY-MM-DD` format
- `/nba/scoreboard/:day/game/:game_id` renders the date scoreboard plus a game box score
- `/nba/standings` renders NBA standings
- `/nba/player/:player_id` renders NBA player stats
- `/wnba/scoreboard` redirects to `/wnba/scoreboard/today`
- `/wnba/scoreboard/today` renders today's WNBA scoreboard
- `/wnba/scoreboard/:day` renders the WNBA games for a date in `YYYY-MM-DD` format
- `/wnba/scoreboard/:day/game/:game_id` renders the date scoreboard plus a game box score
- `/wnba/standings` renders WNBA standings
- `/wnba/player/:player_id` renders WNBA player stats
- `/mlb/scoreboard` redirects to `/mlb/scoreboard/today`
- `/mlb/scoreboard/today` renders today's MLB scoreboard
- `/mlb/scoreboard/:day` renders the MLB games for a date in `YYYY-MM-DD` format
- `/mlb/scoreboard/:day/game/:game_id` renders the date scoreboard plus MLB batting and pitching tables
- `/mlb/standings` renders MLB standings
- `/mlb/player/:player_id` renders MLB player stats
- `/nfl/scoreboard` redirects to `/nfl/scoreboard/today`
- `/nfl/scoreboard/today` renders the latest NFL week with games played
- `/nfl/scoreboard/:week` renders the NFL games for week `1` to `23`, where `19` to `23` are playoff weeks
- `/nfl/scoreboard/:week/game/:game_id` renders the week scoreboard plus NFL stat tables
- `/nfl/standings` renders NFL standings
- `/nfl/player/:player_id` renders NFL player stats
- `/nhl/scoreboard` redirects to `/nhl/scoreboard/today`
- `/nhl/scoreboard/today` renders today's NHL scoreboard
- `/nhl/scoreboard/:day` renders the NHL games for a date in `YYYY-MM-DD` format
- `/nhl/scoreboard/:day/game/:game_id` renders the date scoreboard plus NHL stat tables
- `/nhl/standings` renders NHL standings
- `/nhl/player/:player_id` renders NHL player stats
- `/worldcup/scoreboard` redirects to `/worldcup/scoreboard/today`
- `/worldcup/scoreboard/today` renders today's World Cup scoreboard
- `/worldcup/scoreboard/:day` renders the World Cup matches for a date in `YYYY-MM-DD` format
- `/worldcup/scoreboard/:day/game/:game_id` renders the date scoreboard plus World Cup match stats
- `/worldcup/standings` renders World Cup standings
- `/healthcheck` returns `OK`

## Feature Matrix

| League | Scoreboard | Game | Standings | Player | Bucket | Upstream |
| --- | --- | --- | --- | --- | --- | --- |
| NBA | yes | yes | yes | yes | date | NBA live scoreboard, ESPN site APIs, NBA fallback player endpoint |
| WNBA | yes | yes | yes | yes | date | ESPN site APIs |
| MLB | yes | yes | yes | yes | date | ESPN site APIs |
| NFL | yes | yes | yes | yes | week `1..23` | ESPN site APIs |
| NHL | yes | yes | yes | yes | date | ESPN site APIs |
| World Cup | yes | yes | yes | no | date | ESPN site APIs (`soccer/fifa.world`) |

## Development

Run the Rust service in dev mode to restart automatically when Rust sources, static assets, or Cargo metadata change:

```sh
./dev.sh
```

To watch different paths or change the poll interval:

```sh
WATCH_PATHS="src public" POLL_INTERVAL=0.5 ./dev.sh
```

```sh
cargo run
```

The server listens on `PORT`, defaulting to `8080`.

```sh
PORT=3000 cargo run
```

File-backed JSON cache is stored in `DATA_PATH`, defaulting to `data`.

```sh
DATA_PATH=/tmp/lisports-data cargo run
```

## Checks

```sh
cargo fmt --check
cargo check
```

## Deployment

The Docker image builds the Rust binary and starts `/app/lisports` directly. Fly mounts persistent cache data at `/data`, which matches the production `DATA_PATH`.
