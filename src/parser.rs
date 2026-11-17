use serde::Deserialize;

use std::fs;
use crate::tab::Tab;

pub fn parse(file: String) -> Tab{
    let content = fs::read_to_string(file);
    let tab: Tab = serde_json::from_str(&content).expect("Echec");

    return tab;
}
