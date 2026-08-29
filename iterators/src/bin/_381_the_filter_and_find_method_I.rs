/// The filer method extracts a subset of values that satisfy a condition.
/// Pass a closure that returns true for the elements to keep and false
/// for the elements to exclude.
fn main() {
    let numbers = [10, 13, 23, 2, 8, 9, 6];
    let evens: Vec<&i32> = numbers.iter().filter(|number| *number % 2 == 0).collect();
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|number| *number % 2 == 0)
        .copied()
        .collect();
    let evens: Vec<i32> = numbers
        .into_iter()
        .filter(|number| number % 2 == 0)
        .collect();
    println!("Even numbers: {:?}", evens);
    println!("{:?}", numbers);

    let first_even = numbers.into_iter().find(|number| *number % 2 == 0);
    match first_even {
        Some(value) => println!("Even number is {}", value),
        None => println!("No even number found"),
    }

    let first_odd = numbers.into_iter().find(|number| number % 2 == 1);
    match first_odd {
        Some(value) => println!("Odd number is {}", value),
        None => println!("No odd number found"),
    }

    let nothing = numbers.into_iter().find(|number| *number > 100);
    match nothing {
        Some(value) => println!("Found number greater than 100: {}", value),
        None => println!("No number greater than 100 found"),
    }

    let last_even = numbers.into_iter().rfind(|number| *number % 2 == 0);
    match last_even {
        Some(value) => println!("Even number is {}", value),
        None => println!("No even number found"),
    }

    let last_odd = numbers.into_iter().rfind(|number| number % 2 == 1);
    match last_odd {
        Some(value) => println!("Odd number is {}", value),
        None => println!("No odd number found"),
    }

    let nothing = numbers.into_iter().rfind(|number| *number > 100);
    match nothing {
        Some(value) => println!("Found number greater than 100: {}", value),
        None => println!("No number greater than 100 found"),
    }

    // rfind is the reverse of find, it returns the last element that satisfies the condition.
    //start from the right
}
