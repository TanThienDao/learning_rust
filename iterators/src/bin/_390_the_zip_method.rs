/// The zip method combined two iterators into a single iterator based on their
/// elements having the same index position.
fn main() {
    let first_names = ["Alice", "Bob", "Charlie", "Dan"];
    let last_names = ["Smith", "Johnson", "Williams"];

    for (first, last) in first_names.iter().zip(last_names.iter()) {
        println!("{} {}", first, last);
    }

    first_names
        .iter()
        .zip(last_names.iter())
        .for_each(|(first, last)| {
            println!("{} {}", first, last);
        });

    let complete_names: Vec<String> = first_names
        .iter()
        .zip(last_names.iter())
        .map(|(first, last)| format!("{} {}", first, last))
        .collect();
    println!("{:?}", complete_names);
}
