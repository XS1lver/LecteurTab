use crate::voice::Voice;
//use crate::signature::Signature;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Measure{
    pub voices: Vec<Voice>,
    //pub signature: Signature
}

/*impl Measure{
    pub fn new(voices: Vec<Voice>, signature: Signature) -> Self{
        Self{voices, signature}
    }
}*/
