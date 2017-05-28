use std::io; // io::stdin().read_line(&mut <String>)

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(string) => string,
        Err(err)   => panic!("Error: {}", err),
    };

    let input: Vec<isize> = input
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
