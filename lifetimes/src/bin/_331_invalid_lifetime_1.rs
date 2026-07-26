/// A dangling references is a reference to data that no longer exits
fn main() {
    let some_city = {
        // dangling references
        let cities = vec![
            String::from("London"),
            String::from("New  York"),
            String::from("Tokyo"),
        ];
        &cities[0..2]
    };

    let cities = vec![
        String::from("London"),
        String::from("New  York"),
        String::from("Tokyo"),
    ];

    let favorite_cities = &cities[0..2];
    //drop(cities);
    println!("{favorite_cities:?}");
}
