fn main() {
    let mut numbers = vec![4, 8, 15, 16, 23, 42];
    let mut add_number = || numbers.push(100);
    //println!("{:?}", numbers); // numbers ownership already move and capture by closure.
    add_number();
    add_number(); // live time of capture value in closure is end then we can re use numbers
    println!("{:?}", numbers);
}
