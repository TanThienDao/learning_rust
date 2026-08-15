use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<&str,u32> {
    let words = text.split_whitespace();
    let mut count = HashMap::new();
    for word in words {
        let counter = count.entry(word).or_insert(0);
        *counter += 1;
    }
    count
}
fn count_char(text: &str) -> HashMap<char,u32> {
    let words = text.split_whitespace();
    let mut count = HashMap::new();
    for word in words {
        word.chars().for_each(|char| *count.entry(char).or_insert(0) += 1);
    }
    count
}

fn count_char_simple(text: &str) -> HashMap<char,u32> {
    let words = text.split_whitespace();
    let mut count = HashMap::new();
    for word in words {
        for c in word.chars() {
           let counter = count.entry(c).or_insert(0);
           *counter += 1;
        }
    }
    count
}


fn main() {
    let text = "Sally sells sea shells by the seashore. The shells that Sally sells are surely seashells.";
    let word_counts = count_words(text);
    println!("{:?}", word_counts);
    let char_counts = count_char(text);
    println!("{:?}", char_counts);
}