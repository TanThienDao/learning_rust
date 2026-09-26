/// The flatten method is an adapter that return an iterator that
/// flattens nested data structures.

fn main() {
    let spreadsheet = vec![[100, 200, 300], [123, 456, 789], [987, 654, 321]];
    let flattened: Vec<i32> = spreadsheet.into_iter().flatten().collect();
    println!("{:?}", flattened);
    //println!("{:#?}", spreadsheet);
}
