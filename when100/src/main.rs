use std::io;
use std::io::*;

fn main() {
    print!("What is your name? ");
    io::stdout().flush().unwrap(); // flush
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Error: unable to read user's name.");
    let name_trimmed = name.trim();

    print!("How old are you? ");
    io::stdout().flush().unwrap();
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Error: unable to read user's age.");

    let age_trimmed = age.trim(); // remove leading and trailing whitespaces
    match age_trimmed.parse::<u32>() {
        Ok(i) => println!("{} will turn 100 in {}.", name_trimmed, 100-i+2020),
        Err(..) => println!("Error: unable to read user's age. It was {}", age_trimmed),
    };
}
