use crate::note::Note;
use crate::signature::Signature;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Beat{
    pub notes: Vec<Note>,
    pub duration: Signature,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    dots: Option<i8>,
    //pub r#type: i16,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rest: Option<bool>
}

impl Beat{
    /*pub fn new(notes: Vec<Note>, r#type: i16, duration: Signature, rest: Option<bool>) -> Self{
        Self{notes, r#type, duration, rest}
    }*/

    pub fn get_dots(&self) -> i8{
        if self.dots != None{
            return self.dots.unwrap();
        }
        else{
            return -1;
        }
    }

    pub fn get_rest(&self) -> bool{
        if self.rest != None{
            return true;
        }
        else{
            return false;
        }
    }
}
