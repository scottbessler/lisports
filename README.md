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
- `/mlb/scoreboard` and `/nfl/scoreboard` are placeholders
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

The Docker image builds the Rust binary and starts it with `./start.sh`. Fly mounts persistent cache data at `/data`, which matches the production `DATA_PATH`.
