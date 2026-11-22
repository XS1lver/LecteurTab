use crate::voice::Voice;
use crate::signature::Signature;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Measure{
    voices: Vec<Voice>,
    signature: Signature
}

impl Measure{
    pub fn new(voices: Vec<Voice>, signature: Signature) -> Self{
        Self{voices, signature}
    }

    pub fn get_voices(&self) -> &[Voice]{
        return &self.voices;
    }
    pub fn get_sig(&self) -> &Signature{
        return &self.signature;
    }
}
