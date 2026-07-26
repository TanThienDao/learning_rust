/// The borrow checker is the part of the Rust compiler that validates that all borrows
/// (references) are valid
fn main() {
    let dog = String::from("Watson");
    let my_pet = &dog;
    /// The referent is the value being borrowed(dog).
    /// The referencce is the actual borrow (my_pet).
    println!("{dog}");
    println!("{my_pet}");
    {
        let something = &dog;
        println!("{something}");
    }
}
