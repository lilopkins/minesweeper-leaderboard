FROM rust AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim AS runner
COPY --from=builder /usr/local/cargo/bin/minesweeper-leaderboard /usr/local/bin/minesweeper-leaderboard
WORKDIR /srv
EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
COPY Rocket.docker.toml Rocket.toml
RUN mkdir -p /data
VOLUME /data
CMD [ "minesweeper-leaderboard" ]

