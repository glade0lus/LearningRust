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

    let mut max = isize::min_value();
    let mut min = isize::max_value();

    for num in data {
        if num > max {
            max = num;
        }
        if num < min {
            min = num;
        }
    }

    println!("{} {}", max, min);
}
