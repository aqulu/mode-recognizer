pub mod interval;
pub mod note;

use self::interval::Interval as Interval;
use self::note::Note as Note;

pub struct Mode {
    pub intervals: [Interval; 7],
}

impl Mode {
    pub fn build_scale(&self, root: &str) -> Vec<Option<Note>> {
		note::NOTE_SCALE.iter()
			.position(|note| note.equals(root))
			.map_or(Vec::new(), |position| {
				let mut scale: Vec<Option<Note>> = Vec::new();
				let mut note_iterator = note::NOTE_SCALE.into_iter().cycle().skip(position);

				// TODO check if lifetime is ok (works, but not very confident)
				fn deep_clone<'a>(to_clone: Option<&'a Note>) -> Option<Note<'a>> {
					to_clone.and_then( |note| Some(note.clone()) )
				};

				scale.push(deep_clone(note_iterator.next()));
				for interval in &self.intervals {
					let note = match interval {
						&Interval::WHOLE => {
							note_iterator.next();
							note_iterator.next()
						},
						_ => note_iterator.next(),
					};
					scale.push(deep_clone(note));
				}

				scale
			})
    }
}

pub const IONIAN: Mode = Mode {
	intervals: [
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::HALF,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::HALF,
	]
};

pub const DORIAN: Mode = Mode {
	intervals: [
		Interval::WHOLE,
		Interval::HALF,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::HALF,
		Interval::WHOLE,
	]
};

pub const PHRYGIAN: Mode = Mode {
	intervals: [
		Interval::HALF,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::HALF,
		Interval::WHOLE,
		Interval::WHOLE,
	]
};

pub const LYDIAN: Mode = Mode {
	intervals: [
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::HALF,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::HALF,
	]
};

pub const MIXOLYDIAN: Mode = Mode {
	intervals: [
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::HALF,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::HALF,
		Interval::WHOLE,
	]
};

pub const AEOLIAN: Mode = Mode {
	intervals: [
		Interval::WHOLE,
		Interval::HALF,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::HALF,
		Interval::WHOLE,
		Interval::WHOLE,
	]
};

pub const LOCRIAN: Mode = Mode {
	intervals: [
		Interval::HALF,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::HALF,
		Interval::WHOLE,
		Interval::WHOLE,
		Interval::WHOLE,
	]
};
