use crate::note::Note;
use crate::beat::Beat;
use crate::signature::Signature;

pub struct Measure{
    id: u8,
    beats: Vec<Beat>,
    sig: Signature
}

impl Measure{
    pub fn new(id: u8, beats: Vec<Beat>, sig: Signature) -> Self{
        Self{id, beats, sig}
    }

    pub fn get_id(&self) -> u8{
        return self.id;
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
