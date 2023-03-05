use clap::Parser;
use rusty_ulid::Ulid;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long, default_value_t = 1, help = "Number of ulid to generate")]
	nb: usize,
	#[arg(short, long, help = "Display the ulid using the uuid format")]
	uuid: bool,
}

fn main() {
	let args = Args::parse();
	for _ in 0..args.nb {
		let ulid = generate(&args);
		display(&args, ulid);
	}
}

#[inline]
fn generate(args: &Args) -> Ulid {
	Ulid::generate()
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
