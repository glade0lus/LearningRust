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
    get_input();

    let input: Vec<isize> = get_input()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Error: can't parse input"))
        .collect();

    let mut answer: isize = 0;

    for num in input {
        answer += num;
    }

    println!("{}", answer);
}
