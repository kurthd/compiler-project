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

enum ScannerState {
    CharState(char),
    WordState(String),
    NumberState(String)
}

impl ScannerState {
	fn init(c: char) -> ScannerState {
		if is_symbol_terminating_character(c) {
			CharState(c)
		} else { // TODO
			NumberState(String::new())
		}
	}

	fn process_char(&self, c: char) -> Result<Option<Token>, &'static str> {
		// Update the state
		// return token, if complete
		Ok(None)
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

	// This is the primary interface for the scanner, which will be invoked by
	// the parser.
	pub fn get_symbol(& mut self, buffer: & mut Buffer) ->
			Result<Token, &'static str> {

		
		let mut token: Option<Token> = None;

		// If character not set, read first character
		// TODO: Eliminate code duplication with a closure
		if self.current_char.is_none() {
			match buffer.read_char() {
				Err(_) => {
					return Err("Oops!");
				}
				Ok(c) => {
					self.current_char = Some(c);
				}
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
				match buffer.read_char() {
					Err(_) => {
						return Err("Oops!");
					}
					Ok(c) => {
						self.current_char = Some(c);
					}
				}
			}
		}

		Ok(token.unwrap())
	}
}