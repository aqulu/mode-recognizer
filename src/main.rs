mod mode;

use self::mode::note::Note as Note;

fn main() {

	// TODO take input param, filter built scales, print only matchting
	mode::note::NOTE_SCALE.iter()
		.map( |note| {
			// build all mode scales for every note
			mode::MODES.iter()
				.map(|mode| mode.build_scale(note.name))
				.collect()
		})
		.for_each(|scales: Vec<Vec<Option<Note>>>| {
			scales.iter()
				.for_each(|scale: &Vec<Option<Note>>| {
					scale.iter()
						.for_each(|note: &Option<Note>| match note {
							&Some(x) => println!("{}", x),
							_ => println!("")
						});
					println!("\n")
				});
		});
}
