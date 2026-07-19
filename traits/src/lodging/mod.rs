use std::collections::HashMap;
use std::fmt::{Debug, Display};

pub trait Accommodation {
    fn book(&mut self, name: &str, nights: u32);
}
pub trait Description {
    fn get_description(&self) -> String {
        String::from("A wonderful place to stay")
    }
}
#[derive(Debug)]
pub struct Hotel<T> {
    name: T,
    reservations: HashMap<String, u32>,
}
impl<T> Hotel<T> {
    pub fn new(name: T) -> Self {
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

    pub fn summarize(&self) -> String {
        format!("{}: {}", self.name, self.get_description())
    }
}
impl<T: Debug> Hotel<T> {
    pub fn summarize_2(&self) -> String {
        format!("{:#?}:", self.name)
    }
}
impl<T: Display> Accommodation for Hotel<T> {
    fn book(&mut self, name: &str, night: u32) {
        self.reservations.insert(name.to_string(), night);
    }
}
impl<T: Display> Description for Hotel<T> {
    fn get_description(&self) -> String {
        format!("{} is the pinnacle of luxury", self.name)
    }
}
#[derive(Debug)]
pub struct AirBnB {
    host: String,
    guests: Vec<(String, u32)>,
}
impl AirBnB {
    pub fn new(host: &str) -> Self {
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

impl<T> Description for TestDefault<T> {}
