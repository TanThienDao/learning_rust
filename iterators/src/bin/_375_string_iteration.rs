/// The Chars method returns an iterator of the Unicode characters.
/// The Bytes method returns an iterator of the raw bytes of the string.

fn main() {
    let seafood = String::from("Oysters 🦪 🦪 🦪");
    for byte in seafood.bytes() {
        print!("{}/ ", byte);
    }
    println!("");

    for char in seafood.chars() {
        print!(" {}/ ", char);
    }

    println!("");
    ///These method will expaute the iterators and return the length of the string in bytes and characters.
    println!("Seafood len: {:?}", seafood.len());
    println!("Seafood bytes: {:?}", seafood.bytes());

    let seasfood_bytes: Vec<u8> = seafood.bytes().collect();
    println!("Seafood chars: {:?}", seafood.chars().count()); // this exhausts the iterator and returns the count of characters in the string
    // The chars() method returns an iterator of the Unicode characters in the string, which can be collected into a vector or used in a for loop.
    println!("Seafood chars: {:?}", seafood.chars());
}
