use crate::sound::note::{NoteTempo, NoteType};
use crate::sound::notes_frequencies::*;
use crate::sound::{BeatType, Partition};

impl Partition {
	pub fn mii() -> Self {
		let mut partition = Partition::new(114, BeatType::Quarter);

		partition.add_note(Fs4, NoteTempo::Quarter, NoteType::Base);
		partition.add_note(A4, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Cs5, NoteTempo::Eighth, NoteType::Base);

		partition.add_note(Rest, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(A4, NoteTempo::Eighth, NoteType::Base);

		partition.add_note(Rest, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(D4, NoteTempo::Eighth, NoteType::Base);

		partition.add_note(D4, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(D4, NoteTempo::Eighth, NoteType::Base);

		partition
	}
}
