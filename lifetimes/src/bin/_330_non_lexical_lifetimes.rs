/// Lexical means lasting until the end of the block.
/// Non-lexical means not lasting until the end of the block.
///
/// The borrow checker treats the end of a reference's life time
/// as the last place it is used; a references has non-lexical scope
///
/*fn main() {
    let dog = String::from("Watson");
}
*/
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    println!("After defining closure: {list:?}");
}
