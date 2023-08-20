use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::env;

fn main(){
	let args: Vec<String> = env::args().collect();

	if args.len() > 1 {
		let TEXT = &args[1..].join(" ");
		GEN(String::from(TEXT));
	}else{
		GEN(String::from("Rust is tanoshii"));
	}
}

fn GEN(TEXT: String) {
	let stdout = stdout();
	let message = String::from(&TEXT);  // TEXTの所有権を保持したまま新しいStringを作成
	let width = message.chars().count();

	let mut writer = BufWriter::new(stdout.lock());
	say(TEXT.as_bytes(), width, &mut writer).unwrap();
}
