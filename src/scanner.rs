use std::string::String;


// This is the primary interface for the scanner, which will be invoked by the
// parser.
pub fn getsymbol(buffer: & mut Buffer) -> String {
	match buffer.read_char() {
		Err(_) => {}
		Ok(char) => {
			println!("{}", char);
		}
	}

	String::from_str("yep")
}

#[test]
fn getsymbol_test() {
	use std::io::{Buffer, File, BufferedReader};
	
	let file = File::open(&Path::new("testdata/program1.c"));
	let mut reader = BufferedReader::new(file);
	let symbol = getsymbol(& mut reader);
	assert!(symbol == String::from_str("yep"));
}