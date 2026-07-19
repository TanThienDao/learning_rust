struct BusTrip {
    origin: String,
    destination: String,
    time: String,
}
impl BusTrip {
    fn new(origin: &str, destination: &str, time: &str) -> Self {
        Self {
            origin: origin.to_string(),
            destination: destination.to_string(),
            time: time.to_string(),
        }
    }
}
impl PartialEq for BusTrip {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin
            && self.destination == other.destination
            && self.time == other.time
    }
}
impl PartialEq<Flight> for BusTrip {
    fn eq(&self, other: &Flight) -> bool {
        self.time == other.time
    }
}
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

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}
impl PartialEq<BusTrip> for Flight {
    fn eq(&self, other: &BusTrip) -> bool {
        self.time == other.time
    }
}

/// The PartialEq trait establishes equality between two values.
fn main() {
    let flight_1 = Flight::new("Pheonix", "Japan", "8:00AM");
    let bustrip = BusTrip::new("Pheonix", "Japan", "9:00AM");

    println!("{}", flight_1 == bustrip);
    println!("{}", bustrip == flight_1);
    println!("{}", flight_1.eq(&bustrip));
    println!("{}", bustrip == bustrip);
}
