fn main() {
    let multiplier = 5;
    /*    fn multiply_bt (value: i32) -> i32 {
        value * multiplier
    }*/

    let multiply_by = |value: i32| -> i32 {
        return value * multiplier;
    };

    println!("{}", multiply_by(5));

    let product = |a: i32, b: i32| -> i32 { return a * b };
    println!("{}", product(5, 5));
}
