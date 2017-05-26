use std::io; // io::stdin().read_line(&mut <String>)
use std::io::Write; // io::stdout().flush()

fn reverse(string: String) -> String {
    let mut rstring = String::with_capacity(string.len());
    for c in string.chars().rev() {
        rstring.push(c);
    }
    return rstring;
}

fn main() {
    print!("Enter string: ");
    io::stdout().flush() // Explicit print to stdout
        .expect("Error");

    let mut string = String::new();
    match io::stdin().read_line(&mut string) {
        Ok(string) => string,
        Err(err)     => panic!("Error: {}", err),
    };
    string.pop(); // Remove trailing \n

    println!("Reversed string: {}", reverse(string));
}
