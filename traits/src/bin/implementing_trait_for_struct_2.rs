use std::collections::HashMap;

trait Accommodation {
    fn get_description(&self) -> String;
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
fn main() {
    let mut hotel = Hotel::new("The Luxe");
    println!("{}", hotel.get_description());
    let mut air_bn = AirBnB::new("Tan");
    println!("{}", air_bn.get_description());
    hotel.book("The Luxe", 20);
    println!(" Hotel info: {:#?}", hotel.get_reservation());
    air_bn.book("Nam", 21);
    println!("Airbnb info: {:#?}", air_bn.get_guests());
    println!("Hotel info: {:#?}", hotel);
    println!("AirBnB info: {:#?}", air_bn);
}
