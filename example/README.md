# Nightly Rust

The example uses [Rocket](https://github.com/SergioBenitez/Rocket/) so it requires Nightly Rust.

## Usage

The certificate/private key pair used can be generated via openssl:

```
openssl req -x509 -newkey rsa:4096 -nodes -sha256 -days 3650 -keyout key.pem -out cert.pem
```

Update Rocket.toml with the proper location

```
[global.tls]
certs = "private/cert.pem"
key = "private/key.pem"
```

The certificate is self-signed. You will need to trust it directly for your browser to refer to the connection as secure.

Build and open the demo app

```
https://localhost:30443
```