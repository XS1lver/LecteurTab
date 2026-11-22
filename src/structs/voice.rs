use serde::Deserialize;
use crate::beat::Beat;

#[derive(Debug, Deserialize)]
pub struct Voice{
    pub beats: Vec<Beat>
}

/*impl Voice{
    pub fn new(beats: Vec<Beat>) -> Self{
        Self{beats}
    }
}*/
