use std::io;
use rand::Rng;
use std::cmp::Ordering;
/*
This is a simple guessing game where the user is prompted to input their guess. The program reads the user's input and then prints it back to the console.

*/
fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is: {}", secret_number); // This line is for debugging purposes

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
