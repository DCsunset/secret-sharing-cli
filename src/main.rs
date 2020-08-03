extern crate sharks;
extern crate clap;
extern crate base64;

use sharks::{Share, Sharks};
use clap::{App, Arg, SubCommand};
use std::fs;
use core::convert::TryFrom;

fn main() {
	let matches = App::new("Secret Sharing")
		.version("0.1.0")
		.about("A secret sharing command line tool using Shamir's Secret Sharing")
		.subcommand(
			SubCommand::with_name("split")
				.about("Split the secret")
				.arg(
					Arg::with_name("string")
						.help("The secret string")
						.short("s")
						.long("string")
						.value_name("string")
						.takes_value(true)
				)
				.arg(
					Arg::with_name("file")
						.help("Read secret from file")
						.short("f")
						.long("file")
						.value_name("file")
						.takes_value(true)
				)
				.arg(
					Arg::with_name("threshold")
						.help("Minimum number of shares to reconstruct the secret")
						.short("t")
						.long("threshold")
						.value_name("threshold")
						.takes_value(true)
						.required(true)
				)
				.arg(
					Arg::with_name("number")
						.help("Number of shares to generate (up to 255)")
						.short("n")
						.long("number")
						.value_name("number")
						.takes_value(true)
						.required(true)
				)
		)
		.subcommand(
			SubCommand::with_name("recover")
				.about("Recover the secret")
				.arg(
					Arg::with_name("threshold")
						.help("Minimum number of shares to reconstruct the secret (optional)")
						.short("t")
						.long("threshold")
						.value_name("threshold")
						.takes_value(true)
				)
				.arg(
					Arg::with_name("string")
						.help("Print secret as string")
						.short("s")
						.long("string")
				)
				.arg(
					Arg::with_name("file")
						.help("Write secret to file")
						.short("f")
						.long("file")
						.value_name("file")
						.takes_value(true)
				)
				.arg(
					Arg::with_name("shares")
						.help("Shares to recover the secret")
						.required(true)
						.multiple(true)
						.index(1)
				)
		)
		.get_matches();

	if let Some(subcommand) = matches.subcommand_matches("split") {
		let number: usize = subcommand.value_of("number")
			.unwrap()
			.parse()
			.expect("Invalid number!");

		let threshold: u8 = subcommand.value_of("threshold")
			.unwrap()
			.parse()
			.expect("Invalid threshold!");
		
		if !subcommand.is_present("string") && !subcommand.is_present("file") {
			panic!("No secret is provided!")
		}

		let data: Vec<u8>;
		let secret = if subcommand.is_present("string") {
			subcommand.value_of("string").unwrap().as_bytes()
		} else {
			let filename = subcommand.value_of("file").unwrap();
			data = fs::read(filename).expect("Unable to read the secret file");
			data.as_slice()
		};

		let sharks = Sharks(threshold);
		let dealer = sharks.dealer(secret);
		for share in dealer.take(number) {
			let share_bytes = Vec::from(&share);
			println!("{}", base64::encode(share_bytes));
		}
	}
	else if let Some(subcommand) = matches.subcommand_matches("recover") {
		let threshold: u8 = subcommand.value_of("threshold")
			.unwrap_or("0")
			.parse()
			.expect("Invalid threshold!");
		
		let shares: Vec<Share> = subcommand.values_of("shares")
			.unwrap()
			.map(|share| {
				let share_bytes = base64::decode(share).expect("Invalid share");
				Share::try_from(share_bytes.as_slice()).unwrap()
			})
			.collect();
		
		let sharks = Sharks(threshold);
		let secret = sharks.recover(&shares).unwrap();

		if subcommand.is_present("string") {
			println!("{}", std::str::from_utf8(secret.as_slice()).expect("Invalid string result"));
		}
		else if subcommand.is_present("file") {
			let filename = subcommand.value_of("file").unwrap();
			fs::write(filename, secret).unwrap();
		}
		else {
			panic!("No output format specified!");
		}
	}
}
