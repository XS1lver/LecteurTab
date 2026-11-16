use crate::measure::Measure;

pub struct Tab{
    name: String,
    measures: Vec<Measure>,
    nb_frets: i8,
    nb_strings: i8,
    tuning: [String]
}
