use std::env;
mod tokenizer;
use tokenizer::WeToken;
use tokenizer::tokenize;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		eprintln!("Usage: welang <source_file>");
		std::process::exit(1);
	}

	let input_file_name: String = args[1].clone();
	print!("Tokenizing file: {}\n", input_file_name);
	let tokens: Vec<WeToken> = tokenize(&input_file_name);
	println!("Tokenization complete. Total tokens: {}", tokens.len());

	for token in tokens {
		println!("{:?}", token);
	}
}
