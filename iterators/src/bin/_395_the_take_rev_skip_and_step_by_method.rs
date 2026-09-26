fn main() {
    let fifty_numers = (1..=50);

    /// Take the first 15 numbers from the range and print them
    for num in fifty_numers.take(15) {
        println!("{}", num);
    }

    /// The rev method reverses the order of the elements in the iterator.
    let reversed_numbers = (1..=50).rev();
    println!("Reversed numbers:");
    for num in reversed_numbers {
        println!("{}", num);
    }

    /// The skip method skips the first 10 numbers and prints the rest
    let skipped_numbers = (1..=50).skip(10);
    println!("Skipped numbers:");
    for num in skipped_numbers {
        println!("{}", num);
    }

    /// The step_by method takes every 5th number from the range and prints them
    let stepped_numbers = (1..=50).step_by(5);
    println!("Stepped numbers:");
    for num in stepped_numbers {
        println!("{}", num);
    }

    /// Combining the methods: take the first 20 numbers, skip the first 5, and then take every 3rd number
    let combined_numbers = (1..=50).take(20).skip(5).step_by(3);
    println!("Combined numbers:");
    for num in combined_numbers {
        println!("{}", num);
    }
}
