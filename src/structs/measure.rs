use crate::voice::Voice;
//use crate::signature::Signature;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Measure{
    pub voices: Vec<Voice>,
    //pub signature: Signature
}

impl Measure{
    /*pub fn new(voices: Vec<Voice>, signature: Signature) -> Self{
        Self{voices, signature}
    }*/

    pub fn display_string(&self, string: i8){
        for voice in &self.voices{
            for beat in &voice.beats{
                let mut string_played = false;
                for note in &beat.notes{
                    if note.get_string() == string{
                        if note.get_fret() >= 10{
                            print!("{} ", note.get_fret());
                        }
                        else{
                            print!("{}  ", note.get_fret());
                        }   
                        string_played = true;
                    }
                }
                if !string_played{
                    print!("   ");
                }           
            }
        }
        print!("| "); // Separation between measures
    }
}
