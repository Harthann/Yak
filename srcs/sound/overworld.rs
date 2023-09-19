use crate::sound::note::{NoteTempo, NoteType};
use crate::sound::notes_frequencies::*;
use crate::sound::{BeatType, Partition};

impl Partition {
	pub fn overworld() -> Self {
		let mut partition = Partition::new(100, BeatType::Quarter);
		partition
			.add_note(E5, NoteTempo::Thirty2nd, NoteType::Base)
			.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(G5, NoteTempo::Quarter, NoteType::Base);
		partition.add_note(G4, NoteTempo::Quarter, NoteType::Base);

		for _ in 0..2 {
			partition.add_note(C5, NoteTempo::Eighth, NoteType::Dotted);
			partition.add_note(G4, NoteTempo::Eighth, NoteType::Dotted);
			partition.add_note(E4, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(B4, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(Bb4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Eighth, NoteType::Base);

			partition.add_triplet((G4, E5, G5), NoteTempo::Eighth);
			partition.add_note(A5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(G5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(D5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(B4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Eighth, NoteType::Base);
		}

		// B
		for _ in 0..2 {
			partition.add_note(Rest, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(G5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Gb5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Ds5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(G4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(D5, NoteTempo::Sixteenth, NoteType::Base);

			partition.add_note(Rest, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(G5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Gb5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Ds5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(C6, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(C6, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(C6, NoteTempo::Quarter, NoteType::Base);

			partition.add_note(Rest, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(G5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Gb5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Ds5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(G4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(D5, NoteTempo::Sixteenth, NoteType::Base);

			partition.add_note(Rest, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(Eb5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(D5, NoteTempo::Eighth, NoteType::Dotted);
			partition.add_note(C5, NoteTempo::Quarter, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Quarter, NoteType::Base);
		}

		// C
		partition.add_note(C5, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(D5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(G4, NoteTempo::Quarter, NoteType::Base);

		partition.add_note(C5, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(D5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Half, NoteType::Base);

		partition.add_note(C5, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(D5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(G4, NoteTempo::Quarter, NoteType::Base);

		partition.add_note(E5, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(G5, NoteTempo::Quarter, NoteType::Base);
		partition.add_note(G4, NoteTempo::Quarter, NoteType::Base);

		// A'
		for _ in 0..2 {
			partition.add_note(C5, NoteTempo::Eighth, NoteType::Dotted);
			partition.add_note(G4, NoteTempo::Eighth, NoteType::Dotted);
			partition.add_note(E4, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(B4, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(Bb4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Eighth, NoteType::Base);

			partition.add_triplet((G4, E5, G5), NoteTempo::Eighth);
			partition.add_note(A5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(G5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(D5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(B4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Eighth, NoteType::Base);
		}

		// D
		for _ in 0..2 {
			partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(G4, NoteTempo::Eighth, NoteType::Dotted);
			partition.add_note(Gs4, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Quarter, NoteType::Base);

			partition.add_irregular_note(B4, NoteTempo::Quarter, 3);
			partition.add_irregular_note(A5, NoteTempo::Quarter, 6);
			partition.add_irregular_note(Rest, NoteTempo::Quarter, 6);
			partition.add_irregular_note(A5, NoteTempo::Quarter, 6);
			partition.add_irregular_note(Rest, NoteTempo::Quarter, 6);
			partition.add_triplet((A5, G5, F5), NoteTempo::Eighth);
			partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(G4, NoteTempo::Quarter, NoteType::Base);

			partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(G4, NoteTempo::Eighth, NoteType::Dotted);
			partition.add_note(Gs4, NoteTempo::Eighth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(A4, NoteTempo::Quarter, NoteType::Base);

			partition.add_note(B4, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
			partition.add_note(F5, NoteTempo::Thirty2nd, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
			partition.add_triplet((F5, E5, D5), NoteTempo::Eighth);
			partition.add_note(C5, NoteTempo::Quarter, NoteType::Base);
			partition.add_note(Rest, NoteTempo::Quarter, NoteType::Base);
		}

		// E
		partition.add_note(C5, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(D5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(G4, NoteTempo::Quarter, NoteType::Base);

		partition.add_note(C5, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(D5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Half, NoteType::Base);

		partition.add_note(C5, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(D5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(G4, NoteTempo::Quarter, NoteType::Base);

		partition.add_note(E5, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(C5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(E5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(G5, NoteTempo::Quarter, NoteType::Base);
		partition.add_note(G4, NoteTempo::Quarter, NoteType::Base);

		partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(G4, NoteTempo::Eighth, NoteType::Dotted);
		partition.add_note(Gs4, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(A4, NoteTempo::Quarter, NoteType::Base);

		partition.add_irregular_note(B4, NoteTempo::Quarter, 3);
		partition.add_irregular_note(A5, NoteTempo::Quarter, 6);
		partition.add_irregular_note(Rest, NoteTempo::Quarter, 6);
		partition.add_irregular_note(A5, NoteTempo::Quarter, 6);
		partition.add_irregular_note(Rest, NoteTempo::Quarter, 6);
		partition.add_triplet((A5, G5, F5), NoteTempo::Eighth);
		partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(G4, NoteTempo::Quarter, NoteType::Base);

		partition.add_note(E5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(C5, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(G4, NoteTempo::Eighth, NoteType::Dotted);
		partition.add_note(Gs4, NoteTempo::Eighth, NoteType::Base);
		partition.add_note(A4, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(A4, NoteTempo::Quarter, NoteType::Base);

		partition.add_note(B4, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(F5, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Sixteenth, NoteType::Base);
		partition.add_note(F5, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Thirty2nd, NoteType::Base);
		partition.add_triplet((F5, E5, D5), NoteTempo::Eighth);
		partition.add_note(C5, NoteTempo::Quarter, NoteType::Base);
		partition.add_note(Rest, NoteTempo::Quarter, NoteType::Base);

		partition
	}
}
