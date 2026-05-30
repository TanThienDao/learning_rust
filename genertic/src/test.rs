/*
A generic is a type argument. it's a placeholder for a concrete type.
*/
fn add_5(value: i32) -> i32 {
    value + 5
}

fn identity_i32(value: i32) -> i32 {
    value
}
fn identity_bool(value: bool) -> bool {
    value
}
/// Generic type T
/// In Rust, Monimorphization is a compile-time process where polymorphic functions
/// are replace by many monomorphic functions for each unique instantiation.
fn identity<T>(value: T) -> T {
    value
}
#[derive(Debug)]
struct DeliSandwich;
#[test]
fn intro_to_generics(){
    println!("{}, {}, {}", add_5(10), identity_i32(10), identity_bool(true));
    println!("{} ", identity(5));
    println!("{} ", identity(13.25));
    println!("{} ", identity("string slice"));
    println!("{} ", identity(String::from("heap-allocated, growable string")));
    println!("{:?} ", identity(DeliSandwich));
}
#[test]
fn the_tubofish_operator(){
    println!("{} ", identity::<i8>(5));
    println!("{} ", identity::<i32>(13));
    println!("{} ", identity::<f32>(13.25));
    println!("{} ", identity::<String>(String::from("string slice")));
    println!("{}", identity::<bool>(true));
    println!("{:#?}", identity::<DeliSandwich>(DeliSandwich{}));
}
fn make_tuple<T>(first: T, secound: i32) -> (T, i32){
    (first, secound)
}
fn make_tuple_2<T,U>(first: T, secound: U) -> (T, U){
    (first, secound)
}
#[test]
fn multiple_generics(){
    make_tuple("hello",5);
    make_tuple_2("hello", 5);
}
#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treasure: T,
}
impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string();
    }
}
impl TreasureChest<[&str; 3]> {
    fn list_treasure(&self) {
        for item in &self.treasure {
            println!("Treasure item: {}", item);
        }
    }
    fn amount_of_treasure(&self) -> usize{
        self.treasure.len()
    }

}
impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String{
        self.captain.to_uppercase()
    }
}
#[test]
fn generics_in_structs(){
    let gold_chest = TreasureChest {
        captain: String::from("Blackbeard"),
        treasure: "gold",
    };
    let silver_chest = TreasureChest {
        captain: String::from("Anne Bonny"),
        treasure: String::from("Silver"),
    };
    let special_chest = TreasureChest {
        captain: String::from("Jack Sparrow"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("{:#?} ", gold_chest);
    println!("{:#?} ", special_chest);
}
#[test]
fn generics_and_impl_blocks_I(){
    let mut silver_chest = TreasureChest {
        captain: String::from("Anne Bonny"),
        treasure: String::from("   Silver     "),
    };
    silver_chest.clean_treasure();
    println!("{:#?} ", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Jack Sparrow"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    special_chest.list_treasure();
    println!("{:#?} ", special_chest);
    let size = special_chest.amount_of_treasure();
    println!("size: {}", size);
}
#[test]
fn generics_and_impl_blocks_II(){
    let gold_chest = TreasureChest {
        captain: String::from("Blackbeard"),
        treasure: "gold",
    };
    println!("Captain in uppercase: {}", gold_chest.capital_captain());
    let special_chest = TreasureChest {
        captain: String::from("Jack Sparrow"),
        treasure: ["Gold", "Silver", "Platinum"],
    };
    println!("Captain in uppercase: {}", special_chest.capital_captain());
}
#[derive(Debug)]
enum Cheesesteak<T>{
    Plain,
    Topping(T),
}
#[test]
fn generics_in_enumd(){
    let mushroom = Cheesesteak::Topping("mushroonm");
    let oniion = Cheesesteak::Topping("onionm".to_string());
    let toping = "bacon".to_string();
    let bacon = Cheesesteak::Topping(&toping);
    // let plain = Cheesesteak::Plain; // error: cannot infer type for type parameter `T` declared on the enum `Cheesesteak`
    let mut plain:Cheesesteak<String> = Cheesesteak::Plain;
    //plain = Cheesesteak::Topping("mushroom"); // error cannot assign `Cheesesteak<&str>` to `Cheesesteak<String>`
    plain = Cheesesteak::Topping("mushroom".to_string());
    println!("{:#?} ", mushroom);

}