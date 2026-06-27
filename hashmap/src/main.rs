mod tetst;
use std::collections::HashMap;

/// Project practice for section 16 will me in main.rs
fn main() {
    let mut sources_to_meals = HashMap::<&str, Vec<&str>>::from([
        ("Ketchup", vec!["French Fries", "Burgers", "Hot Dogs"]),
        ("Mayonnaise", vec!["Sandwiches", "Burgers", "Coleslaw"]),
        ("Mustard", vec!["Hot Dogs", "Pretzels", "Burgers"]),
    ]);
    let mayonaise = sources_to_meals.remove("Mayonnaise");
    println!("1 {:#?}", mayonaise);

    let mustard = sources_to_meals.get("Mustard");
    println!("2 {:#?}", mustard);

    sources_to_meals
        .entry("Soy Sauce")
        .or_insert(vec!["Sushi", "Dumplings"]);

    println!("3 {:#?}", sources_to_meals);
}
