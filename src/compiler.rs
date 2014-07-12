use std::io::{File, BufferedReader};

mod scanner;

fn main() {
	println!("Welcome to my compiler!");
	let file = File::open(&Path::new("testdata/program1.c"));
	let mut reader = BufferedReader::new(file);
	scanner::getsymbol(& mut reader);
}