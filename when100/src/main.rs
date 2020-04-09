extern crate chrono;

use std::io;
use std::io::*;

use chrono::prelude::*;

fn main() {
    print!("What is your name? ");
    io::stdout().flush(); // flush
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Error: unable to read user's name.");
    let name_trimmed = name.trim();

    print!("How old are you? ");
    io::stdout().flush();
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Error: unable to read user's age.");

    let this_year = Utc::now().year();
    let age_trimmed = age.trim_right(); // remove trailing whitespaces
    match age_trimmed.parse::<i32>() {
        Ok(i) => println!("{} will turn 100 in {}.", name_trimmed, 100-i+this_year),
        Err(..) => println!("Error: unable to read user's age. It was {}", age_trimmed),
    };
}
