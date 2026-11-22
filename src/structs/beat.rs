use crate::note::Note;
use crate::signature::Signature;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Beat{
    pub notes: Vec<Note>,
    pub duration: Signature,
    //pub r#type: i16,
    //pub rest: Option<bool>
}

/*impl Beat{
    pub fn new(notes: Vec<Note>, r#type: i16, duration: Signature, rest: Option<bool>) -> Self{
        Self{notes, r#type, duration, rest}
    }
}*/
