#[allow(non_snake_case_functions)]

pub mod CLI;
pub mod Input;
pub mod Interpreter;

fn main(){
	let options = CLI::main();
	if options.hasErrors() {
		print!("The program is terminated due to errors.")
		return
	}
	let code = Input::getInput(options.getProgram());
	let mut interpreter = Interpreter::init();

	let mut nextIndex = interpreter.nextIndex();
	let mut stringChars = Vec::new();

	for c in code.as_slice().chars() {
		stringChars.push(c);
	}

	while stringChars.len() -1 > nextIndex {
		let nextChar = stringChars[nextIndex];
		interpreter.nextCommand(nextChar);
		nextIndex = interpreter.nextIndex()
	}
	print!("\n");
//interpreter.debug();
}
