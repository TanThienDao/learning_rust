/// A Closure is a function without a name.
/// It is sometimes called an anonymous function
/// or lambda.
///
/// Closure is helpful when we want to declare a quick,
/// one-off procedure that doesn't reallt merit a.
///
/// Functional programing treats a function like any other value in a program.
///
fn main() {
    let multiplier = 5;
    fn multiply_bt(value: i32) -> i32 {
        value * multiplier
    }
    println!("{}", multiply_bt(5));
}
