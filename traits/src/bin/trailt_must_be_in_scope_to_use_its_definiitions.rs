use std::ops::Add;
use std::str::FromStr;
// trait need to be available

fn main() {
    let a: i32 = 5;
    let b: i32 = 10;
    let sum = a.add(b);
    println!("{sum}");
    let numberic_count = u64::from_str("5");
    println!("{}", numberic_count.unwrap());
}
