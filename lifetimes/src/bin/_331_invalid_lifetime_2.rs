/// A dangling references is a reference to data that no longer exits
fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New  York"),
        String::from("Tokyo"),
    ];

    //let places = cities;
    let favorite_cities = &cities[0..2]; // citi is anotherr dangling reference since life time of city is already end at line 9
    //drop(cities);
    println!("{favorite_cities:?}");
    let places = cities;
    //println!("{favorite_cities:?}");// lexical scope of city is end at line 12 so this is adnalging references
}
