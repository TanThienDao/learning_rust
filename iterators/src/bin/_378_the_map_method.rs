/// An adapter method is one that transforms an iterator into another
/// iterator based on some logic.
///
/// The map method applies a closure to each item
/// in a iterator to arrive at a new iterator of
/// values.
fn main() {
    let mut numbers = vec![4, 8, 15, 16, 23, 42];
    let my_iterator = numbers.iter();
    let squares = my_iterator.map(|number: &i32| number.pow(2));
    //println!("{:?}", my_iterator);
    println!("spares: {:?}", squares);
    for n in squares {
        println!("{}", n);
    }
    //println!("{:?}", squares);
    println!("{:?}", numbers);

    for number in numbers.iter().map(|number| number.pow(2)) {
        println!("{}", number);
    }
    println!("{:?}", numbers);
}
