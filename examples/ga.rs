//! Common TOTP implementation (SHA1, 6 digits, 30s period)
//!
//! Run this example as follows:
//! ```sh
//!   cargo run --example ga
//! ```

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, SystemTime};
use totp_lite::{Sha1, totp_custom};

fn main() {
	println!("Press Ctrl-C to exit");
	let mut output = vec![0; 50];
	let bytes = loop {
		print!("Enter TOTP secret: ");
		io::stdout().flush().unwrap();
		let mut line = String::new();
		io::stdin().read_line(&mut line).unwrap();
		line = line.trim().to_ascii_lowercase().chars().filter(|&b| !b.is_whitespace()).collect();
		let res = base32::RFC4648_LOWER_NOPAD.decode_buf(&line, &mut output);
		match res {
			Ok(bytes) => break bytes,
			Err(base32::Error::InvalidLength) => println!("Invalid length: {}", line.len()),
			Err(base32::Error::InvalidSymbol { offset, symbol }) => println!("Invalid Base32 character '{}' at position {}", symbol as char, offset + 1),
			Err(base32::Error::InvalidPadding) => println!("Invalid Base32 padding"),
			Err(base32::Error::BufferTooSmall { expected }) => println!("Base32 output buffer needs {} bytes", expected),
		};
	};

	loop {
		// The number of seconds since the Unix Epoch, used to calcuate a TOTP secret.
		let secs: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
		println!(
			"Your TOTP code: {}",
			totp_custom::<Sha1>(
				30,    // Every 30 seconds
				6,     // 6 digit code
				bytes, // Decoded base32
				secs,  // Seconds since the Unix Epoch
			)
		);
		thread::sleep(Duration::from_secs(30 - secs % 30));
	}
}
