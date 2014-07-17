use std::string::String;

pub enum Token {
	IdentifierToken(String),
	LeftParenToken,
	RightParenToken,
	LeftCurlyToken,
	RightCurlyToken,
	IfToken,
	ElseToken,
	WhileToken,
	AssignmentToken,
	NumberToken(f64),
	SemicolonToken,
	EndOfFileToken,
	ReturnToken
}

impl Token {
	pub fn is_eof(&self) -> bool {
		true
	}
}

type SawDecimal = bool;

enum ScannerState {
    CharState(char),
    WordState(String),
    NumberState(String, SawDecimal)
}

impl ScannerState {
	fn init(c: char) -> ScannerState {
		if is_symbol_terminating_character(c) {
			CharState(c)
		} else if c.is_digit() || c == '.' {
			NumberState(String::from_char(1, c), c == '.')
		} else {
			WordState(String::from_char(1, c))
		}
	}

	fn to_token(&self) -> Token {
		match *self {
			CharState(c) => {
				match c {
					'(' => LeftParenToken,
					')' => RightParenToken,
					'{' => LeftCurlyToken,
					'}' => RightCurlyToken,
					'=' => AssignmentToken,
					';' => SemicolonToken,
					_ => ReturnToken,
				}
			},
			WordState(ref word) => {
				match word.as_slice() {
					"if" => IfToken,
					"else" => ElseToken,
					"while" => WhileToken,
					_ => IdentifierToken(word.clone())
				}
			},
			NumberState(ref number, _) => {
				NumberToken(from_str::<f64>(number.as_slice()).unwrap())
			}
		}
	}

	fn process_char(&mut self, c: char) -> Result<Option<Token>, &'static str> {
		match *self {
			CharState(_) => {
				Ok(Some(self.to_token()))
			},
			WordState(ref mut word) => {
				word.grow(1, c);
				Ok(None)
			},
			NumberState(ref mut num_str, saw_decimal) => {
				if c == '.' {
					if saw_decimal {
						Err("Invalid number")
					} else {
						// TODO: Extract common code
						if is_symbol_terminating_character(c) {
							// TODO: Return token
							// TODO: Figure out why self.to_token() can't be called.
							//       Seems to be related to using String in enum...
							Ok(None)
						} else {
							num_str.grow(1, c);
							Ok(None)
						}
					}
				} else if c.is_digit() {
					if is_symbol_terminating_character(c) {
						// TODO: Return token
						Ok(None)
					} else {
						num_str.grow(1, c);
						Ok(None)
					}
				} else {
					Err("Invalid number")
				}
			}
		}
	}
}

fn is_symbol_terminating_character(c: char) -> bool {
	match c {
		'(' => true,
		')' => true,
		'{' => true,
		'}' => true,
		'=' => true,
		';' => true,
		_  	=> false
	}
}

pub struct Scanner {
    current_char: Option<char>
}

impl Scanner {
	pub fn new() -> Scanner {
		Scanner { current_char: None }
	}

	fn read_char(&mut self, buffer: &mut Buffer) -> bool {
		match buffer.read_char() {
			Err(_) => { false }
			Ok(c) => {
				self.current_char = Some(c);
				true
			}
		}
	}

	// This is the primary interface for the scanner, which will be invoked by
	// the parser.
	pub fn get_symbol(&mut self, buffer: &mut Buffer) ->
			Result<Token, &'static str> {
		
		let mut token: Option<Token> = None;

		// If character not set, read first character
		if self.current_char.is_none() {
			if !self.read_char(buffer) {
				return Err("Failed to read character!");
			}
		}

		let mut state = ScannerState::init(self.current_char.unwrap());

		while token.is_none() {
			match state.process_char(self.current_char.unwrap()) {
				Err(_) => { return Err("Oops!"); }
				Ok(t) => { token = t; }
			}

			// Read the next character, if a token is not yet constructed
			if token.is_none() {
				if !self.read_char(buffer) {
					return Err("Failed to read character!");
				}
			}
		}

		Ok(token.unwrap())
	}
}