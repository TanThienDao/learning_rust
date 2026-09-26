/// The reduce methods is similar to fold but it supplies
/// the iterator's first element as the starter value.
///
/// The reduce method returns an Option enum to account for the possibility of an empty iterator.
///
/// it reduce to only accept closure with 2 arguments: an accumulator and the next value in the iterator.
/// it does not let us modify the starter value, and it does not let us specify a starter value.

fn main() {
    let earning = [100, 200, 300, 400, 500];
    let sum = earning.into_iter().reduce(|a, b| a + b);
    println!("Total earning: {:?}", sum.unwrap());

    let earning_empty: [i32; 0] = [];
    let sum_empty = earning_empty.into_iter().reduce(|a, b| a + b);
    println!(
        "Total earning for empty array: {:?}",
        sum_empty.unwrap_or(0)
    );

    let address_portions = [
        String::from("123 Main St"),
        String::from("Apt 4B"),
        String::from("Springfield"),
        String::from("IL"),
        String::from("62704"),
    ];
    let address_portions_2 = address_portions.clone();

    let full_address = address_portions
        .into_iter()
        .reduce(|mut accumulator, portion| {
            accumulator.push_str(", ");
            accumulator.push_str(&portion);
            accumulator
        });
    println!("Full address: {:?}", address_portions_2.join(", "));
    println!("Full address: {:?}", full_address.unwrap());
}
