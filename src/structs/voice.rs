use serde::Deserialize;
use crate::beat::Beat;

#[derive(Debug, Deserialize)]
pub struct Voice{
    beats: Vec<Beat>
}

impl Voice{
    pub fn new(beats: Vec<Beat>) -> Self{
        Self{beats}
    }

    pub fn get_beats(&self) -> &[Beat]{
        &self.beats
    }
}
