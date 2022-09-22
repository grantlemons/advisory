# Advisory Creator

<!-- ## Project Details -->

## Setup & Execution

### TLS Certificates

Create certificates via OpenSSL

- If not installed, install OpenSSL
- Navigate to the project's root directory (the one this README is in)
- Execute the following bash command and answer the given prompts

```txt
openssl req -x509 -newkey rsa:2048 -nodes -keyout backend/self_signed_certs/key.pem -out backend/self_signed_certs/cert.pem
```

### Docker

The backend and database can be run in Docker containers locally for testing ECS compatibility etc.

- Install `docker` & `docker-compose` if not already installed
- Navigate to the base directory and execute the following command

```txt
docker compose up
```

To run in detached mode, append either the `--detach` or `--wait` flag to the command.

If there have been any changes to code since the last time you ran the startup command for this project, you'll need to append the `--build` flag to force docker to implement the new changes in the container image.

In order to connect to the backend when running in Docker, send an HTTP request to the appropriate endpoint of `https://localhost/`.

## Compiling Locally

- If not installed, install rust & rustup
- Execute the following bash command in the project's root directory

```txt
cargo build --release
```

- To run, find the executable file in `/target/release/` and run it

Alternatively, to run without producing an executable execute

<pre>
cargo run --release -p advisory-backend
</pre>

In order to connect to the backend when running locally, send an HTTP request to the appropriate endpoint of `https://localhost:3000/` or `https://localhost:7878/`.

## Testing

- Navagate to the project's root directory
- Startup database with the following command

```txt
docker compose up database --wait
```

- Run all unit & integration tests with the following command

```txt
cargo test --release
```
