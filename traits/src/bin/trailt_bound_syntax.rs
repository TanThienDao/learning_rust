use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
    fn book(&mut self, name: &str, nights: u32);
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
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name)
    }
    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
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
    fn get_description(&self) -> String {
        format!("Please enjoy {}'s apartment.", self.host)
    }
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
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

    fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}
impl Accommodation for TestDefault {
    fn book(&mut self, name: &str, nights: u32) {
        self.list.push(Hotel::new(name));
    }
}
fn book_for_one_night<T: Accommodation>(entity: &mut T, guest: &str) {
    println!("Booking for one night: {}", entity.get_description());
    entity.book(guest, 1);
}
/// A trait bouynd requires that a generic type implement a specific trait
fn mix_and_match(first: &mut impl Accommodation, secound: &mut impl Accommodation, guest: &str) {
    first.book(guest, 1);
    secound.book(guest, 1);
}
fn mix_and_match_2<T: Accommodation, G: Accommodation>(
    first: &mut T,
    secound: &mut G,
    guest: &str,
) {
    first.book(guest, 1);
    secound.book(guest, 1);
}

fn main() {
    let mut hotel = Hotel::new("Hotel");
    let mut air_bn = AirBnB::new("Air BnB");
    mix_and_match_2(&mut air_bn, &mut hotel, "Bob");
    println!("{:#?} {:#?}", hotel, air_bn);
}
