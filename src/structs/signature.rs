use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Signature{
    pub num: u8,
    pub denom: u8
}

impl Signature{
    /*pub fn new(num: u8, denom: u8) -> Self{
        Self{num, denom}
    }

    pub fn add(&mut self, a: &Signature){
        if self.denom == a.denom{
            self.num += a.num;
        }
        else{
            self.num = self.num * a.denom + a.num * self.denom;
            self.denom = self.denom * a.denom;
        }
    }*/

    pub fn print_duration(&self){
        const DURATIONS: [&str; 8] = [
            "𝅝", "𝅗𝅥", "𝅘𝅥", "𝅘𝅥𝅮", "𝅘𝅥𝅯", "𝅘𝅥𝅰", "𝅘𝅥𝅱", "𝅘𝅥𝅲"
        ];
        print!("\x1b[{}m{}", 36, ""); // Blue color
        print!("{} ", DURATIONS[self.denom.ilog2() as usize]);
        print!("\x1b[m"); // Return to default color settings
    }
}
