use std::io; // io::stdin().read_line(&mut <String>)
use std::f64; // f64::NAN

fn get_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(string) => string,
        Err(err)   => panic!("Error: {}", err),
    };
    input
}

fn main() {
    let pairs: isize = get_input()
        .trim()
        .parse::<isize>().expect("Error: can't parse input");

    let mut data: Vec<isize> = Vec::new();

    for _ in 0..pairs {
        let pairs: Vec<f64> = get_input()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<f64>().expect("Error: can't parse input"))
            .collect();

        let mut div = f64::NAN;

        for num in pairs {
            if div.is_nan() {
                div = num;
            }
            else {
                div /= num;
            }
        }
        let div = div.round() as isize;
        data.push(div);
    }

    println!("");
    for ans in data {
        print!("{} ", ans);
    }
    println!("");
}
