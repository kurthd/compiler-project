use std::io::{File, BufferedReader};

mod scanner;

fn main() {
	let file = File::open(&Path::new("testdata/program1.c"));
	let mut reader = BufferedReader::new(file);
	let mut s = scanner::Scanner::new();

	let mut current_token: Option<scanner::Token> = None;

	while current_token.is_none() || !current_token.unwrap().is_eof() {
		match s.get_symbol(& mut reader) {
			Err(_) => {
				println!("Error encountered")
				return;
			}
			Ok(token) => {
				current_token = Some(token);
			}
		}
	}
}