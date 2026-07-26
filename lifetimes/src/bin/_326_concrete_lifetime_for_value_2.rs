/// The lifetime of a value refers to how long it is valid within the code.
/// A value's lifetime is the time during which it exits at a particular memory address.
fn main() {
    let a = 1;
    // A is valid from where it init to the end of the scope

    {
        let b = 2;
    }
    //println!({b}); b life time already end in that scope
    let c = String::from("Winter");
    //let d = c; // the life time of c end when d pick the owner ship of c, c become invalid and no longer alive
    drop(c); // c life time end here because drop create the end of lifetime of c
}
