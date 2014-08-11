#[allow(non_snake_case_functions)]
// This is the main logic piece
// In order to be testable, I will separate it from the rest of the code
// it will have a current status layer and a next command

use std::char;
use std::io;

struct Code {
	codeIndex: uint,
	jumps: Vec<uint>
}
struct Tape {
	tapeIndex: uint,
	store: Vec<int>
}

impl Code {

}

impl Tape{

}

pub struct Interpreter {
	code: Code,
	tape: Tape,
	skipWhile: bool,
	whileCount: uint
}
#[allow(non_snake_case_functions)]
impl Interpreter {
	pub fn new() -> Interpreter {
		let store = vec![0i];
		let jumps = vec![];

		return Interpreter {
			code: Code {
				codeIndex: 0,
				jumps: jumps
			},
			tape: Tape {
				tapeIndex: 0,
				store: store
			},
			skipWhile: false,
			whileCount: 0
		};
	}
	pub fn debugGetTapeIndex(&self) -> uint {
		return self.tape.tapeIndex;
	}
	pub fn debugTapeValue(&self) -> int {
		return self.tape.store[self.tape.tapeIndex];
	}
	pub fn debug(&self) -> bool {
		print!("\n==============\nInterpreter.code.codeIndex {}\nInterpreter.tape.tapeIndex {}\n", self.code.codeIndex, self.tape.tapeIndex)
		for entry in self.tape.store.iter() {
			print!("\nentry {}\n", entry);
		}
		for jump in self.code.jumps.iter(){
			print!("\njumps {}\n", jump);
		}
		return true
	}
	pub fn nextIndex(&self) -> uint {
		return self.code.codeIndex;
	}
	fn change(& mut self, value: int) {
		*self.tape.store.get_mut(self.tape.tapeIndex) = value;
	}
	pub fn nextCommand(& mut self, command: char) -> bool {
		if self.skipWhile {
			self.code.codeIndex = self.code.codeIndex + 1;
			match command{
				'[' => {
					self.whileCount = self.whileCount + 1;
				 },
				']' => {
					self.whileCount = self.whileCount - 1;
					if self.whileCount == 0 {
						self.skipWhile = false;
					}
				 },
				_ => {
					;
				}
			}
			return true;
		}
		match command {
			'+' => {
//						+						Increment current tape value
				self.code.codeIndex = self.code.codeIndex + 1;
				let value = self.tape.store[self.tape.tapeIndex] + 1;
				self.change(value)
			},
			'-' => {
//					 -						Decrement current tape value
				self.code.codeIndex = self.code.codeIndex + 1;
				let value = self.tape.store[self.tape.tapeIndex] - 1;
				self.change(value)
			},
			'>' => {
//					 >						Process next-right tape value
				self.code.codeIndex = self.code.codeIndex + 1;

				self.tape.tapeIndex = self.tape.tapeIndex + 1;
				if self.tape.store.len() -1 < self.tape.tapeIndex {
					self.tape.store.push(0i);
				}
			},
			'<' => {
//					 <						Process next-left tape value
				self.code.codeIndex = self.code.codeIndex + 1;
				if self.tape.tapeIndex > 0 {
					self.tape.tapeIndex = self.tape.tapeIndex -1;
				}
			},
			'[' => {
//					 [						Pretest loop begins
				if self.tape.store[self.tape.tapeIndex] == 0 {
					// JUMP FORWARD
					self.skipWhile = true;
					self.whileCount = self.whileCount + 1
				} else {
					self.code.jumps.push(self.code.codeIndex)
				}
				self.code.codeIndex = self.code.codeIndex + 1;
			},
			']' => {
//					 ]						Pretest loop terminates
				let jump = self.code.jumps.pop();
				self.code.codeIndex = jump.unwrap();
			},
			'.' => {
//					 .						Send output value (as character)
				self.code.codeIndex = self.code.codeIndex + 1;
				let value = self.tape.store[self.tape.tapeIndex];
				print!("{}", char::from_u32(value as u32).unwrap());
			},
			',' => {
//					 ,						Retrieve input character (as integer)
				self.code.codeIndex = self.code.codeIndex + 1;
				let byte = io::stdin().read_byte().unwrap();
				self.change(byte as int);
			},
			_	=> {self.code.codeIndex = self.code.codeIndex + 1; },
		}
		return true
	}
}
/*
					 #						Output partial tape state (not required)
*/
pub fn init() -> Interpreter {
	return Interpreter::new()
}
