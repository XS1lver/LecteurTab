mod structs;
mod parser;

use structs::tab;
use structs::signature;
use structs::note;
use structs::measure;
use structs::beat;
use crate::tab::Tab;

fn main() {
    let tab: Tab = parser::parse("ressources/test.json");
    println!("Nom: {}",tab.get_name());
    tab.display_tab();
}
