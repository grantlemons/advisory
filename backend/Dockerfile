FROM rust as builder
WORKDIR /usr/src/backend
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/backend/self_signed_certs/ /usr/src/backend/self_signed_certs/
COPY --from=builder /usr/local/cargo/bin/advisory-backend /usr/local/bin/advisory-backend
EXPOSE 7878
EXPOSE 3000

CMD ["advisory-backend"]
