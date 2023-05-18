use crate::pic::pit::{speaker_off, speaker_on};
use crate::time::sleep;
use crate::vec::Vec;

mod notes_frequencies;
use notes_frequencies::*;
mod note;
use note::{Note, NoteTempo, NoteType};

mod mii;
mod overworld;

#[repr(u32)]
enum BeatType {
	WHOLE     = 1,
	HALF      = 2,
	QUARTER   = 4,
	EIGTH     = 8,
	SIXTEENTH = 16,
	THIRTY2ND = 32
}

struct Partition {
	bpm:                     usize,
	whole_note_duration:     usize,
	half_note_duration:      usize,
	quarter_note_duration:   usize,
	eigth_note_duration:     usize,
	sixteenth_note_duration: usize,
	thirty2nd_note_duration: usize,
	notes:                   Vec<Note>
}

impl Partition {
	pub fn new(bpm: usize, beat_type: BeatType) -> Self {
		let whole_note = (60000 / bpm) * beat_type as usize;
		Partition {
			bpm:                     bpm,
			whole_note_duration:     whole_note,
			half_note_duration:      whole_note / 2,
			quarter_note_duration:   whole_note / 4,
			eigth_note_duration:     whole_note / 8,
			sixteenth_note_duration: whole_note / 16,
			thirty2nd_note_duration: whole_note / 32,
			notes:                   Vec::new()
		}
	}

	pub fn add_note(
		&mut self,
		frequency: f32,
		note_tempo: NoteTempo,
		note_type: NoteType
	) -> &mut Self {
		let duration: usize = match note_tempo {
			NoteTempo::WHOLE => self.whole_note_duration,
			NoteTempo::HALF => self.half_note_duration,
			NoteTempo::QUARTER => self.quarter_note_duration,
			NoteTempo::EIGTH => self.eigth_note_duration,
			NoteTempo::SIXTEENTH => self.sixteenth_note_duration,
			NoteTempo::THIRTY2ND => self.thirty2nd_note_duration
		};
		match note_type {
			NoteType::BASE => self.notes.push(Note::new(frequency, duration)),
			NoteType::DOTTED => {
				self.notes.push(Note::new(frequency, duration * 3 / 2))
			},
			NoteType::TRIPLET => {
				self.notes.push(Note::new(frequency, duration / 3));
				self.notes.push(Note::new(frequency, duration / 3));
				self.notes.push(Note::new(frequency, duration / 3));
			}
		}
		self
	}

	pub fn add_triplet(
		&mut self,
		frequencies: (f32, f32, f32),
		note_tempo: NoteTempo
	) {
		let duration: usize = match note_tempo {
			NoteTempo::WHOLE => self.whole_note_duration,
			NoteTempo::HALF => self.half_note_duration,
			NoteTempo::QUARTER => self.quarter_note_duration,
			NoteTempo::EIGTH => self.eigth_note_duration,
			NoteTempo::SIXTEENTH => self.sixteenth_note_duration,
			NoteTempo::THIRTY2ND => self.thirty2nd_note_duration
		};
		self.notes.push(Note::new(frequencies.0, duration / 3));
		self.notes.push(Note::new(frequencies.1, duration / 3));
		self.notes.push(Note::new(frequencies.2, duration / 3));
	}

	pub fn add_irregular_note(
		&mut self,
		frequency: f32,
		note_tempo: NoteTempo,
		div: usize
	) {
		let duration: usize = match note_tempo {
			NoteTempo::WHOLE => self.whole_note_duration,
			NoteTempo::HALF => self.half_note_duration,
			NoteTempo::QUARTER => self.quarter_note_duration,
			NoteTempo::EIGTH => self.eigth_note_duration,
			NoteTempo::SIXTEENTH => self.sixteenth_note_duration,
			NoteTempo::THIRTY2ND => self.thirty2nd_note_duration
		};
		self.notes.push(Note::new(frequency, duration / div));
	}

	pub fn play(&self) {
		speaker_on();
		for i in self.notes.iter() {
			i.play();
		}
		speaker_off();
	}
}

pub fn play(sound: &str) {
	let known_sound = ["overworld"];

	match sound {
		"overworld" => Partition::overworld().play(),
		_ => {
			crate::kprintln!("Usage:");
			crate::kprintln!("    play [sound name]");
			crate::kprintln!("Sound available:");
			for i in known_sound {
				crate::kprintln!("    - {}", i);
			}
		}
	}
}
