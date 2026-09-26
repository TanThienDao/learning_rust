/// The partition method groups and returns the values for which
/// the closure returns true and for which the closure return false.
fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (even_numbers, odd_numbers): (Vec<i32>, Vec<i32>) =
        numbers.iter().partition(|&n| n % 2 == 0);
    println!("Even numbers: {:?}", even_numbers);
    println!("Odd numbers: {:?}", odd_numbers);

    let groups: (Vec<i32>, Vec<i32>) = numbers.iter().partition(|&n| n % 2 == 0);
    println!("Groups: {:?}", groups);
}
