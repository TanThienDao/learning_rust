fn main() {
    let my_vector = vec![4, 8, 15, 16, 23, 42];
    for value in &my_vector {
        println!("value: {}", value);
    }
    println!("Is the iterator exhausted? {:?}", &my_vector);
}
