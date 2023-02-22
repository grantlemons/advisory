FROM rust as builder
WORKDIR /usr/src/backend
COPY ./Cargo.toml .
COPY ./src/lib ./src/lib
COPY ./benches ./benches
RUN echo "fn main() {}\n" > dummy.rs
RUN sed -i 's/src\/main.rs/dummy.rs/' Cargo.toml
RUN cargo build --release
COPY . .
RUN cargo build --release --offline

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/backend/self_signed_certs/ /usr/src/backend/self_signed_certs/
COPY --from=builder /usr/src/backend/target/release/advisory_backend_bin /usr/local/bin/advisory_backend_bin
EXPOSE 80

CMD ["advisory_backend_bin"]
