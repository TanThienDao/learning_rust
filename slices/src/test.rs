use std::hint::black_box;

#[test]
fn create_a_string_slice_from_a_string() {
    let action_hero = String::from("Batman Superman");
    let batman = &action_hero[0..=6];  //slice references from char 0 to 4
    println!("slices reference string Batman from 0...=4 ->{}", batman);

    let superman = &action_hero[7..];
    println!("slices reference superman from 7..  ->{}", superman);
}

#[test]
fn string_slice_and_string_literals(){
    let action_hero = "Batman Superman"; //string literal is already a string slice
    let batman = &action_hero[0..=6];  //slice references from char 0 to 4
    println!("slices reference string Batman from 0...=4 -> {}", batman);

    let superman = &action_hero[7..];
    println!("slices reference superman from 7..  -> {}", superman);
}
#[test]
fn string_slice_and_string_literals_2(){
    let first_name = {
        let action_hero = "Batman Superman";
        &action_hero[0..=6]
    };
    println!("first name is {}", first_name);
}
/// The length of a string slice refers to a count of its bytes, not its characters.
#[test]
fn string_slice_length(){
    let food = "pizza 🍕";
    println!("food is {}", food);
    println!("length is {}", food.len());
    let  pizza_slice = &food[0..3];
    println!("pizza slice is {}", pizza_slice);

    println!("memory of food is {:p}",food);
    println!("memory of pizza_slice is {:p}",pizza_slice);
}

/// slice
#[test]
fn syntactic_shortcuts(){
    let action_hero = "Batman Superman";
    let first_hero = &action_hero[..=6];
    println!("first hero is {}", first_hero);
    let secound_hero = &action_hero[7..];
    println!("secound hero is {}", secound_hero);

    let full_hero = &action_hero[..];
    println!("full hero is {}", full_hero);
}
#[test]
fn string_slices_as_function_parameter() {
    let action_hero = String::from("Batman Superman");
    do_hero_stuff(&action_hero);
    let another_action_hero = "WonderWoman";
    do_hero_stuff(&another_action_hero);
}
fn do_hero_stuff(hero_name: &str) {
    println!("{} is doing hero stuff!", hero_name);

}

#[test]
fn array_slice() {
    let value = [4,8,15,16,23,42];
    let my_slice = &value[0..3];
    println!("my_slice is {:?}", my_slice);
    let my_slices = &value[3..];
    println!("my_slices is {:?}", my_slices);
    let my_slices = &value[..5];
    println!("my_slices is {:?}", my_slices);
    let my_slices = &value[..];
    println!("my_slices is {:?}", my_slices);
    let my_slices = &value;
    println!("my_slices is {:?}", my_slices);
}
#[test]
fn deref_coercion_with_array_slices () {
    let value:[i32;6] = [4,8,15,16,23,42];
    let regular_reference = &value;
    print_length(regular_reference);
    let slice_of_three = &value[..3];
    print_length(slice_of_three);

}
fn print_length(reference: &[i32]) {
    println!("length is {}", reference.len());
}
#[test]
fn mutable_array_slices() {
    let mut my_array = [10,15,20,25,30];
    println!("my_array is {:?}", my_array);
    let my_slice = &mut my_array[2..4];
    println!("my_slice is {:?}", my_slice);
    my_slice[0] = 100;
    println!("my_slice is {:?}", my_slice);
    println!("my_array is {:?}", my_array);
}

#[test]
fn project_section_8 () {
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted Flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch")
    ];

    let first_two = &cereals[0..2];
    println!("first_two is {:?}", first_two);
    let mid_three = &cereals[1..=3];
    println!("mid_three is {:?}", mid_three);
    let last_three = &mut cereals[2..];
    println!("last_three is {:?}", last_three);
    last_three[last_three.len() -1] = String::from("Lucky Charms");
    println!("last_three is {:?}", last_three);

    let cookie_crisp = &cereals[0];
    println!("cookie_crisp is {}", cookie_crisp);
    let cookie = &cookie_crisp[0..=5];
    println!("cookie_crisp is {}", cookie);

    let cocoa_puffs = &cereals[3];
    let puffs = &cocoa_puffs[6..];
    println!("cocoa_puffs is {:?}", cocoa_puffs);
    println!("puffs is {:?}", puffs);


}
