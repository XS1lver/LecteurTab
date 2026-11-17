use crate::signature::Signature;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Note{
    string: u8,
    fret: u8,
}

impl Note{
    pub fn new(string: u8, fret: u8) -> Self{
        Self{string, fret}
    }

    pub fn get_string(&self) -> u8{
        return self.string;
    }

    pub fn get_fret(&self) -> u8{
        return self.fret;
    }
}
