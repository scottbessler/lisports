FROM rust:1.95-slim AS build

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

ENV DATA_PATH=/data
ENV PORT=8080

RUN mkdir -p /data

WORKDIR /app

COPY --from=build /app/target/release/lisports /app/lisports
COPY public ./public

ENTRYPOINT ["/app/lisports"]
