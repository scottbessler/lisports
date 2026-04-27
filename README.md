# LiSports

LiSports is a Rust web server that renders static HTML for a small sports dashboard.

## Routes

- `/` redirects to `/nba/scoreboard`
- `/nba/scoreboard` redirects to the best current scoreboard day
- `/nba/scoreboard/today` redirects to today's NBA scoreboard
- `/nba/scoreboard/:day` renders the NBA games for a date in `YYYY-MM-DD` format
- `/nba/scoreboard/:day/game/:game_id` renders the date scoreboard plus a game box score
- `/nba/standings` renders NBA standings
- `/nba/player/:player_id` renders NBA player stats
- `/mlb/scoreboard` redirects to the best current MLB scoreboard day
- `/mlb/scoreboard/today` redirects to today's MLB scoreboard
- `/mlb/scoreboard/:day` renders the MLB games for a date in `YYYY-MM-DD` format
- `/mlb/scoreboard/:day/game/:game_id` renders the date scoreboard plus MLB batting and pitching tables
- `/mlb/standings` renders MLB standings
- `/nfl/scoreboard` is a placeholder
- `/healthcheck` returns `OK`

## Development

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
