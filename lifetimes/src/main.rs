fn double_the_length<T>(vector: &Vec<T>) -> usize {
    vector.len() * 2
}
fn last_two<T>(vector: &[T]) -> &[T] {
    if vector.len() > 2 {
        &vector[vector.len() - 2..]
    } else {
        vector
    }
}

fn first_five<'a>(text: &'a str, announcement: &str) -> &'a str {
    println!("Announcement! {}", announcement);
    &text[..5]
}

fn find_string_that_has_content<'a>(first: &'a str, second: &'a str, target: &str) -> &'a str {
    if (first.contains(target)) {
        first
    } else if (second.contains(target)) {
        second
    } else {
        ""
    }
}

fn main() {
    println!("Hello, world!");
    let test = vec![1, 2, 3, 4, 5];
    let size = double_the_length(&test);
    println!("The double length is: {}", size);
    let test_2 = last_two(&test);
    println!("Last two is: {:?}", test_2);
    let first_five = first_five("refrigerator", "Hello");
    println!("The first five is: {}", first_five);
    let test_has_content = find_string_that_has_content("programming", "dining", "gram");
    println!("The string that has content is: {}", test_has_content);
}
