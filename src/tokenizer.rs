pub enum WeTokenType {
	COMMA,
    R_BRACKET,
    L_BRACKET,
    R_BRACE,
    L_BRACE,
    ASSIGN,
    REL_OP,
    MATH_OP,
    SEMICOLON,
    NUMBER,
    ID_KEYWORD,
    COLON,
    STRING
}


pub struct WeToken {
	pub string_value: String,
	pub token_type: WeTokenType,
	pub source_file: String,
	pub line_number: usize,
}

pub fn tokenize(source_file: &str) -> Vec<WeToken> {
	let tokens: Vec<WeToken> =  Vec::new();
	let mut current_position: usize = 0;
	let mut current_line: usize = 1;
	let input_file: String = std::fs::read_to_string(source_file).expect("Failed to read source file");

	let total_length: usize = input_file.len();

	while current_position < total_length {
		let current_char: char = input_file.chars().nth(current_position).unwrap();

		if current_char.is_whitespace() {
			if current_char == '\n' {
				current_line += 1;
			}
			current_position += 1;
			continue;
		}
	}

	tokens
}