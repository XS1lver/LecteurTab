mod structs;
mod parser;

use structs::tab;
use structs::signature;
use structs::note;
use structs::measure;
use structs::beat;
use crate::tab::Tab;

fn main() {
    println!("Hello, world!");
    let tab: Tab = parser::parse("ressources/test.json");
    println!("Nom: {}",tab.get_name());
    println!("Nb frettes: {}", tab.get_nb_frets());
    println!("Nb cordes: {}", tab.get_nb_strings());
    //println!("Accordage: {}", tab.get_tuning());
}
