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


    
    pub fn display_tab(&self){
        for i in 0..self.nb_strings{
            self.display_string(self.nb_strings - i - 1);
        }
        self.display_beats_durations();
    }

    fn display_beats_durations(&self){
        print!("   ");
        for measure in &self.measures{
            for beat in measure.get_beats(){
                beat.get_time().print_duration();
            }
            print!("  ");
        }
        print!("\n");
    }

    fn display_string(&self, string: u8){
        print!("{}: ", &self.tuning[string as usize]);
        for measure in &self.measures{
            for beat in measure.get_beats(){
                let mut string_played = false;
                for note in beat.get_notes(){
                    if note.get_string() == string{
                        print!("{} ", note.get_fret());
                        string_played = true;
                    }
                }
                if !string_played{
                    print!("  ");
                }
            }
            print!("| "); // Separation between measures 
        }
        print!("\n");
    }
}
