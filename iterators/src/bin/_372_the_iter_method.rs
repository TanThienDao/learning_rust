/// The iter method will create an iterator that yields immutable references
/// to the collection's elements.
fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = my_vector.iter();
    for value in my_iterator {
        println!("value: {}", value);
    }
    println!("my_vector: {:?}", my_vector); // my_vector still exists and can be used after the iteration
    for value in &my_vector {
        println!("value: {}", value);
    }
    // This desgin principle matters, especially when we are dealing with owned types that do not implement
    // the Copy trait. The iter method allows us to iterate over the collection without taking ownership of its elements,
    // same as the iter()

    let cites = vec![String::from("Phoenix"), String::from("Dallas")];
    for city in cites {
        println!("city: {}", city);
    }
    //println!("my_vector: {:?}", cites); // This will not compile because cites has been moved into the for loop and is no longer available after the loop`
}
