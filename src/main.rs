use clap::Parser;
use rusty_ulid::Ulid;
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(short, long)]
	uuid: bool,
}

fn main() {
	let args = Args::parse();

	let ulid = Ulid::generate();
	let ulid_str = if args.uuid {
		Uuid::from_u128(ulid.into()).to_string()
	} else {
		ulid.to_string()
	};

	println!("{ulid_str}");
}
