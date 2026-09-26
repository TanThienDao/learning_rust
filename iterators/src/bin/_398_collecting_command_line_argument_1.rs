use std::env;

/// Command-line arguments are values passed into the program
/// from the terminal command prompt.
fn main() {
    let args = env::args();

    for arg in args {
        println!("{}", arg);
    }
}
