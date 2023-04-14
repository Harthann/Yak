#[derive(Clone,Debug)]
pub enum Input {
    Ascii(char),
    Tcaps(Termcaps),
    Unknwown(char)
}

#[derive(Clone,Debug)]
pub enum Termcaps {
    ArrowUP,
    ArrowDOWN,
    ArrowLEFT,
    ArrowRIGHT
}
