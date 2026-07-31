fn main() {
    let multiplier = 5;

    let multiply_by = |value: u8| value * multiplier;
    println!("{}", multiply_by(10));

    let numbers = vec![4, 8, 15, 16, 23, 42];
    //println!("{:?}", numbers);

    let print_numbers = || println!("{:?}", numbers);
    print_numbers();
    print_numbers();
    print_numbers();
    println!("{:#?}", numbers);
}
