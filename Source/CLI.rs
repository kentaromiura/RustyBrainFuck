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
		return String::from_str(self.program.as_slice());
	}
}

pub fn main() -> Options{
	let args = os::args();

	if args.len() < 2 {
		println!("You need to pass the program source or filepath");
		return Options::new(String::from_str(""), true)
	}

//	println!("The first argument is {}, length {}\n", args[1], args.len());
	return Options::new(String::from_str(args[1].as_slice()), false);
}
