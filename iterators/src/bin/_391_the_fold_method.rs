/// The fold method exhausts an iterator to build up
/// and produce a single value at the end of iteration.
/// The fold method takes 2 arguments: an initial value and a closure.
/// The closure takes 2 arguments: an accumulator and the next value in the iterator.
/// The closure returns a new accumulator value which is passed to the next iteration.
/// The final accumulator value is returned at the end of iteration.
///
/// .fold (
/// ///     initial_value,
/// ///     |accumulator, current_value| {
/// ///         // do something with accumulator and current_value
/// ///         // return new accumulator value
/// ///     }
/// /// )
use std::collections::HashMap;

struct SupportStaff {
    day: String,
    employee: String,
}
fn main() {
    let earning = [100, 200, 300, 400, 500];
    let sum = earning.into_iter().fold(3, |total, current| {
        println!("Total: {}, Current: {}", total, current);
        total + current
    });
    println!("Total earning: {}", sum);

    let week = [
        SupportStaff {
            day: String::from("Monday"),
            employee: String::from("Alice"),
        },
        SupportStaff {
            day: String::from("Tuesday"),
            employee: String::from("Bob"),
        },
        SupportStaff {
            day: String::from("Wednesday"),
            employee: String::from("Alice"),
        },
        SupportStaff {
            day: String::from("Thursday"),
            employee: String::from("Charlie"),
        },
        SupportStaff {
            day: String::from("Friday"),
            employee: String::from("Bob"),
        },
    ];

    let mut hash_map_staff = week.into_iter().fold(HashMap::new(), |mut data, entry| {
        data.insert(entry.day, entry.employee);
        data
    });
    println!("HashMap of Support Staff: {:?}", hash_map_staff);
}
