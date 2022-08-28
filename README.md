# Advisory Creator

## Project Details

## Installation

### TLS Certificates

Create certificates via OpenSSL

- If not installed, install OpenSSL
- Navigate to the project's root directory (the one this README is in)
- Create certificates with the following bash command

```bash
openssl req -x509 -newkey rsa:2048 -nodes -keyout self_signed_certs/key.pem -out self_signed_certs/cert.pem
```

## Compiling

- Install rust via rustup
- Run the following bash command in the project directory

```bash
cargo build --release
```

- To run, find the executable file in `/target/release/` and run it

Alternatively, to run without producing an executable execute

```bash
cargo run
```
