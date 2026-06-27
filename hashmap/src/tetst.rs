/// A hash map is a collection type that
/// consists of key-value pairs.
use std::collections::{HashMap, HashSet};
#[test]
fn create_a_hashmap_with_new_functions() {
    let mut menu: HashMap<String, f64> = HashMap::new();
    menu.insert(String::from("Steak"), 8.99);
    menu.insert(String::from("Tuna"), 4.2);
    menu.insert(String::from("Burger"), 10.0);
    println!("{:#?}", menu);
    let mut country_capitals = HashMap::<&str, &str>::new(); //Turbo fish syntax
    country_capitals.insert("USA", "Washington DC");
    country_capitals.insert("France", "Paris");
    println!("{:#?}", country_capitals);
}
/// The remove method deletes a key-value by using the key
#[test]
fn the_remove_method() {
    let data = [("Bobby", 3), ("Sally", 5), ("Tommy", 7)];
    let mut year_at_comp = HashMap::from(data);
    println!("{:#?}", year_at_comp);
    let bobby = year_at_comp.remove("Bobby");
    println!("{:#?}", year_at_comp);
    println!("{:#?}", bobby);
    println!("{}", bobby.unwrap());
}
fn add(num1: &i32, num2: &i32) -> i32 {
    num1 + num2
}
#[test]
fn it_works() -> Result<(), String> {
    let result = add(&2, &2);
    if result == 4 {
        Ok(())
    } else {
        Err(format!("Expected 4 but got {}", result))
    }
}

#[test]
fn hash_maps_and_ownership() {
    let mut cofee_pairings = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    /// at this point the hashmap owns the drink and milk variables, so we can no longer use them after this point
    cofee_pairings.insert(&drink, &milk);
    //cofee_pairings.insert("Flat White", "Almond Milk");
    println!("{:#?}", cofee_pairings);
    println!("Length of the hash map: {:#?}", cofee_pairings.len());
    println!("{} {}", drink, milk);
}
#[test]
fn access_a_value_by_key() {
    let mut cofee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    /// at this point the hashmap owns the drink and milk variables, so we can no longer use them after this point
    cofee_pairings.insert(&drink, &milk);
    cofee_pairings.insert("Flat White", "Almond Milk");
    let invalid_value = cofee_pairings["Flat White"];
    println!("{:#?}", invalid_value);
    let value = cofee_pairings.get("Flat White");
    println!("{:#?}", value);

    let none_exit_value = cofee_pairings.get("NoneExit");
    println!("None exit value with get{:#?}", none_exit_value);

    //let invalid_none_exit_value = cofee_pairings["NoneNotExit"];
    //println!("None exit value  {}", invalid_none_exit_value);

    let copy_value = cofee_pairings
        .get("Flat aas")
        .copied()
        .unwrap_or("None value for default");
    println!("Copy value: {:#?}", copy_value);
}

#[test]
fn overwriting_a_value_with_an_exiting_key() {
    let mut cofee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    /// at this point the hashmap owns the drink and milk variables, so we can no longer use them after this point
    cofee_pairings.insert(&drink, &milk);
    cofee_pairings.insert("Flat White", "Almond Milk");

    cofee_pairings.insert("Latte", "Pistachio Milk");
    cofee_pairings.entry("Flat White").or_insert(&milk);
    cofee_pairings
        .entry("None_exit_key")
        .or_insert("Default Value");
    println!("{:#?}", cofee_pairings);
}
#[test]
fn the_entry_method() {
    let mut cofee_pairings: HashMap<&str, &str> = HashMap::new();
    let drink = String::from("Latte");
    let milk = String::from("Oat Milk");
    /// at this point the hashmap owns the drink and milk variables, so we can no longer use them after this point
    cofee_pairings.insert(&drink, &milk);
    cofee_pairings.insert("Flat White", "Almond Milk");

    cofee_pairings.entry("Latte").or_insert("Pistachio Milk");
    println!("{:#?}", cofee_pairings);

    cofee_pairings
        .entry("Cappuccino")
        .or_insert("Pistachio Milk");
    println!("{:#?}", cofee_pairings);
}
/// A hash set is a collection type that stores unique values.
#[test]
fn the_hash_set() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    println!("{:#?}", concert_queue);
    concert_queue.insert("Molly");
    concert_queue.insert("Megan");
    println!("{:#?}", concert_queue);
    println!("{:#?}", concert_queue.len());
    concert_queue.insert("Molly");
    println!("{:#?}", concert_queue);
    concert_queue.insert("Megan");
    println!("{:#?}", concert_queue);
    println!("{:#?}", concert_queue.len());
    println!("{:#?}", concert_queue.remove("Molly"));
    println!("{:#?}", concert_queue.len());
    println!("{:#?}", concert_queue.remove("Franny"));
    println!("{:#?}", concert_queue.len());

    println!("{:#?}", concert_queue.contains("Megan"));
    println!("{:#?}", concert_queue.capacity());
    println!("{:#?}", concert_queue.get("Megan"));
    println!("{:#?}", concert_queue.len());
}
#[test]
fn hashset_operations() {
    let mut concert_queue: HashSet<&str> = HashSet::new();
    let mut movie_queue: HashSet<&str> = HashSet::new();
    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");

    movie_queue.insert("Boris");
    movie_queue.insert("Phil");

    /// Union in rust HashSet
    println!("Union {:#?}", concert_queue.union(&movie_queue));

    /// Different method is going to giive you the values that are found in the first set, which is the
    /// one that the method is invoked upon but not found in the secound set. +
    println!("different {:#?}", concert_queue.difference(&movie_queue));

    ///Symmetric different is going to do is give you the values that are in either
    /// one of the sets but not both.
    println!(
        "symmetric different {:#?}",
        concert_queue.symmetric_difference(&movie_queue)
    );

    /// is_disjoint this method reutnr true if the sets have no elements in common, and false if they do have elements in common.
    println!("is disjoint {:#?}", concert_queue.is_disjoint(&movie_queue));

    /// is_subset this method returns true if all the elements of the first set are also in the second set, and false if they are not.
    println!("is subset {:#?}", concert_queue.is_subset(&movie_queue));
    let mut attendees: HashSet<&str> = HashSet::new();
    attendees.insert("Boris");
    println!("is subset {:#?}", attendees.is_subset(&concert_queue));

    /// is_superset this method returns true if all the elements of the second set are also in the first set, and false if they are not.
    println!("is superset {:#?}", concert_queue.is_superset(&attendees));
}
