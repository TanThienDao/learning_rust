/// A review of Generics
///
/// A Generic is a placeholder for a future type.
/// Generics add flexibility by not hardcoding an exact type.
/// Code can use a variety of types in places of the generic.
///
///
/// Generic lifetimes vs Concrete lifetimes
///
/// A concrete lifetime is the region of code that a value exits in
/// the program (the time it lives in its memory address),
///
/// A generic life time is more abstract. It is a hypothetical lifetime,
/// a non-specific life-time, a future lifetime that can vary.
///
/// We can annotate generic lifetimes in our code. This enables functions
/// that are flexibles enough to handle varying lifetimes.
///
///
/// Lifetime Annotations (in order to declare a generic lifetime)
///
/// A lifetime annotation is a name or label for a lifetime.
///
/// Lifetime annotations don't change the reference's lifetime.
/// They don't affect the logic in any way.
///
/// A lifetime annotation is a piece of metadata that we provide to the
/// borrow checcker so that it can validate that references are valid.

fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
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
