use std::collections::HashMap;
use std::fmt::{Debug, Display};

trait Accommodation: Debug {
    // Make Debug a supertrait of Accommodation
    fn book(&mut self, name: &str, nights: u32);
}
trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}
#[derive(Debug)]
struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}
impl<T> Hotel<T> {
    fn new(name: T) -> Self {
        Self {
            name: name,
            reservations: HashMap::new(),
        }
    }
}
// using trait bound to resolve the issue with to summarize function missing display trait
impl<T: Display> Hotel<T> {
    /*    fn new(name: T) -> Self {
        Self {
            name: name,
            reservations: HashMap::new(),
        }
    }*/
    fn get_reservation(&self) -> &HashMap<String, u32> {
        &self.reservations
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}
impl<T: Display + Debug> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
    }
}
impl<T: Display> Description for Hotel<T> {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury hotel", self.name)
    }
}
#[derive(Debug)]
struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}
impl AirBnB {
    fn new(host: &str) -> Self {
        Self {
            host: host.to_string(),
            guests: vec![],
        }
    }
    fn get_guests(&self) -> &Vec<(String, u32)> {
        &self.guests
    }
}
impl Accommodation for AirBnB {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}
impl Description for AirBnB {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury airbnb", self.host)
    }
}
#[derive(Debug)]
struct TestDefault<T> {
    name: String,
    list: Vec<Hotel<T>>,
}
impl<T> TestDefault<T> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            list: vec![],
        }
    }
}
/*impl<T> Accommodation for TestDefault<T> {
    fn book(&mut self, name: &str, nights: u32) {
        self.list.push(Hotel::new(name));
    }
}*/
impl<T> Description for TestDefault<T> {}
fn book_for_one_night<T: Accommodation + Description>(entity: &mut T, guest: &str) {
    println!(
        "Booking for one night at {} for {}",
        entity.get_description(),
        guest
    );
    entity.book(guest, 1);
}
/// A trait bouynd requires that a generic type implement a specific trait
fn mix_and_match(
    first: &mut (impl Accommodation + Description),
    secound: &mut impl Accommodation,
    guest: &str,
) {
    first.book(guest, 1);
    first.get_description();
    secound.book(guest, 1);
}
fn mix_and_match_2<T: Accommodation + Description, G: Accommodation>(
    first: &mut T,
    secound: &mut G,
    guest: &str,
) {
    first.book(guest, 1);
    secound.book(guest, 1);
}
fn mix_and_match_3<T, G>(first: &mut T, secound: &mut G, guest: &str)
where
    T: Accommodation + Description,
    G: Accommodation,
{
    first.book(guest, 1);
    first.get_description();
    secound.book(guest, 1);
}
fn choose_best_place_to_stay() -> impl Accommodation + Description {
    Hotel::new("The luxe")
}

/// A trait object is an instance of a type that implements a particular trait
/// whose methods will be accessed at runtime using a feature called dynamic dispatch.
fn main() {
    let mut hotel1 = Hotel::new(String::from("The luxe"));
    let mut airbnb = AirBnB::new("air bnb house ");

    let stays: Vec<&dyn Description> = vec![&hotel1, &airbnb];
    println!("{}", stays[0].get_description());
    println!("{}", stays[1].get_description());

    let mut stays_2: Vec<&mut dyn Accommodation> = vec![&mut hotel1, &mut airbnb]; // Changed back to dyn Accommodation
    stays_2[0].book("peer", 2);
    stays_2[1].book("haha", 3);

    //println!("{:#?}", hotel1);

    //println!("{:#?}", airbnb);

    for place in stays_2.iter_mut() {
        println!("check: {:#?}", place);
    }
}
