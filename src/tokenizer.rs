use std::fmt::Formatter;
use std::fmt::Debug;

pub enum WeTokenType {
	COMMA,
    L_PARENTHESIS,
    R_PARENTHESIS,
    L_CURLY_BRACE,
    R_CURLY_BRACE,
	L_SQUARE_BRACKET,
	R_SQUARE_BRACKET,
    ASSIGN,
    REL_OP,
    MATH_OP,
    SEMICOLON,
    COLON,
    STRING,
	Comment,
    NUMBER,
    ID_KEYWORD,
}

impl Debug for WeTokenType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			WeTokenType::COMMA => write!(f, "COMMA"),
			WeTokenType::L_PARENTHESIS => write!(f, "L_PARENTHESIS"),
			WeTokenType::R_PARENTHESIS => write!(f, "R_PARENTHESIS"),
			WeTokenType::L_CURLY_BRACE => write!(f, "L_CURLY_BRACE"),
			WeTokenType::R_CURLY_BRACE => write!(f, "R_CURLY_BRACE"),
			WeTokenType::L_SQUARE_BRACKET => write!(f, "L_SQUARE_BRACKET"),
			WeTokenType::R_SQUARE_BRACKET => write!(f, "R_SQUARE_BRACKET"),
			WeTokenType::ASSIGN => write!(f, "ASSIGN"),
			WeTokenType::REL_OP => write!(f, "REL_OP"),
			WeTokenType::MATH_OP => write!(f, "MATH_OP"),
			WeTokenType::SEMICOLON => write!(f, "SEMICOLON"),
			WeTokenType::COLON => write!(f, "COLON"),
			WeTokenType::STRING => write!(f, "STRING"),
			WeTokenType::Comment => write!(f, "Comment"),
			WeTokenType::NUMBER => write!(f, "NUMBER"),
			WeTokenType::ID_KEYWORD => write!(f, "ID_KEYWORD"),
		}
	}
}

impl PartialEq for WeTokenType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

pub struct WeToken {
	pub string_value: String,
	pub token_type: WeTokenType,
	pub source_file: String,
	pub line_number: usize,
}

impl Debug for WeToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "WeToken {{ \"{}\", token_type: {:?}, source_file: \"{}\", line_number: {} }}", self.string_value, self.token_type, self.source_file, self.line_number)
	}
}

fn next_2_match_token(file_name: &str, line_number: usize, token_string: &str) -> Option<WeToken> {
	match token_string {
		"//" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::Comment,
			source_file: file_name.to_string(),
			line_number,
		}),
		"==" | "!=" | "<=" | ">=" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::REL_OP,
			source_file: file_name.to_string(),
			line_number,
		}),
		_ => None
	}
}

fn next_1_match_token(file_name: &str, line_number: usize, token_string: &str) -> Option<WeToken> {
	match token_string {
		"," => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::COMMA,
			source_file: file_name.to_string(),
			line_number,
		}),
		"(" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::L_PARENTHESIS,
			source_file: file_name.to_string(),
			line_number,
		}),
		")" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::R_PARENTHESIS,
			source_file: file_name.to_string(),
			line_number,
		}),
		"{" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::L_CURLY_BRACE,
			source_file: file_name.to_string(),
			line_number,
		}),
		"}" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::R_CURLY_BRACE,
			source_file: file_name.to_string(),
			line_number,
		}),
		"[" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::L_SQUARE_BRACKET,
			source_file: file_name.to_string(),
			line_number,
		}),
		"]" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::R_SQUARE_BRACKET,
			source_file: file_name.to_string(),
			line_number,
		}),
		"=" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::ASSIGN,
			source_file: file_name.to_string(),
			line_number,
		}),
		"+" | "-" | "*" | "/" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::MATH_OP,
			source_file: file_name.to_string(),
			line_number,
		}),
		";" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::SEMICOLON,
			source_file: file_name.to_string(),
			line_number,
		}),
		":" => Some(WeToken {
			string_value: token_string.to_string(),
			token_type: WeTokenType::COLON,
			source_file: file_name.to_string(),
			line_number,
		}),
		_ => None
	}
}

pub fn tokenize(source_file_name: &str) -> Vec<WeToken> {
	let mut tokens: Vec<WeToken> =  Vec::new();
	let mut current_position: usize = 0;
	let mut current_line: usize = 1;
	let input_file: String = std::fs::read_to_string(source_file_name).expect("Failed to read source file");
	let total_length: usize = input_file.len();

	while current_position < total_length {
		print!("Current position: {}, Current line: {}\n", current_position, current_line);
		let current_char: char = input_file.chars().nth(current_position).unwrap();

		if current_char.is_whitespace() {
			if current_char == '\n' {
				current_line += 1;
			}
			current_position += 1;
			continue;
		}


		if let Some(token) = next_2_match_token(source_file_name.clone(), current_line, &input_file[current_position..current_position + 2]) {
			current_position += token.string_value.len();
			
			if token.token_type == WeTokenType::Comment {
				// We now need to skip the rest of the line since it's a comment
				while current_position < total_length && input_file.chars().nth(current_position).unwrap() != '\n' {
					current_position += 1;
				}
				continue; // Skip adding the comment token to the list of tokens
			}

			tokens.push(token);
		}

		if let Some(token) = next_1_match_token(source_file_name.clone(), current_line, &input_file[current_position..current_position + 1]) {
			current_position += token.string_value.len();
			tokens.push(token);
		}
	}

	tokens
}