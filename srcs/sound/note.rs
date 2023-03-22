use crate::sound::sleep;
use crate::pic::pit::play_sound;

pub enum NoteTempo {
    WHOLE,
    HALF,
    QUARTER,
    EIGTH,
    SIXTEENTH,
    THIRTY2ND
}

pub enum NoteType {
    BASE,
    DOTTED,
    TRIPLET
}

pub struct Note {
    frequency: f32, // Hertz
    duration: usize   // millisecond
}

impl Note {
    pub fn new(frequency: f32, duration: usize) -> Self {
        Note {
            frequency: frequency,
            duration:  duration
        }
    }

    pub fn play(&self) {
        play_sound(self.frequency);
        sleep(self.duration);
        play_sound(crate::sound::Rest);
        sleep(15);
    }
}


