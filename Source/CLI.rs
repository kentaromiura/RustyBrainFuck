// The CLI will read STDIN and output on STDOUT/STDERR,
// allowing redirection to happens
use std::os;
use std::string::String;

pub struct Options {
	program: String,
	errors: bool
}

impl Options{
	fn new(program: String, errors: bool) -> Options {
		Options { program: program, errors: errors }
	}
	pub fn hasErrors(&self) -> bool {
		return self.errors;
	}
	pub fn getProgram(&self) -> String {
		return self.program.to_string();;
	}
}

pub fn main() -> Options{
	let args = os::args();

	if args.len() < 2 {
		println!("You need to pass the program source or filepath");
		return Options::new(String::from_str(""), true)
	}

	return Options::new(args[1].to_string(), false);
}
