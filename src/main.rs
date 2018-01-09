mod mode;

use self::mode::Mode as Mode;
use self::mode::interval::Interval as Interval;
use self::mode::note::Note as Note;

fn main() {
	print_mode("E", mode::IONIAN);
}

fn print_mode(root: &str, mode: Mode) {
	println!("For root note {}", root);
	for note in mode::IONIAN.build_scale(root) {
		match note {
			Some(x) => println!("{}", x),
			_ => println!("")
		};
	}
}
