use log::error;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Error, Read};

#[test]
fn test_error_handling() {
    let greeting_file_result = File::open("hello.txt");
    println!("greeting files begin -> {:?}", greeting_file_result);
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
    println!("greeting file after -> {:?}", greeting_file);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Err(Error::new(ErrorKind::Other, "Some error"))
}
fn read_username_from_file_4() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
fn read_username_from_file_5() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}
#[test]
fn test_unwrap() {
    let greeting_file_result = File::open("hello.txt");
    println!("greeting files begin -> {:?}", greeting_file_result);
}

#[test]
fn test_expect() {
    let greeting_file_result = File::open("hello.txt").expect("Failed to open hello.txt");
    println!("greeting files begin -> {:?}", greeting_file_result);
}
#[test]
fn test_username_from_file() {
    let username_result = read_username_from_file();
    println!("username result -> {:?}", username_result);
}
#[test]
fn test_username_from_file_2() {
    let username_result = read_username_from_file_2();
    println!("username result 2 -> {:?}", username_result);
}
#[test]
fn test_username_from_file_3() {
    let username_result = read_username_from_file_3();
    println!("username result 3 -> {:?}", username_result);
}

#[test]
fn test_username_from_file_4() {
    let username_result = read_username_from_file_4();
    println!("username result 4 -> {:?}", username_result);
}
#[test]
fn test_username_from_file_5() {
    let username_result = read_username_from_file_5();
    println!("username result 5 -> {:?}", username_result);
}

fn last_char_of_first_line(text: &String) -> Option<char> {
    text.lines().next()?.chars().last()
}
#[test]
fn test_last_char_of_first_line() {
    println!(
        "last char of the first line : {:?}",
        last_char_of_first_line(&String::from("\nhi"))
    );
    println!(
        "last char of the first line : {:?}",
        last_char_of_first_line(&String::from("hi"))
    );
}
#[test]
fn test_dangling_references() {
    let r;

    {
        let x = 5;
        r = x;
    }

    println!("r: {r}");
}
//===============================================================================================================================================================
/// The Option enum models a scenario where a type
/// could be a valid value or nothing at all.
#[test]
fn the_option_enum() {
    let a = Option::Some(5);
    let b = Option::Some("hello");
    let c = Option::Some(true);
    let a: Option<i8> = Option::Some(5);
    let a = Option::<i16>::Some(5); //turbofish operator ?

    let d: Option<&str> = Option::None; // only store string slice.
}
#[test]
fn real_example_of_option_enum() {
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instrument.get(2); // return an option ref String
    println!("bass is {:?}", bass); //bass is Some("Bass")
    let invalid_instrument = musical_instrument.get(100);
    println!("invalid_instrument is {:?}", invalid_instrument);
}
/// The upwrap method attempts to extract the associated data out of the Some variant
#[test]
fn the_unwrap_and_expect_methods() {
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instrument.get(2); // return an option ref String
    println!("bass is {:?}", bass); //bass is Some("Bass")
    let valid_instrument = bass.unwrap();
    println!("valid_instrument is {}", valid_instrument);
    let valid_instrument_2 = bass.expect("Unable to retrieve the instrument");
    println!("valid_instrument_2 is {}", valid_instrument_2);

    let invalid_instrument = musical_instrument.get(100);
    println!("invalid_instrument is {:?}", invalid_instrument);
    //invalid_instrument.unwrap(); // will panic because there are nothing to unwrap.
    invalid_instrument.expect("Unable to retrieve the invalid instrument");
}
fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Some(instrument_string) => {
            println!("instrument_string is {}", instrument_string);
        }
        None => {
            println!("No instrument specified");
        }
    }
}
#[test]
fn the_match_keyword_with_option_enum() {
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Drum"),
        String::from("Bass"),
    ];

    let bass: Option<&String> = musical_instrument.get(2);
    match bass {
        Option::Some(instrument) => println!("The instrument is {}", instrument),
        Option::None => println!("No instrument found"),
    }

    let invalid_instrument = musical_instrument.get(100);
    match invalid_instrument {
        Option::Some(instrument) => println!("The instrument is {}", instrument),
        Option::None => println!("No instrument found"),
    }

    play(bass);
    play(invalid_instrument);
}
fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool> {
    if (item_is_in_system && item_is_in_stock) {
        Some(true)
    } else if (item_is_in_system || item_is_in_stock) {
        Some(false)
    } else {
        Option::None
    }
}
#[test]
fn returning_an_option_enum_from_a_function() {
    let availabolity = is_item_in_stock(false, true);
    println!("availabolity is {:?}", availabolity);
    match availabolity {
        Option::Some(value) => println!("availabolity is {}", value),
        Option::None => println!("No instrument found"),
        _ => println!("Invalid value"),
    }
}

/// The Rust Prelude is a collection of named constructs
/// that are available automatically in every program.
#[test]
fn top_level_option_variants() {
    // nothing to code here,
    // lecture about the Option is available and no need to wirete out.
}
#[test]
fn the_unwrap_or_method() {
    let present_value = Option::Some(13);
    let missing_value: Option<i32> = Option::None;

    println!("present_value is {:?}", present_value.unwrap());
    println!("present_value is {:?}", present_value.unwrap_or(0));
    //println!("missing_value is {:?}", missing_value.unwrap());
    println!("missing_value is {:?}", missing_value.unwrap_or(0));
}
#[derive(Debug, Copy, Clone)] //todo need to understand what is Copy and Clone trait here
enum MyOption {
    Some(i32),
    None,
}
impl MyOption {
    fn unwrap(self) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Should have been MyOption::None"),
        }
    }
    fn unwrap_or(self, default: i32) -> i32 {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => default,
        }
    }
}
#[test]
fn building_option_from_scratch() {
    let some_option = MyOption::Some(13);
    println!("some_option is {:?}", some_option.unwrap());

    let none_option = MyOption::None;
    println!("none_option is {:?}", none_option.unwrap_or(1));
    println!("some_option is {:?}", none_option.unwrap());
}
/// The result enum models the outcome of an evaluation that can produce either a success or an error.
#[test]
fn the_result_enaum() {
    let ok: Result<i32, &str> = Ok(5);
    println!("ok is {:?}", ok);
    let dissaster: Result<i32, &str> = Result::Err("Something went terribly wrong");
    println!("dissaster is {:?}", dissaster);
}
#[test]
fn real_example_of_result_enum() {
    let text = "50";
    let text_as_number = text.parse::<i32>();
    println!("text_as_number is {:?}", text_as_number);
    //text_as_number is Ok(50)

    let text = "Alabama";
    let text_as_number = text.parse::<i32>();
    println!("text_as_number is {:?}", text_as_number);
    //text_as_number is Err(ParseIntError { kind: InvalidDigit })
}
fn divide(numerator: f64, demoninator: f64) -> Result<f64, String> {
    if demoninator == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(numerator / demoninator)
    }
}
#[test]
fn returning_a_result_enum_from_a_function() {
    let result = divide(5.0, 0.0);
    println!("Result is {:?}", result);
    match result {
        Ok(calculation) => println!("Result is {:?}", calculation),
        Err(message) => println!("Result is {:?}", message),
    }
}
#[test]
fn result_method() {
    let result = divide(10.0, 2.0);
    println!("Result 1 is {:?}", result.unwrap());

    let result2 = divide(10.0, 0.0);
    println!("Result 3 is {:?}", result2.is_ok());
    println!("Result 2 is {:?}", result2.is_err());
    let result3 = result2.clone();
    println!("Result 3 is {:?}", result3.unwrap_or(0.0));
    println!(
        "Result 2 is {:?}",
        result2.expect("Failed to divide by zero")
    );
}
fn operation(greate_success: bool) -> Result<&'static str, &'static str> {
    if greate_success {
        Ok("success")
    } else {
        Err("Operation failed")
    }
}
#[test]
fn nuances_of_unwrap_method_on_result() {
    let my_result: Result<&str, &str> = operation(true);
    let content = match my_result {
        Ok(message) => message,
        Err(error) => error,
    };
    println!("my_result is {:?}", my_result.unwrap());
}
#[test]
fn the_while_let_construct() {
    let mut sources = vec!["Mayonnaise", "Ketchup", "Ranch"];
    while let Some(source) = sources.pop() {
        println!("source is {}", source);
    }
}
#[derive(Debug)]
struct Food {
    name: String,
}
#[derive(Debug)]
struct Restaurant {
    reservations: u32,
    has_mice_infestation: bool,
}
impl Restaurant {
    fn chef_special(&self) -> Option<Food> {
        if (self.has_mice_infestation) {
            return None;
        }
        if (self.reservations < 12) {
            Some(Food {
                name: String::from("Uni Sashimi"),
            })
        } else {
            Some(Food {
                name: String::from("Ribeye Steak"),
            })
        }
    }
    fn deliver_burger(&self, address: &str) -> Result<Food, String> {
        if self.has_mice_infestation {
            return Err("Sorry, we have a mice problem".to_string());
        }
        if address.is_empty() {
            return Err("No delivery address specified".to_string());
        }
        Ok(Food {
            name: String::from("Burger"),
        })
    }
}
#[test]
fn project_section_12() {
    let restaurant_1 = Restaurant {
        reservations: 11,
        has_mice_infestation: true,
    };
    let rs_sp_1 = restaurant_1.chef_special();
    println!("rs_sp_1 is {:?}", rs_sp_1);
    println!("{:?}", restaurant_1.deliver_burger("123 Elm Street"));

    let restaurant_2 = Restaurant {
        reservations: 15,
        has_mice_infestation: false,
    };
    let rs_sp_2 = restaurant_2.chef_special();
    println!("rs_sp_2 is {:?}", rs_sp_2);
    println!("{:?}", restaurant_2.deliver_burger(""));
    println!("{:?}", restaurant_2.deliver_burger("valid address"));
}
