fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}
fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Tokyo"),
    ];
    let two_cities = {
        let cities_references = &cities;
        select_first_two_elements(&cities_references)
    };
    println!("{two_cities:?}");

    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let two_coffees = select_first_two_elements(&coffees);
        println!("{:#?}", two_coffees)
    }
}
