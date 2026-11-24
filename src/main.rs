mod structs;
mod parser;

use structs::tab;
use structs::signature;
use structs::note;
use structs::measure;
use structs::beat;
use structs::voice;
use crate::tab::Tab;

fn main() {
    let tab: Tab = parser::parse("ressources/1.json");
    tab.display_tab();
}
