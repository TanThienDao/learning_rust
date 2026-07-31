fn main() {
    let multiplier = 5;
    let multiply_by = |value| value * multiplier;
    println!("The result is: {}", multiply_by(3 as u8));
    //println!("The result is: {}", multiply_by(100));

    let mirror = |value| value;
    println!("The result is:{}", mirror("whY"));
    //println!("The result is:{}", mirror(1));
}
