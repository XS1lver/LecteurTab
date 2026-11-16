use crate::signature::Signature;

pub struct Note{
    string: u8,
    fret: u8,
    time: Signature
}

impl Note{
    pub fn new(string: u8, fret: u8, time: Signature) -> Self{
        Self{string, fret, time}
    }

    pub fn getString(&self) -> u8{
        return self.string;
    }

    pub fn getFret(&self) -> u8{
        return self.fret;
    }

    pub fn getTimeSig(&self) -> &Signature{
        return &self.time;
    }
}
