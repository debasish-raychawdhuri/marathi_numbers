use std::io;

use colored::*;
use rand::prelude::*;

fn to_marathi(v: u16) -> String {
    const MARATHI_NUMERALS: [char; 10] = ['०', '१', '२', '३', '४', '५', '६', '७', '८', '९'];
    let mut output = String::new();
    let mut w = v;
    while w > 0 {
        output = format!("{}", MARATHI_NUMERALS[(w % 10) as usize]) + output.as_str();
        w /= 10;
    }
    output
}

fn main() {
    println!("Marathi Numerals Training!");

    loop {
        let mut rng = rand::thread_rng();
        let v: u16 = rng.gen();
        println!("{}", to_marathi(v));

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: u16 = input.trim().parse().unwrap();

        if n == v {
            println!("{}", "Correct".green());
        } else {
            println!("{} {}", "Correct answer is".red(), v);
        }
    }
}
