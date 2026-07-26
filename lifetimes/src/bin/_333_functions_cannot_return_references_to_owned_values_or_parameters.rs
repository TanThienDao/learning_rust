fn create_slice(item: Vec<i32>) -> &[i32] {
    &item[0]
}
fn main() {
    let my_ref = create_slice(vec![1, 2, 3]);
}
