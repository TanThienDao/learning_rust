fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    let mut my_iterator = my_vector.into_iter();

    while let Some(value) = my_iterator.next() {
        println!("value: {}", value);
        println!("current my_iterator state: {:?}", my_iterator);
    }
    println!("Is the iterator exhausted? {:?}", my_iterator.next());
    //println!( "Vector {:?}", my_vector);

    println!(
        "Is the iterator exhausted? {:?}",
        my_iterator.next().is_none()
    );
    println!("what in site my_iterator ? ->  {:?}", my_iterator);
}
