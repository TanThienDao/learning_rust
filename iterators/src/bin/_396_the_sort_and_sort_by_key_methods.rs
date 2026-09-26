///
#[derive(Debug)]
struct GasStation {
    stack_count: u32,
    manager: String,
    employee_count: u32,
}
fn main() {
    let mut points = [3, 8, 22, 1, 11, 4, 7, 2, 9, 5, 6, 10];
    println!("{:?}", points.is_sorted()); // Checks if the array is sorted
    points.sort();
    println!("{:?}", points.is_sorted()); // Checks if the array is sorted after sorting
    println!("{:?}", points);
    points.reverse();
    println!("{:?}", points);
    println!("is sorted: {:?}", points.is_sorted()); // only work for ascending order, so this will return false

    let mut exerise = ["Squat", "bench press", "deadlift", "overhead press"];
    exerise.sort(); // Sorts the exercises by the default order (alphabetical)
    println!("{:?}", exerise);

    let mobil = GasStation {
        stack_count: 4,
        manager: String::from("Alice Johnson"),
        employee_count: 12,
    };

    let exxon = GasStation {
        stack_count: 5,
        manager: String::from("John Doe"),
        employee_count: 10,
    };
    let shell = GasStation {
        stack_count: 3,
        manager: String::from("Jane Smith"),
        employee_count: 8,
    };

    let mut stops = [mobil, exxon, shell];
    stops.sort_by_key(|station| -(station.employee_count as i32)); // Sorts the gas stations by employee count in descending order
    println!("{:?}", stops);
}
