# Advisory Creator

## Project Details

## Setup & Execution

### TLS Certificates

Create certificates via OpenSSL

- If not installed, install OpenSSL
- Navigate to the project's root directory (the one this README is in)
- Create certificates with the following bash command

```bash
openssl req -x509 -newkey rsa:2048 -nodes -keyout backend/self_signed_certs/key.pem -out backend/self_signed_certs/cert.pem
```

## Compiling

- If not installed, install rust & rustup
- Run the following bash command in the project directory

```bash
cargo build --release
```

- To run, find the appropriate executable file in `/target/release/` and run it

Alternatively, to run without producing an executable execute

<pre>
cargo run -p <i>advisory-backend</i> | <i>advisory-frontend</i>
</pre>
