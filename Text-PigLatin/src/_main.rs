use std::io; // io::stdin().read_line(&mut <String>)
use std::io::Write; // io::stdout().flush()

fn to_pig_latin(string: String) -> String {

}

fn from_pig_latin(string: String) -> String {

}

fn main() {
    static VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
    static CONSONANTS: [char, 20] = ['b', 'c', 'd', 'f', 'g',
        'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't',
        'v', 'w', 'x', 'z'];

    print!("Enter string: ");
    io::stdout().flush() // Explicit print to stdout
        .expect("Error");

    let mut string = String::new();
    match io::stdin().read_line(&mut string) {
        Ok(num)  => num,
        Err(err) => panic!("Error: {}", err),
    };
    string.pop(); // Remove trailing \n

    println!("1. To pig latin\n2. From pig latin");

    let mut choice = String::new();
    match io::stdin().read_line(&mut choice) {
        Ok(num)  => num,
        Err(err) => panic!("Error: {}", err),
    };

    let new_string = match choice.chars().next() {
        '1' => to_pig_latin(string),
        '2' => from_pig_latin(string),
    };
    println!("Translated string: {}", new_string);
}
