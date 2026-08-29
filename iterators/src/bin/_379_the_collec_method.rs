/// The collect method exhausts the iterator and gathers the resulting values
/// in a new collection type.
use std::collections::HashSet;
fn main() {
    let numbers = vec![4, 8, 15, 16, 23, 42];
    let squares: Vec<i32> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
    let squares: Vec<_> = numbers.iter().map(|number: &i32| number.pow(2)).collect();
    let squares = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<Vec<i32>>();
    let squares = numbers
        .iter()
        .map(|number: &i32| number.pow(2))
        .collect::<HashSet<i32>>();
    println!("squares: {:?}", squares);
    println!("numbers: {:?}", numbers);
}
