use std::os::unix::raw::off_t;

#[derive(PartialEq, Eq)]
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
//impl Eq for Flight{}

/// The equality trait `Eq`, is a subtrait of the PartialEq trait.
/// There are will be 3 principal will applied
/// first:      reflexive:  a == a
/// Second :    symmetric:  a==b implied b==a ( required by PartialEq as well)
/// Third:      transitive: a == b and b == c implies a == c (require by PartialEq as well).
///
/// in case f32 != f63 did not implement Eq trait
fn main() {
    let a = Flight::new("New York", "Los Angeles", "10:00 AM");
    let b = Flight::new("New York", "Los Angeles", "10:00 AM");
    let c = Flight::new("New York", "Los Angeles", "10:00 AM");

    println!("a == b: {}", a == b);
    println!("b == c: {}", b == c);
    println!("a == c: {}", a == c);

    let division: f64 = 0.0 / 0.0;
    println!("division: {}", division);

    let var: f32 = 3.4;
    println!("var: {}", var == var);
    println!("{}", division == division); // Nan != Nan
}
