use serde::Deserialize;
use std::error::Error;
use std::fs;
use crate::tab::Tab;

pub fn parse(file: &str) -> Tab{
    serde_json::from_str(file).unwrap() 
}
