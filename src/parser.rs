use crate::tab::Tab;
use std::fs::File;

pub fn parse(file: &str) -> Tab{
    let f = File::open(file).expect("Erreur");
    serde_json::from_reader(f).unwrap() 
}
