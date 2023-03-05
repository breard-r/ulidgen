use rusty_ulid::Ulid;

fn main() {
	let ulid = Ulid::generate();
	println!("{}", ulid.to_string());
}
