/* Another Rust Shell
 * Trevor Calderwood
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
		let stream = io::stdin();
	}
	else {
		let stream = File::open(margv[1]);
	}

	loop {
		// get line
		let mut line = String::new();
		stream.read_line(&line);
		let mut line_len = line.len();
		
		// remove trailing newline char
		_ = line.pop();
		
		// remove comments
		let ret = remove_comments(&line, line_len);
		if ret == 0 {
			// execute line
			process_line(buf, 0, 1, WAIT|EXPAND);
		}
		else {
			// handle error
		}
	}
}

// removes any comment starting with #. Ignores $#
fn remove_comments(line: &mut String, line_len: i32) {
	if line[0] == '#' {
		// line is empty, nothing to execute
		-1
	}
	for i in (1..line_len - 1) {
		if (line[i] == '#') AND (line[i-1] != '$') {
			line.truncate(i);
			break;
		}
	}
	0
}

fn process_line(line: &String, in_fd: i32, out_fd: i32, flags: i32) -> i32 {
	0
}

fn arg_parse(line: &String, argc: i32) {

}

fn print_args(argv: Args, argc: i32) {
	
}

fn squish() -> i32 {
	0
}

fn handle_sigint() {

}
