/// The lines method returns an iterator of the string's individual lines.
use std::fs::read_to_string;
fn main() -> std::io::Result<()> {
    let text = "Hello, world!\nThis is a test.\nLet's see how it works.";
    for line in text.lines() {
        println!("{}", line);
    }

    // read a text file
    /// The try operator `?` is used to propagate errors.
    /// If reading the file fails, the error will be returned from the `main` function.
    /// It requires the `main` function to return a `Result` type,
    /// which is why we have `-> std::io::Result<()>` in the function signature.
    let file_content = read_to_string("src/bin/story.txt")?;
    for line in file_content.lines() {
        println!("{}", line);
    }
    Ok(())
}
