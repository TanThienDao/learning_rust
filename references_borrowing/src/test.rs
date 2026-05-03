


#[test]
fn immutable_and_mutable_reference(){
    let mut current_meal = String::new();
    /// need to add &mut to make it mutable reference, otherwise it will be immutable reference and cannot modify the meal
    add_flour(&mut current_meal);
    println!("{}", current_meal);
    show_my_meal(&current_meal);
}
/// meal : String  take full ownership
/// mut meal : String  the meal take ownership and has permission to modify.
/// meal: &String take reference of the meal but not ownership, and cannot modify the meal
/// meal: &mut String take mutable reference of the meal, and can modify the meal but not take ownership
fn add_flour(meal: &mut String)  {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String)  {
    println!("Meal steps: {}", meal);
}

#[test]
fn multiple_immutable_reference(){
    let car = String::from("Car");
    let ref1 = &car;
    let ref2 = &car;
    println!("ref1: {}, ref2 : {}, and {}", ref1,ref2,&car);

}

#[test]
/**
    when we have a mutable var we can. only have 1 reference of a time not multiple like immutable references.
*/
fn mutable_reference_restructions() {
    let mut car = String::from("Car");
    let ref1 = &mut car;
    let ref2 = &car;  /// can not have a second reference for a mutable var
    //ref1.push_str("s ");  //if multiple here ref 2 will have problem
    println!("{}" ,ref2);       // but if ref1 did not used some how compile is success,
}
#[test]
fn ownership_with_immutable_and_mutable_references(){
    let mut coffee = String::from("Coffee");
    let a = &mut coffee;
    println!("{}", a);
    let b = a;
    println!("{}", b);
    b.push_str(" is hot");
    println!("{}", b);
}
/// A dangling references is a pointer to a memory address that has been deallocated  
#[test]
fn dangling_references (){
    let city = create_city();
    println!("{}", city);

}
/// this is a dangling reference that return a reference that no longer exit.
/*fn create_city() -> &String {
    let city = String::from("New York City");
    &city
}*/
fn create_city() -> String {
    let city = String::from("New York City");
    city
}

#[test]
fn ownership_with_arrays_and_tuplets() {
    let registrations: [bool; 3] = [true, false, true];
    let first = registrations[0];
    println!("First registration: {}, and registrations {:?}", first, registrations);

    let language = [String::from("en"), String::from("fr"), String::from("nl"), String::from("de")];
    let first = language[0].clone();  // This is duplicate the memory in the heap
    let second = &language[1];      // borrow the memory reference in the array

    let registration_tuple: (bool, bool, bool) = (true, false, true);
    let registration_first = registration_tuple.0;
    println!("First registration: {}, and registration tuple {:?}", registration_first, registration_tuple);

    let language_tuple = (String::from("en"), String::from("fr"), String::from("nl"), String::from("de"));
    let first_language_tuple = language_tuple.0.clone();
    let second_language_tuple = &language_tuple.1;
    println!("First language: {}", first_language_tuple);
    println!("language tuples: {:?}", language_tuple);
}

#[test]
fn project_section_7(){
    let mut trip = start_trip();
    visit_philadelphia(&mut trip);
    trip.push_str(" and ");
    visit_new_york(&mut trip);
    trip.push_str(" and ");
    visit_boston(&mut trip);
    println!("{}", trip);
    show_itinerary(&trip);
}

fn start_trip() -> String{
    String::from("The plan is ....")
}

fn visit_philadelphia ( text :&mut String){
    text.push_str(" Philadelphia");
}
fn visit_new_york ( text :&mut String ){
    text.push_str(" New York");
}
fn visit_boston ( text :&mut String ){
    text.push_str(" Boston");
}
fn show_itinerary(text: &String){
    println!("The trip itinerary is : {}", text);

}

