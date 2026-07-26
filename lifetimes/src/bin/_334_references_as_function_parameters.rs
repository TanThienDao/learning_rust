fn select_first_two_elements(items: &[String]) -> &[String] {
    // this is a slice  collection of string, can take in dynamic collection of type
    &items[..2]
}

fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Tokyo"),
    ];
    let two_cities = select_first_two_elements(&cities);
    println!("{two_cities:?}");
    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{:#?}", two_coffees)
    }
}
