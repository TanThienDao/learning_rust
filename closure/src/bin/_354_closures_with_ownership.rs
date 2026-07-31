fn main() {
    let number = 13;
    let capture_number = || number; //This is impl Fn()
    let a = capture_number();
    let b = capture_number();
    println!("{} {} {}", a, b, number);

    let first_name = String::from("Alice");
    let capture_string = || first_name; // This is impl FnMut()
    let owner = capture_string();
    //capture_string(); // This will give error because first_name ownership is moved into the closure and we can't use it again.
    //capture_string();
    //println!("{}", first_name); // first_name ownership is moved into the closure and we can't use it again.
}
