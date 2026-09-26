use colored::Colorize;
use std::io::{self, Write};
/*fn main() {
    let word = "trait";
    let input = io::stdin();

    for i in 1..=6 {
        println!("Attempt {}/6: Enter your guess:", i);
        let mut guess = String::new();
        input.read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim();

        if guess.len() != word.len() {
            println!("Please enter a {}-letter word.", word.len());
            continue;
        }

        let mut feedback = String::new();
        for (g_char, w_char) in guess.chars().zip(word.chars()) {
            if g_char == w_char {
                feedback.push_str(&g_char.to_string().green().to_string());
            } else if word.contains(g_char) {
                feedback.push_str(&g_char.to_string().yellow().to_string());
            } else {
                feedback.push_str(&g_char.to_string().red().to_string());
            }
        }
        println!("{}", feedback);

        if guess == word {
            println!("Congratulations! You've guessed the word!");
            return;
        }
    }
}*/

fn main() {
    let word = "trait";
    let input = io::stdin();
    for i in 1..=6 {
        let mut user_input = String::new();
        println!("Enter your guess (5 letters): {}", i);
        input
            .read_line(&mut user_input)
            .expect("Failed to read line");
        for (word_char, user_char) in word.chars().zip(user_input.trim().chars().take(5)) {
            if word_char == user_char {
                print!("{}|", format!("{}", user_char).on_green());
            } else if word.contains(user_char) {
                print!("{}|", format!("{}", user_char).on_yellow());
            } else {
                print!("{}|", format!("{}", user_char).on_black());
            }
            io::stdout().flush().unwrap();
        }
        println!();
        if user_input.trim() == word {
            println!("Congratulations! You've guessed the word! {}", word.green());
            return;
        }
    }
}
