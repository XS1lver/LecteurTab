use crate::note::Note;
use crate::signature::Signature;

pub struct Measure{
    id: u8,
    notes: Vec<Note>,
    sig: Signature
}

impl Measure{
    pub fn new(id: u8, notes: Vec<Note>, sig: Signature) -> Self{
        Self{id, notes, sig}
    }

    pub fn getId(&self) -> u8{
        return self.id;
    }
    pub fn getNotes(&self) -> &[Note]{
        return &self.notes;
    }
    pub fn getSig(&self) -> &Signature{
        return &self.sig;
    }

    pub fn sum_sigs(&self) -> Signature{
        let mut res = Signature::new(0, 1);
        for note in &self.notes{
            res.add(note.getTimeSig());
        }
        return res;
    }
}
