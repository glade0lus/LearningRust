use std::fs::File; // File::open(&str)
use std::io::prelude::*; // std::fs::File.read_to_string(&mut str)

static VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

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
    let input = get_input()
        .to_lowercase();

    let data: Vec<&str> = input
        .trim()
        .lines()
        .collect();

    let mut counters: Vec<isize> = Vec::new();
    for (i, string) in data[1..].iter().enumerate() {
        counters.push(0);
        for ch in string.chars() {
            if VOWELS.contains(&ch) {
                counters[i] += 1;
            }
        }
    }

    println!("");
    for ans in counters {
        print!("{} ", ans);
    }
    println!("");
}
