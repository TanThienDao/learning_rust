fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter(); // Iterator does not need to be mutable here,
    // because we are not calling next() directly, but using a for loop which handles the iteration internally.
    //warning: `iterators` (bin "_371_why_iterator_can_be_immutable") generated 1 warning (run `cargo fix --bin "_371_why_iterator_can_be_immutable" -p iterators` to apply 1 suggestion)
    for value in my_iterator {
        println!("value: {}", value);
    }
}
