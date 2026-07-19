#[derive(PartialEq)]
struct Flight {
    origin: String,
    destination: String,
    time: String,
}
impl Flight {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}

/*impl PartialEq for Flight{
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}*/

/// The PartialEq trait establishes equality between two values.
fn main() {
    let flight_1 = Flight::new("Pheonix", "Japan", "8:00AM");
    let flight_2 = Flight::new("Pheonix", "Japan", "9:00AM");
    let flight_3 = Flight::new("Pheonix", "Los Angeles", "10:00AM");

    println!("{}", flight_1 == flight_2);
    println!("{}", flight_1 == flight_3);
    println!("{}", flight_1 != flight_3);

    println!("{}", flight_1.eq(&flight_2));
    println!("{}", flight_1.ne(&flight_3));
}
