use std::io; // io::stdin().read_line(&mut <String>)
use std::io::Write; // io::stdout().flush()

static VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

fn input(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush() // Explicit print to stdout
        .expect("Error: can't flush stdout");

    let mut string = String::new();
    io::stdin().read_line(&mut string)
        .expect("Error: can't read input from stdin");
    string.pop(); // Remove trailing \n
    string
}

fn to_pig_lat(string: String) -> String {
    let string_lowercase = string.to_lowercase();
	let mut suffix = String::new();
	let mut word = String::new();

    if 'a' == string_lowercase.chars().next().expect("Error: string is empty [to_pig_lat()]") {
        return format!("a-ay");
    }

    if 'h' == string_lowercase.chars().next().expect("Error: string is empty [to_pig_lat()]")
        && VOWELS.contains(&string_lowercase.chars().nth(1).expect("Error: string contains only one char [to_pig_lat()]"))
        {
        return format!("{}-ay", string);
    }

    for (i, c) in string.chars().enumerate() {
        if VOWELS.contains(&c) {
            word = string[i..].to_string();
            break;
        }
        else {
            suffix.push(c);
        }
    }

	format!("{}-{}ay", word, suffix)
}

fn from_pig_lat(string: String) -> String {
    if string.split('-').count() != 2 {
        panic!("Error: can't parse word (less or more than one '-' char) [from_pig_lat()]");
    }
    let ayless_string = string[..string.len()-2].to_string();
    let split: Vec<_> = ayless_string
        .split('-')
        .map(|s| s.to_string())
        .collect();

    format!("{}{}", split[1], split[0])
}

fn main() {
    let text = input("Enter the text:");
    let new_text = to_pig_lat(text);

    println!("{}", new_text);


    let text = input("Enter the text:");
    let new_text = from_pig_lat(text);

    println!("{}", new_text);

}
