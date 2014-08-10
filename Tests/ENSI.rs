#[path="../Source/Interpreter.rs"]
mod Interpreter;


/*
The BrainF*** Language Specification (01 January 2002)

1.  Language Assumptions:  The BrainF*** language assumes a Harvard
Architecture system (i.e., code and data are stored separately) with a
sequentially-addressable storage area for data.  If such does not
exist, it must be supplied (simulated) by an underlying system layer.

2.  Program Layout:  BrainF*** is a freeform language, which may be
presented and/or formatted using any system amenable to data transfer
which preserves the readability of the keywords.  For convenience, it
is suggested that on most systems that this format be a sequentially-
stored file, which the BrainF*** programming beginning at the first
byte and continuing for the length of the file.

3.  Program Execution:  A BrainF*** program begins at the first
recognized keyword (see "Program Layout," above, and "Arithmetic,"
"Addressing," "Control Structures," and "Communications," below), and
executes each such keyword in the program in the sequential order they
are listed, skipping over any comments (see "Comments," below).  This
basic control flow is modified by BrainF***'s control structures (see
"Control Structures," below) and is halted when the implementation
executes a keyword which is followed by no other recognizable keywords.

4.  Comments:  A BrainF*** implementation will ignore anything in a
source file which is not a standard keyword.  Any such content may be
used as a comment.  Beyond that, no stylistic approaches regarding the
form of such comments are required or endorsed.
*/



#[test]
fn must_ignore_any_non_command_characters() {
		let mut interpreter = Interpreter::init();

		interpreter.nextCommand('£');
		if interpreter.debugTapeValue() != 0 {
			fail!("the interpreter don't ignore comments")
		}
}
/*

5.  Data Types:  BrainF*** has only a single data type, the
implementation- defined integer.  However, the integer may be
interpreted as one of three data types:  Integers, Characters, and
Booleans.
  a) Mapping from integer to character involves examining the least-
     significant bits of the integer.  Character values, however, are
     directly usable as integers.
  b) Mapping from integer to boolean simply involves comparison with
     (integer) zero.  A zero integer is equal to FALSE, and nonzero,
     TRUE.  Booleans cannot be mapped back to integers.
*/
#[test]
fn must_map_integer_to_character(){
// ???
}
#[test]
fn must_map_integer_to_boolean(){
		let mut interpreter = Interpreter::init();
		interpreter.nextCommand('[');
		interpreter.nextCommand('+');
		interpreter.nextCommand(']');
		if interpreter.debugTapeValue() != 0 {
			fail!("the interpreter failed to evaluate 0 to false");
		}
		interpreter.nextCommand('+');
		interpreter.nextCommand('[');
		if interpreter.debugTapeValue() != 1 {
			fail!("the interpreter failed to evaluate 1 to true");
		}
		interpreter.nextCommand('-');
		interpreter.nextCommand(']');
		if interpreter.debugTapeValue() != 0 {
			fail!("the interpreter failed to evaluate 1 to true");
		}
}
/*
6.  Arithmetic:  Calculation in BrainF*** is astoundingly simple.
There are only two operations:  Increment (represented by the keyword
"+"), and Decrement (represented by the keyword "-").  When executed,
the operation is applied to the "current" value.
*/
#[test]
fn must_increment(){
	let mut interpreter = Interpreter::init();
	interpreter.nextCommand('+');
	if interpreter.debugTapeValue() != 1 {
			fail!("the interpreter failed to increment");
	}
}
#[test]
fn must_decrement(){
	let mut interpreter = Interpreter::init();
	interpreter.nextCommand('-');
	if interpreter.debugTapeValue() != -1 {
			fail!("the interpreter failed to decrement");
	}
}

#[test]
fn program_cannot_go_left_past_beginning_of_the_tape(){
	let mut interpreter = Interpreter::init();
	interpreter.nextCommand('<');
	if interpreter.debugGetTapeIndex() != 0 {
		fail!("the interpreter can go past the beginning of the tape");
	}
}
/*
7.  Addressing:  Calculation is not extraordinarily useful if only one
value can be manipulated.  Therefore, BrainF*** permits navigation
through the data memory as if it were a Turing-Machine-like "infinite
tape."  Starting from the current tape position, the program can be
made to use the next cell to the right (the keyword ">") or to the left
("<").  [NB:  Due to obvious restrictions of "real world" computers,
BrainF*** implementations are permitted to use only a "semi-infinite
tape," on which movement can always progress to the right, but may not
go further left than the program's original starting position.]
*/
#[test]
fn program_start_on_the_left_of_the_tape(){
	let interpreter = Interpreter::init();
	if interpreter.debugGetTapeIndex() != 0 {
		fail!("the interpreter didn't start on the leftmost part of the tape");
	}
}

/*
8.  Control Structures:  BrainF*** has a single, simple but flexible,
control structure--a pretest loop with a predefined condition.  The
loop condition, before executing the loop body, treats the current
value (on the tape) as a Boolean (see Data Types), and executes the
loop body only if the resulting boolean is TRUE.  Loops are defined by
the keyword pair "["..."]."  Note that, due to the "shallow" nature of
the arithmetic operations, flow control may be used to enhance these
operations and perform more complicated mathematical tasks.
*/
#[test]
fn loop_only_if_true(){
// already tested.
}

#[test]
fn if_true_must_follows_jumps(){
// already tested.
}

/*

9.  Communications:  BrainF*** interacts with the outside world in a
character-oriented fashion.  Specifically, a single character can be
retrieved from, or sent to, the outside world.  Retrieval of a
character from the standard input device (designated by the ","
keyword) converts the character to an integer, and stores it as the
current value on the tape.  Outputting a character (through the "."
keyword) interprets the current value as a character, and sends it to
the standard output device.
*/
#[test]
fn must_read_from_stdin(){
// don't know how to test from here
}
#[test]
fn must_write_to_stout(){
// don't know how to test from here
}
/*

9a. Debugging:  In some BrainF*** implementations, the local portion of
the tape may be processed as output using the special keyword "#."
Such enhancements are beyond the scope of this document, however.

10. Summary:  BrainF*** is a small language with a high degree of
orthogonality and conceptual economy.  The command set should be
learnable in only a few minutes.  Specifically, the distilled command
set is:
        Keyword         Action
           +            Increment current tape value
           -            Decrement current tape value
           >            Process next-right tape value
           <            Process next-left tape value
           [            Pretest loop begins
           ]            Pretest loop terminates
           ,            Retrieve input character (as integer)
           .            Send output value (as character)
           #            Output partial tape state (not required)

--------

See why nobody ever bothered...?
*/
