use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Note{
    #[serde(default, skip_serializing_if = "Option::is_none")]
    string: Option<i8>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    fret: Option<i8>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    rest: Option<bool>
}

impl Note{
    pub fn new(string: Option<i8>, fret: Option<i8>, rest: Option<bool>) -> Self{
        Self{string, fret, rest}
    }

    pub fn get_string(&self) -> i8{
        if self.string != None{
            return self.string.unwrap();
        } 
        else{
            return -1;
        }
    }

    pub fn get_fret(&self) -> i8{
        if self.fret != None{
            return self.fret.unwrap();
        }
        else{
            return -1;
        }
    }   
}
