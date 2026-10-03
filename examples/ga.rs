//! A Google Authenticator compatible TOTP implementation.
//! Technical reference:
//! https://en.wikipedia.org/wiki/Google_Authenticator#Technical_description
//!
//! You can run this example as follows:
//! ```sh
//!   cargo run --example ga
//! ```

use base32ct::{Base32Unpadded, Encoding};
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, SystemTime};
use totp_lite::{Sha1, totp_custom};

fn main() {
	println!("Press Ctrl-C to exit");
	let bytes = loop {
		print!("Enter TOTP secret (16/26/32 chars): ");
		io::stdout().flush().unwrap();
		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		let input: Vec<u8> = line.bytes().filter(|&b| !b.is_ascii_whitespace()).map(|b| b.to_ascii_lowercase()).collect();
		let len = input.len();
		if len != 16 && len != 26 && len != 32 {
			println!("Invalid TOTP secret length: {len}");
			continue;
		}

		let mut decoded = [0u8; 20];
		let decoded = match Base32Unpadded::decode(&input, &mut decoded) {
			Ok(decoded) => decoded,
			Err(_) => {
				println!("Invalid Base32 secret");
				continue;
			}
		};
		break decoded.to_vec();
	};

	loop {
		// The number of seconds since the Unix Epoch, used to calcuate a TOTP secret.
		let secs: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
		println!(
			"Your TOTP code: {}",
			totp_custom::<Sha1>(
				30,     // Every 30 seconds
				6,      // 6 digit code
				&bytes, // 10, 16 or 20 bytes
				secs,   // Seconds since the Unix Epoch
			)
		);
		thread::sleep(Duration::from_secs(30 - secs % 30));
	}
}
