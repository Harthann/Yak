use crate::pic::pit::play_sound;
use crate::sound::sleep;

pub enum NoteTempo {
	Whole,
	Half,
	Quarter,
	Eighth,
	Sixteenth,
	Thirty2nd
}

pub enum NoteType {
	Base,
	Dotted,
	Triplet
}

pub struct Note {
	frequency: f32,   // Hertz
	duration:  usize  // millisecond
}

impl Note {
	pub fn new(frequency: f32, duration: usize) -> Self {
		Note { frequency, duration }
	}

	pub fn play(&self) {
		play_sound(self.frequency);
		sleep(self.duration);
		play_sound(crate::sound::Rest);
		sleep(15);
	}
}
