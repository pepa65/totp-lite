[![version](https://img.shields.io/crates/v/totp-lite_.svg)](https://crates.io/crates/totp-lite_)
[![build](https://github.com/pepa65/totp-lite/actions/workflows/rust.yml/badge.svg)](https://github.com/pepa65/totp-lite/actions/workflows/rust.yml)
[![dependencies](https://deps.rs/repo/github/pepa65/totp-lite/status.svg)](https://deps.rs/repo/github/pepa65/totp-lite)
[![docs](https://img.shields.io/badge/docs-totp-lite_-blue.svg)](https://docs.rs/crate/totp-lite_/latest)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/pepa65/totp-lite/blob/master/LICENSE)
[![downloads](https://img.shields.io/crates/d/totp-lite_.svg)](https://crates.io/crates/totp-lite_)
# totp-lite_ 2.2.0
**A simple, correct TOTP library**

## Replacement for the totp-lite crate
**This repo is cloned from github.com/fosskers/totp-lite in order to modernize it and bring it up to date**

To use this crate instead of the unmaintained `totp-lite`, add this to `Cargo.toml`:
```
[dependencies]
totp-lite = { package = "totp-lite_", version = "2" }
```

## Digest
Time-based One-time Passwords are a useful way to authenticate a client,
since a valid password expires long before it could ever be guessed by an
attacker. This library provides an implementation of TOTP that matches its
specification [RFC6238], along with a simple interface.

## Usage
The standard `totp` function uses a time step of `30` seconds and
produces `8` digits of output. Use `totp_custom` for other values.

```rust
use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp, Sha512};

// Negotiated between you and the authenticating service.
let secret: &[u8] = b"secret";

// The number of seconds since the Unix Epoch.
let time: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

// Specify the Hash algorithm with a type parameter: Sha1, Sha256 or Sha512.
let code: String = totp::<Sha512>(secret, time);
assert_eq!(8, code.len());
```

## Resources
* [RFC6238: TOTP](https://tools.ietf.org/html/rfc6238)
* [RFC6238 Errata](https://www.rfc-editor.org/errata_search.php?rfc=6238)
* License: MIT
