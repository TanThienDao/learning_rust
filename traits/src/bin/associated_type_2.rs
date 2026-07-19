use std::ops::Add;
fn add_two_numbers<T: Add<Output = T>>(a: T, b:T) -> T {
    a + b
}

fn main() {
    let integer_sum = add_two_numbers(1,2);
    println!("Sum of integers: {}", integer_sum);
    let float_sum = add_two_numbers(1.5, 2.5);
    println!("Sum of floats: {:.4}", float_sum);
}