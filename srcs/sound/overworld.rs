use crate::sound::Partition;
use crate::sound::BeatType;
use crate::sound::note::{NoteTempo, NoteType};
use crate::sound::notes_frequencies::{*};

impl Partition {
    pub fn overworld() -> Self {
        let mut partition = Partition::new(100, BeatType::QUARTER);
        partition.add_note(E5, NoteTempo::THIRTY2ND, NoteType::BASE)
                 .add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(G5, NoteTempo::QUARTER, NoteType::BASE);
        partition.add_note(G4, NoteTempo::QUARTER, NoteType::BASE);
        
        for i in 0..2 {
            partition.add_note(C5, NoteTempo::EIGTH, NoteType::DOTTED);
            partition.add_note(G4, NoteTempo::EIGTH, NoteType::DOTTED);
            partition.add_note(E4, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(B4, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(Bb4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::EIGTH, NoteType::BASE);
        
            partition.add_triplet((G4,E5,G5), NoteTempo::EIGTH);
            partition.add_note(A5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(G5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(D5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(B4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::EIGTH, NoteType::BASE);
        }
        
        // B
        for i in 0..2 {
            partition.add_note(Rest, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(G5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Gb5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Ds5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(G4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(D5, NoteTempo::SIXTEENTH, NoteType::BASE);
        
            partition.add_note(Rest, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(G5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Gb5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Ds5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(C6, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(C6, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(C6, NoteTempo::QUARTER, NoteType::BASE);
        
            partition.add_note(Rest, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(G5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Gb5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Ds5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(G4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(D5, NoteTempo::SIXTEENTH, NoteType::BASE);
        
            partition.add_note(Rest, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(Eb5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(D5, NoteTempo::EIGTH, NoteType::DOTTED);
            partition.add_note(C5, NoteTempo::QUARTER, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::QUARTER, NoteType::BASE);
        }
        
        // C
        partition.add_note(C5, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(D5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(G4, NoteTempo::QUARTER, NoteType::BASE);
        
        partition.add_note(C5, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(D5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::HALF, NoteType::BASE);
        
        partition.add_note(C5, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(D5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(G4, NoteTempo::QUARTER, NoteType::BASE);
        
        partition.add_note(E5, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(G5, NoteTempo::QUARTER, NoteType::BASE);
        partition.add_note(G4, NoteTempo::QUARTER, NoteType::BASE);
        
        // A'
        for i in 0..2 {
            partition.add_note(C5, NoteTempo::EIGTH, NoteType::DOTTED);
            partition.add_note(G4, NoteTempo::EIGTH, NoteType::DOTTED);
            partition.add_note(E4, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(B4, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(Bb4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::EIGTH, NoteType::BASE);
        
            partition.add_triplet((G4,E5,G5), NoteTempo::EIGTH);
            partition.add_note(A5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(G5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(D5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(B4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::EIGTH, NoteType::BASE);
        }
        
        // D
        for i in 0..2 {
            partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(G4, NoteTempo::EIGTH, NoteType::DOTTED);
            partition.add_note(Gs4, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::QUARTER, NoteType::BASE);
        
            partition.add_irregular_note(B4, NoteTempo::QUARTER, 3);
            partition.add_irregular_note(A5, NoteTempo::QUARTER, 6);
            partition.add_irregular_note(Rest, NoteTempo::QUARTER, 6);
            partition.add_irregular_note(A5, NoteTempo::QUARTER, 6);
            partition.add_irregular_note(Rest, NoteTempo::QUARTER, 6);
            partition.add_triplet((A5,G5,F5), NoteTempo::EIGTH);
            partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(G4, NoteTempo::QUARTER, NoteType::BASE);
        
            partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(G4, NoteTempo::EIGTH, NoteType::DOTTED);
            partition.add_note(Gs4, NoteTempo::EIGTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(A4, NoteTempo::QUARTER, NoteType::BASE);
        
            partition.add_note(B4, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
            partition.add_note(F5, NoteTempo::THIRTY2ND, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
            partition.add_triplet((F5,E5,D5), NoteTempo::EIGTH);
            partition.add_note(C5, NoteTempo::QUARTER, NoteType::BASE);
            partition.add_note(Rest, NoteTempo::QUARTER, NoteType::BASE);
        }
        
        // E
        partition.add_note(C5, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(D5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(G4, NoteTempo::QUARTER, NoteType::BASE);
        
        partition.add_note(C5, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(D5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::HALF, NoteType::BASE);
        
        partition.add_note(C5, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(D5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(G4, NoteTempo::QUARTER, NoteType::BASE);
        
        partition.add_note(E5, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(C5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(E5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(G5, NoteTempo::QUARTER, NoteType::BASE);
        partition.add_note(G4, NoteTempo::QUARTER, NoteType::BASE);
        
        partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(G4, NoteTempo::EIGTH, NoteType::DOTTED);
        partition.add_note(Gs4, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(A4, NoteTempo::QUARTER, NoteType::BASE);
        
        partition.add_irregular_note(B4, NoteTempo::QUARTER, 3);
        partition.add_irregular_note(A5, NoteTempo::QUARTER, 6);
        partition.add_irregular_note(Rest, NoteTempo::QUARTER, 6);
        partition.add_irregular_note(A5, NoteTempo::QUARTER, 6);
        partition.add_irregular_note(Rest, NoteTempo::QUARTER, 6);
        partition.add_triplet((A5,G5,F5), NoteTempo::EIGTH);
        partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(G4, NoteTempo::QUARTER, NoteType::BASE);
        
        partition.add_note(E5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(C5, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(G4, NoteTempo::EIGTH, NoteType::DOTTED);
        partition.add_note(Gs4, NoteTempo::EIGTH, NoteType::BASE);
        partition.add_note(A4, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(A4, NoteTempo::QUARTER, NoteType::BASE);
        
        partition.add_note(B4, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(F5, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::SIXTEENTH, NoteType::BASE);
        partition.add_note(F5, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::THIRTY2ND, NoteType::BASE);
        partition.add_triplet((F5,E5,D5), NoteTempo::EIGTH);
        partition.add_note(C5, NoteTempo::QUARTER, NoteType::BASE);
        partition.add_note(Rest, NoteTempo::QUARTER, NoteType::BASE);

        partition
    }
}
