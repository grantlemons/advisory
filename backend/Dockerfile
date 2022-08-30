FROM rust as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/backend /usr/local/bin/backend
EXPOSE 3000

ENV ROCKET_CONFIG=/home/Rocket.toml
CMD ["backend"]
