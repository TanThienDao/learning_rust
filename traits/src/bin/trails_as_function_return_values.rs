use std::collections::HashMap;

trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}
trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}
#[derive(Debug)]
struct Hotel {
    name: String,
    reservations: HashMap<String, u32>,
}
impl Hotel {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            reservations: HashMap::new(),
        }
    }
    fn get_reservation(&self) -> &HashMap<String, u32> {
        &self.reservations
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}
impl Accommodation for Hotel {
    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
    }
}
impl Description for Hotel {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name)
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
        format!("{} is the pinnacle of luxury", self.host)
    }
}
#[derive(Debug)]
struct TestDefault {
    name: String,
    list: Vec<Hotel>,
}
impl TestDefault {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            list: vec![],
        }
    }
}
impl Accommodation for TestDefault {
    fn book(&mut self, name: &str, nights: u32) {
        self.list.push(Hotel::new(name));
    }
}
impl Description for TestDefault {}
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
fn main() {
    let mut hotel = choose_best_place_to_stay();
    let mut air_bn = AirBnB::new("Air BnB");
    mix_and_match(&mut hotel, &mut air_bn, "Air BnB");
}
