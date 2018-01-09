use std::fmt::{ Display, Formatter, Error };

pub const NOTE_SCALE: [Note; 12] = [
    Note { name: "C", alt: None },
    Note { name: "C#", alt: Some("Db") },
	Note { name: "D", alt: None },
	Note { name: "D#", alt: Some("Eb") },
	Note { name: "E", alt: None },
	Note { name: "F", alt: None },
	Note { name: "F#", alt: Some("Gb") },
	Note { name: "G", alt: None },
	Note { name: "G#", alt: Some("Ab") },
	Note { name: "A", alt: None },
	Note { name: "A#", alt: Some("Bb") },
	Note { name: "B", alt: None },
];


#[derive(Debug, Clone, Copy)]
pub struct Note<'a> {
    pub name: &'a str,
    pub alt: Option<&'a str>,
}

impl<'a> Note<'a> {
	pub fn equals(&self, note: &str) -> bool {
		note == self.name || self.alt.map_or(false, |alt| note == alt)
	}
}

impl<'a> Display for Note<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		let alt = &self.alt.map_or(String::from(""), |value| String::from("/") + value );
        write!(f, "{0}{1}", &self.name, alt)
    }
}
