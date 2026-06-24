///String is a piece of text or, alternatively,
///a sequence of characters.
/// &str -> a string slice, ref str, reference to area of executable, embedded to the binary executable
/// String ->

#[test]
fn review_of_string() {
    let pirate = "Bloodhook";
    let sailer = String::from(pirate);
    let bad_guy = pirate.to_string();
    println!("{} and {} and {}", pirate, sailer, bad_guy);
    let first_initial = &pirate[0..4];
    println!("{} and {}", first_initial, pirate);
}
#[test]
fn concatenation() {
    let mut full_name = String::from("Sylvester");
    let last_name = "Stallone";
    full_name.push(' '); // push is add a single character
    full_name.push_str(last_name); // add a full string
    println!("{}", full_name);

    let first_name = String::from("Sylvester");
    let last_name = String::from("Blood");
    let full_name = first_name + &last_name; // first_name is moved and can no longer be used
    println!("{}", full_name);
}
#[test]
fn the_format_macro() {
    let first_name = String::from("Sylvester");
    let last_name = String::from("Blood");
    let full_name = format!("{} {}", first_name, last_name); // first_name and last_name are not moved
    println!("{}", full_name);
    println!("{}", &first_name);
}
#[test]
fn common_string_methods_trim_casing_replace() {
    let mut music_genres = "   Rock, Metal, Country, Rap    ";
    println!("{}", music_genres.trim());
    println!("{}", music_genres.trim_start());
    println!("{}", music_genres.trim_end());

    music_genres = music_genres.trim();
    println!("{}", music_genres.to_uppercase());
    println!("{}", music_genres.to_lowercase());
    println!("{}", music_genres.replace("a", "@"));

    let genres: Vec<&str> = music_genres.split(", ").collect();
    println!("{:?}", genres);
}
use std::io;
#[test]
fn collecting_user_input_with_read_line_method() {
    println!("Hello, world!");
    println!("Please enter your name: ");
    /// read_line will return a heap string reason is we can not predict what user input.
    /// this only work for main
    let mut name = String::new();
    let user_inout = io::stdin().read_line(&mut name);
    match user_inout {
        Ok(n) => println!("User name: {}", name.trim()),
        Err(ref e) if e.kind() == io::ErrorKind::Interrupted => println!("CTRL-C"),
        Err(e) => panic!("Error: {}", e),
    }
}
// project section 15 for string will be implemented at main.rs
