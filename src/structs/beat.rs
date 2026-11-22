use crate::note::Note;
use crate::signature::Signature;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Beat{
    notes: Vec<Note>,
    r#type: i16,
    duration: Signature,
    rest: Option<bool>
}

impl Beat{
    pub fn new(notes: Vec<Note>, r#type: i16, duration: Signature, rest: Option<bool>) -> Self{
        Self{notes, r#type, duration, rest}
    }

    pub fn get_notes(&self) -> &[Note]{
        &self.notes
    }
    pub fn get_duration(&self) -> &Signature{
        &self.duration
    }
}
