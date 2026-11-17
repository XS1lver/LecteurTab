use crate::measure::Measure;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tab{
    name: String,
    measures: Vec<Measure>,
    nb_frets: u8,
    nb_strings: u8,
    tuning: Vec<String>
}

impl Tab{
    pub fn new(name: String, measures: Vec<Measure>, nb_frets: u8, nb_strings: u8, tuning: Vec<String>) -> Self{
        Self{name, measures, nb_frets, nb_strings, tuning}
    }

    pub fn get_name(&self) -> &String{
        return &self.name;
    }
    pub fn get_measures(&self) -> &[Measure]{
        return &self.measures;
    }
    pub fn get_nb_frets(&self) -> u8{
        return self.nb_frets;
    }
    pub fn get_nb_strings(&self) -> u8{
        return self.nb_strings;
    }
    pub fn get_tuning(&self) -> &[String]{
        return &self.tuning;
    }
}
