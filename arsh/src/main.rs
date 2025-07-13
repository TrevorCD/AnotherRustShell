/* Pashmina shell
 * Trevor Calderwood
 * Last Modified 7/5/25
 */
use std::fs::File;
use std::io::BufRead;
use std::io::stdin;
use std::env::args;
use std::env::Args;

/* Constants */
const LINE_LEN: i32 = 200_000;
const WAIT = 1;
const EXPAND = 2;
const NOFLAGS = 0;

fn main() {

	let margv = args();
	let margc = args().len();

	if margc == 1 {
		let mut streamp = stdin;
	}
	else {
		let mut streamp = File::open(margv[1]);
	}

	loop {

		// get line

		// remove trailing newline

		// remove comments

		// execute line
		process_line(buf, 0, 1, WAIT|EXPAND);
	}
}

fn process_line(line:String, in_fd::i32, out_fd::i32, flags::i32) -> i32 {
	0
}

fn arg_parse(line:String, &mut argc:i32) {

}

fn print_args(argv:Args, argc:i32) {

}

fn squish() -> i32 {
	0
}

fn handle_sigint() {

}
