pub mod interval;
pub mod note;

use self::interval::Interval as Interval;
use self::note::Note as Note;

pub struct Mode<'a> {
	pub name: &'a str,
    pub intervals: [Interval; 7],
}

impl<'a> Mode<'a> {
    pub fn build_scale(&self, root: &str) -> Vec<Option<Note>> {
		note::NOTE_SCALE.iter()
			.position(|note| note.equals(root))
			.map_or(Vec::new(), |position| {
				let mut scale: Vec<Option<Note>> = Vec::new();
				let mut note_iterator = note::NOTE_SCALE.into_iter().cycle().skip(position);

				scale.push(note_iterator.next().cloned());
				for interval in &self.intervals {
					let note = match interval {
						&Interval::WHOLE => {
							note_iterator.next();
							note_iterator.next()
						},
						_ => note_iterator.next(),
					};
					scale.push(note.cloned());
				}

				scale
			})
    }
}

pub const MODES: [Mode; 7] = [
	Mode {
		name: "Ionian",
		intervals: [
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::HALF,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::HALF,
		],
	},
	Mode {
		name: "Dorian",
		intervals: [
			Interval::WHOLE,
			Interval::HALF,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::HALF,
			Interval::WHOLE,
		],
	},
	Mode {
		name: "Phrygian",
		intervals: [
			Interval::HALF,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::HALF,
			Interval::WHOLE,
			Interval::WHOLE,
		],
	},
	Mode {
		name: "Lydian",
		intervals: [
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::HALF,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::HALF,
		],
	},
	Mode {
		name: "Mixolydian",
		intervals: [
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::HALF,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::HALF,
			Interval::WHOLE,
		],
	},
	Mode {
		name: "Aeolian",
		intervals: [
			Interval::WHOLE,
			Interval::HALF,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::HALF,
			Interval::WHOLE,
			Interval::WHOLE,
		],
	},
	Mode {
		name: "Locrian",
		intervals: [
			Interval::HALF,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::HALF,
			Interval::WHOLE,
			Interval::WHOLE,
			Interval::WHOLE,
		],
	},
];
