/// Elision is the act of omitting something.
/// Lifetime elision means omitting generic lifetime
/// annotations in situations where the borrow checker can infer the lifetime
/// relationships automatically.
///
/// First Elision rule: the compiler assigns a lifetime to each parameter that is a reference.
///
/// Second Elision rule: if there is one reference parameter and the return value is reference,
/// the borrow checker will infer that their lifetimes are related.

fn my_awsome_funtion<'a, 'b>(value: &'a i32, secound: &'b i32) -> &'a i32 {
    value
}

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
