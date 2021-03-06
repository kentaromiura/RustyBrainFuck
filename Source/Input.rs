// Ideally we should be able to pass a string or a file,
// if the file exists then we will use FS API otherwise Strings API.
use std::io::File;
use std::string::String;
use std::path::Path;
#[allow(non_snake_case_functions)]
pub fn getInput(input: String) -> String {
	let path = Path::new(input.as_slice());

	if path.exists() {
		let contents = File::open(&path).read_to_string();
		match contents {
				Ok(text) => return text,
				Err(err) => {print!("Error: {}", err);},
		}
	}
	return input;
}
