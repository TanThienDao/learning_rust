#[test]
fn string_reassignment_works() {
    let mut s = String::from("hello");
    s = String::from("world");

    assert_eq!(s, "world");
}

#[test]
fn five_returns_minus_five() {
    assert_eq!(super::five(), -5);
}
