fn main() {
    let first_name = String::from("Alice");
    let last_name = String::from("Bob");
    let capture_string = move || println!("{first_name} {last_name}");
    capture_string(); // ownership of first_name and last_name is moved into the closure but not consume so we can able to call it multiple times.
    capture_string();
    capture_string();

    //println!("{} {}", first_name, last_name);
}
