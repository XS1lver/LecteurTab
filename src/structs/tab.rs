use crate::measure::Measure;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tab{
    name: String,
    balance: i8,
    volume: u8,
    measures: Vec<Measure>,
    frets: u8,
    strings: i8,
    tuning: Vec<u16>
}

impl Tab{
    pub fn new(name: String, balance: i8, volume: u8, measures: Vec<Measure>, frets: u8, strings: i8, tuning: Vec<u16>) -> Self{
        Self{name, balance, volume, measures, frets, strings, tuning}
    }

    pub fn get_name(&self) -> &String{
        return &self.name;
    }
    pub fn get_measures(&self) -> &[Measure]{
        return &self.measures;
    }
    pub fn get_nb_frets(&self) -> u8{
        return self.frets;
    }
    pub fn get_nb_strings(&self) -> i8{
        return self.strings;
    }
    pub fn get_tuning(&self) -> &[u16]{
        return &self.tuning;
    }

    fn midi_to_tuning(midi: u16){
        const NOTES: [&str; 12] = [
            "C ", "C#", "D ", "D#", "E ", "F ", "F#", "G ", "G#", "A ", "A#", "B "
        ];
        let note = NOTES[(midi %  12) as usize];
        print!("{}: ", note);
    }
    
    pub fn display_tab(&self){
        for i in 0..self.strings{
            self.display_string(i);
        }
        self.display_beats_durations();
    }

    fn display_beats_durations(&self){
        print!("    ");
        for measure in &self.measures{
            for voice in measure.get_voices(){
                for beat in voice.get_beats(){
                    beat.get_duration().print_duration();
                }
            }
            print!("  ");
        }
        print!("\n");
    }

    fn display_string(&self, string: i8){
        Self::midi_to_tuning(self.tuning[string as usize]);
        for measure in &self.measures{
            for voice in measure.get_voices(){
                for beat in voice.get_beats(){
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
            }
            print!("| "); // Separation between measures 
        }
        print!("\n");
    }
}
