use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::env;

fn main(){
	let args: Vec<String> = env::args().collect();

	let mut TEXT = "Rust is tanoshii";

	if args.len() > 1 {
		TEXT = &args[1];
	}

	let stdout = stdout();
	let message = String::from(TEXT);
	let width = message.chars().count();

	let mut writer = BufWriter::new(stdout.lock());
	say(TEXT.as_bytes(), width, &mut writer).unwrap();
}
