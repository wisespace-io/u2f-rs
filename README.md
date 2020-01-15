# Rust FIDO U2F Library &emsp; [![Build Status]][travis] [![Latest Version]][crates.io] [![MIT licensed]][MIT] [![Apache-2.0 licensed]][APACHE]

[Build Status]: https://travis-ci.org/wisespace-io/u2f-rs.png?branch=master
[travis]: https://travis-ci.org/wisespace-io/u2f-rs
[Latest Version]: https://img.shields.io/crates/v/u2f.svg
[crates.io]: https://crates.io/crates/u2f
[MIT licensed]: https://img.shields.io/badge/License-MIT-blue.svg
[MIT]: ./LICENSE-MIT
[Apache-2.0 licensed]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[APACHE]: ./LICENSE-APACHE

## u2f-rs

Rust [FIDO U2F](https://fidoalliance.org/specifications/download/) library is a simple server side implementation to register and check signatures provided by U2F clients/devices. See [U2F Technical Overview](https://developers.yubico.com/U2F/Protocol_details/Overview.html)

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
u2f = "0.2"
```

Make sure that you have read [Using a U2F library](https://developers.yubico.com/U2F/Libraries/Using_a_library.html) before continuing.

See provided [example](https://github.com/wisespace-io/u2f-rs/tree/master/example)
