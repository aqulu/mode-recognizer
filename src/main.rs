mod mode;

use std::env;
use self::mode::note::Note as Note;

fn main() {

    let args: Vec<_> = env::args().collect();
	let inputs: Vec<_> = args.iter().skip(1).collect();

	mode::note::NOTE_SCALE.iter()
		.map(|note| {
			// build all mode scales for every note
			mode::MODES.iter()
				.map(|mode|
					(mode.name.to_string() + " " + note.name, mode.build_scale(note.name))
				)
				.collect()
		})
		.for_each(|scales: Vec<(String, Vec<Option<Note>>)>| {
			scales.into_iter()
				.filter(|scale| {
					scale.1.iter().fold(true, |carry, note| match note {
						&Some(x) => carry && inputs.iter().find(| input | x.equals(input)).is_some(),
						_ => carry
					})
				})
				.for_each(print_scale);
		});
}

fn print_scale (scale: (String, Vec<Option<Note>>)) {
	println!("{}", scale.0);
	let notes = scale.1.iter().fold(String::from(""), |carry, note: &Option<Note>| match note {
		&Some(x) => format!("{0} {1}", carry, x),
		_ => carry
	});
	println!("{}", notes);
	println!("\n")
}
