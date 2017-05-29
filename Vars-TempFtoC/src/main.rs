use std::fs::File; // File::open(&str)
use std::io::prelude::*; // std::fs::File.read_to_string(&mut str)

fn get_input() -> String {
    let mut file = match File::open("input.txt") {
        Ok(input) => input,
        Err(err)  => panic!("Error: {}", err),
    };
    let mut input = String::new();
    match file.read_to_string(&mut input) {
        Ok(input) => input,
        Err(err)  => panic!("Error: {}", err),
    };
    input
}

fn main() {
    let data: Vec<isize> = get_input()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<isize>().expect("Error: can't parse input"))
            .collect();

    let mut c: Vec<f64> = Vec::new();
    for f in data[1..].iter().map(|f| *f as f64) {
        c.push((f + 40.0_f64) / 1.8 - 40.0_f64);
    }

    println!("");
    for ans in c {
        let ans = ans.round();
        print!("{} ", ans);
    }
    println!("");
}
