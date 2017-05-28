use std::io; // io::stdin().read_line(&mut <String>)

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
        data.push(get_input()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<isize>().expect("Error: can't parse input"))
            .min().unwrap()
        );
    }

    println!("");
    for ans in data {
        print!("{} ", ans);
    }
    println!("");
}
