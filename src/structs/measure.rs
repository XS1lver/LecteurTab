use crate::beat::Beat;
use crate::signature::Signature;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Measure{
    beats: Vec<Beat>,
    sig: Signature
}

impl Measure{
    pub fn new(beats: Vec<Beat>, sig: Signature) -> Self{
        Self{beats, sig}
    }

    pub fn get_beats(&self) -> &[Beat]{
        return &self.beats;
    }
    pub fn get_sig(&self) -> &Signature{
        return &self.sig;
    }

    pub fn sum_sigs(&self) -> Signature{
        let mut res = Signature::new(0, 1);
        for beat in &self.beats{
            res.add(beat.get_time());
        }
        return res;
    }
}
