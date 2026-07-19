use trail::lodging::{Accommodation, AirBnB, Description, Hotel};
use trail::utils;
fn main() {
    let mut hotel1 = Hotel::new(String::from("The luxe"));
    let mut aiirbnb = AirBnB::new("The sweet except");
    println!("Hotels: {:?}", hotel1.summarize());
    hotel1.book("Dana", 5);

    let hotel2 = Hotel::new("The grand");
    println!("Hotels: {:?}", hotel2.summarize());

    let hotel3 = Hotel::new(vec!["The Sweet except", "The luxury"]);
    println!("Hotels: {:?}", hotel3.summarize_2());

    utils::book_for_one_night(&mut hotel1, "Bob");
    utils::mix_and_match(&mut hotel1, &mut aiirbnb, "Bob");
}
