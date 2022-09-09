FROM rust as builder
WORKDIR /usr/src/backend
COPY ./Cargo.toml .
RUN echo "fn main() {}\n" > dummy.rs
RUN sed -i 's/src\/main.rs/dummy.rs/' Cargo.toml
RUN cargo build --release
COPY . .
RUN cargo build --offline --release

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/backend/self_signed_certs/ /usr/src/backend/self_signed_certs/
COPY --from=builder /usr/src/backend/target/release/advisory-backend /usr/local/bin/advisory-backend
EXPOSE 7878
EXPOSE 3000

CMD ["advisory-backend"]
