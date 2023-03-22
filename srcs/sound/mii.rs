use crate::sound::Partition;
use crate::sound::BeatType;
use crate::sound::note::{NoteTempo, NoteType};
use crate::sound::notes_frequencies::{*};

impl Partition {
    pub fn mii() -> Self {
        let mut partition = Partition::new(114, BeatType::QUARTER);

        partition.add_note(Fs4,  NoteTempo::QUARTER,   NoteType::BASE);
        partition.add_note(A4,   NoteTempo::EIGTH,     NoteType::BASE);
        partition.add_note(Cs5,  NoteTempo::EIGTH,     NoteType::BASE);

        partition.add_note(Rest, NoteTempo::EIGTH,     NoteType::BASE);
        partition.add_note(A4,   NoteTempo::EIGTH,     NoteType::BASE);

        partition.add_note(Rest, NoteTempo::EIGTH,     NoteType::BASE);
        partition.add_note(D4,   NoteTempo::EIGTH,     NoteType::BASE);

        partition.add_note(D4,   NoteTempo::EIGTH,     NoteType::BASE);
        partition.add_note(D4,   NoteTempo::EIGTH,     NoteType::BASE);

        return partition;
    }
}
