use std::fmt::format;
use std::io;

mod test;
fn main() {
    let mut amount = String::from("100");
    make_money(&mut amount);
    println!("amount {}", amount);

    let trimmed_and_capitalized = trim_and_capitalize("   hello world   ");
    println!("trimmed and capitalized: {}", trimmed_and_capitalized);

    let elements = element("Gold!Silver!Platinum");
    println!("elements: {:?}", elements);

    let name = get_identity();
    println!("name: {}", name);
}
fn make_money(string: &mut String) {
    string.push_str("$$$");
}

fn trim_and_capitalize(string: &str) -> String {
    string.trim().to_uppercase()
}

fn element(string: &str) -> Vec<&str> {
    string.split('!').collect::<Vec<&str>>()
}

fn get_identity() -> String {
    println!("Please enter your name: ");
    let mut first_name = String::new();
    let mut last_name = String::new();
    let input = io::stdin();
    println!("Please enter your first name:");
    input
        .read_line(&mut first_name)
        .expect("Failed to read line first name");

    println!("Please enter your last name:");
    input
        .read_line(&mut last_name)
        .expect("Failed to read line last name");
    format!("{} {}",first_name.trim(),last_name.trim())
}
