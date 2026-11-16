use crate::measure::Measure;

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

    pub fn getName(&self) -> &String{
        return &self.name;
    }
    pub fn getMeasures(&self) -> &[Measure]{
        return &self.measures;
    }
    pub fn getNbFrets(&self) -> u8{
        return self.nb_frets;
    }
    pub fn getNbStrings(&self) -> u8{
        return self.nb_strings;
    }
    pub fn getTuning(&self) -> &[String]{
        return &self.tuning;
    }
}
