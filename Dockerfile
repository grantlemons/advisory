FROM rust as builder
WORKDIR /usr/src/advisory
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/advisory /usr/local/bin/advisory
EXPOSE 3000

ENV ROCKET_CONFIG=/home/Rocket.toml
CMD ["advisory"]
