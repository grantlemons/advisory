FROM rust as builder
WORKDIR /usr/src/advisory-backend
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/advisory-backend /usr/local/bin/advisory-backend
EXPOSE 3000

CMD ["advisory-backend"]
