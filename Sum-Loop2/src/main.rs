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

    let mut answers: Vec<isize> = Vec::new();

    for _ in 0..pairs {
        answers.push(get_input()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<isize>().expect("Error: can't parse input"))
            .sum()
        );
    }

    println!("");
    for ans in answers {
        print!("{} ", ans);
    }
    println!("");
}
