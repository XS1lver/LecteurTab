use crate::note::Note;
use crate::signature::Signature;

pub struct Beat{
    notes: Vec<Note>,
    time: Signature
}

impl Beat{
    pub fn new(notes: Vec<Note>, time: Signature) -> Self{
        Self{notes, time}
    }

    pub fn get_notes(&self) -> &[Note]{
        &self.notes
    }
    pub fn get_time(&self) -> &Signature{
        &self.time
    }
}
