[![Workflow Status](https://github.com/pepa65/totp-lite/workflows/Rust/badge.svg)](https://github.com/pepa65/totp-lite/actions?query=workflow%3A%22Rust%22)
[![](https://img.shields.io/crates/v/totp-lite_.svg)](https://crates.io/crates/totp-lite_)
# totp-lite_
**A simple, correct TOTP library**

Time-based One-time Passwords are a useful way to authenticate a client,
since a valid password expires long before it could ever be guessed by an
attacker. This library provides an implementation of TOTP that matches its
specification [RFC6238], along with a simple interface.

## Replacement for the totp-lite crate
**This repo is cloned from github.com//totp-lite in order to modernize it and bring it up to date**

To use this crate instead of the unmaintained `totp-lite`, add this to `Cargo.toml`:
```
[dependencies]
totp-lite = { package = "totp-lite_", version = "2" }
```

## Usage
The `totp` function is likely what you need. It uses the default time step
of 30 seconds and produces by default 8 digits of output:

```rust
use std::time::{SystemTime, UNIX_EPOCH};
use totp_lite::{totp, Sha1};

// Negotiated between you and the authenticating service.
let password: &[u8] = b"secret";

// The number of seconds since the Unix Epoch.
let seconds: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

// Specify the desired Hash algorithm via a type parameter.
// `Sha512` and `Sha256` are also available.
let result: String = totp::<Sha1>(password, seconds);
assert_eq!(8, result.len());
```

For full control over how the algorithm is configured, consider
`totp_custom`.

## Resources
* [RFC6238: TOTP][RFC6238]
* [RFC6238 Errata](https://www.rfc-editor.org/errata_search.php?rfc=6238)

[RFC6238]: https://tools.ietf.org/html/rfc6238

License: MIT
