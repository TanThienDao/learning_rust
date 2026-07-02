fn main() {
    let mut animals = vec!["Dog", "Cat", "Bird"];
    println!("{:?}", length_of_the_last_element(&mut animals));
}
fn length_of_the_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())
}
