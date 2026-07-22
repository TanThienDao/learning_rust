use std::fs::File;

mod test;

fn main() {
    println!("Hello, world!");
    let greeting_file_result = File::open("hello.txt").expect("Failed to open hello.txt");
}
