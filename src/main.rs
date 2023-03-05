use clap::Parser;
use rusty_ulid::Ulid;
use std::str::FromStr;
use time::macros::format_description;
use time::{Date, OffsetDateTime};
use uuid::Uuid;

#[derive(Clone, Debug)]
struct DateArg {
	inner: Date,
}

impl DateArg {
	fn timestamp(&self) -> u64 {
		let dt = OffsetDateTime::now_utc().replace_date(self.inner);
		dt.unix_timestamp() as u64 * 1_000 + dt.millisecond() as u64
	}
}

impl FromStr for DateArg {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let format = format_description!("[year]-[month]-[day]");
		let inner = Date::parse(s, &format).map_err(|e| e.to_string())?;
		if inner.year() < 1970 {
			return Err(format!("{s}: dates before 1970 are not supported"));
		}
		Ok(Self { inner })
	}
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long, default_value_t = 1, help = "Number of ulid to generate")]
	nb: usize,
	#[arg(short, long, help = "Display the ulid using the uuid format")]
	uuid: bool,
	#[arg(short, long, value_parser = clap::value_parser!(DateArg), help = "Generate the uuid using the specified date")]
	date: Option<DateArg>,
	#[arg(
		short,
		long,
		help = "Use a monotonic increment when generating multiple ulid"
	)]
	monotonic: bool,
}

fn main() {
	let args = Args::parse();
	let mut previous_ulid = None;
	for _ in 0..args.nb {
		let ulid = generate(&args, previous_ulid);
		previous_ulid = Some(ulid);
		display(&args, ulid);
	}
}

#[inline]
fn generate(args: &Args, previous_ulid: Option<Ulid>) -> Ulid {
	if args.monotonic {
		if let Some(prev) = previous_ulid {
			return match &args.date {
				Some(dt) => Ulid::next_monotonic_from_timestamp_with_rng(
					prev,
					dt.timestamp(),
					&mut rand::thread_rng(),
				),
				None => Ulid::next_monotonic(prev),
			};
		}
	}
	match &args.date {
		Some(dt) => Ulid::from_timestamp_with_rng(dt.timestamp(), &mut rand::thread_rng()),
		None => Ulid::generate(),
	}
}

#[inline]
fn display(args: &Args, ulid: Ulid) {
	let ulid_str = if args.uuid {
		Uuid::from_u128(ulid.into()).to_string()
	} else {
		ulid.to_string()
	};
	println!("{ulid_str}");
}
