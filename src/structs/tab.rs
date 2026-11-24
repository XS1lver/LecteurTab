use crate::measure::Measure;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Tab{
    pub name: String,
    //balance: i8,
    //volume: u8,
    pub measures: Vec<Measure>,
    //frets: u8,
    pub strings: i8,
    pub tuning: Vec<u16>
}

impl Tab{
    /*pub fn new(name: String, measures: Vec<Measure>, strings: i8, tuning: Vec<u16>) -> Self{
        Self{name, measures, strings, tuning}
    }*/

    fn midi_to_tuning(midi: u16){
        const NOTES: [&str; 12] = [
            "C ", "C#", "D ", "D#", "E ", "F ", "F#", "G ", "G#", "A ", "A#", "B "
        ];
        let note = NOTES[(midi %  12) as usize];
        print!("{}: ", note);
    }
    
    pub fn display_tab(&self){
        let mut cpt: usize = 0;
        println!("Nom: {}", &self.name);
        while cpt < self.measures.len(){
            let max: usize;
            // Print 4 measures per line
            if cpt + 4 > self.measures.len(){
                max = self.measures.len() - cpt;
            }
            else{
                max = cpt + 4;
            }
            for i in 0..self.strings{
                Self::midi_to_tuning(self.tuning[i as usize]);
                for j in cpt..max{
                    let measure = &self.measures[j];
                    measure.display_string(i);
                }
                print!("\n");
            }
            self.display_beats_durations(cpt, max);
            cpt = max;
        }
    }

    fn display_beats_durations(&self, start: usize, end: usize){
        print!("    ");
        for i in start..end{
            let measure = &self.measures[i as usize];
            for voice in &measure.voices{
                for beat in &voice.beats{
                    beat.duration.print_duration(beat.get_dots(), beat.get_rest());
                }
            }
            print!("  ");
        }
        print!("\n");
    }
}
