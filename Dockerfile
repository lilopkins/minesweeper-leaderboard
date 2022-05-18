FROM rust:slim
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .
WORKDIR /srv
EXPOSE 8000
ENV ROCKET_ADDRESS=0.0.0.0
COPY Rocket.docker.toml Rocket.toml
RUN mkdir -p /data
VOLUME /data
CMD [ "minesweeper-leaderboard" ]

