# Advisory Creator

## Installation

### TLS

Create certificates via OpenSSL

- If not installed, install OpenSSL
- Navigate to the crate's root directory (the one this README is in)
- Create certificates with the following bash command

```bash
openssl req -x509 -newkey rsa:4096 -keyout self_signed_certs/key.pem -out self_signed_certs/cert.pem
```
