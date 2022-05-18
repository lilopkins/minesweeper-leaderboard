FROM rust:slim
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
EXPOSE 8000
WORKDIR /srv
COPY Rocket.docker.toml Rocket.toml
RUN mkdir -p /data
VOLUME /data
CMD [ "minesweeper-leaderboard" ]
