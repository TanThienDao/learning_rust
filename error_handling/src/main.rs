use std::fs::{self, File};
use std::io::{BufReader, Error, Read, stdin};
use std::process;
use std::fmt::{self, Error as FmtError, Formatter};

mod test;
mod main_2;

fn main() {
    // 279 Propagating Errors
    let file_result = read_file_3();
    match file_result {
        Ok(contents) => {
            println!("File contents: {}", contents)
        }
        Err(error) => {
            eprintln!("Error reading file: {:#?}", error);
            process::exit(1);
        }
    }
    //280 Understand the error type Redeclaration
    // explaination of Result type in Error module.

}
// 279 Propagating Errors
fn read_file() -> Result<String, Error> {
    //277 Asking the user input
    println!("Please enter the name of the file you'd like to read: ");
    let mut input = String::new();
    let user_request_file = stdin().read_line(&mut input);
    if let Err(error) = user_request_file {
        /*eprintln!("Error reading user request file: {}", error);
        process::exit(1)*/
        return Err(error);
    }
    /*    match stdin().read_line(&mut String::new()){
        Ok(size) => println!("The size of the user in bytes is {}", size),
        Err(error) => {
            eprintln!("Something went wrong collection user input. The error  was: {}", error);
            process::exit(1);
        }
    };*/

    let mut file = match File::open(&input.trim()) {
        Ok(file) => file,
        Err(error) => {
            /*eprintln!("Failed to open file: {}", error);
            process::exit(1);*/
            return Err(error);
        }
    };
    // 278 Reading the file's content
    let mut content = String::new();
    let read_operator = file.read_to_string(&mut content);
    if let Err(error) = read_operator {
        /*eprintln!("Error reading operator: {}", error);
        process::exit(1);*/
        return Err(error);
    }
    println!("Content file: {:?}", content);
    Ok(content)
}

//281 the ? operator (the try operator)
fn read_file_2() -> Result<String, Error> {
    //277 Asking the user input
    println!("Please enter the name of the file you'd like to read: ");
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let mut content = String::new();
    File::open(&input.trim())?.read_to_string(&mut content)?;

    println!("Content file: {:?}", content);
    Ok(content)
}

// 282 the read_to_string associated function
fn read_file_3() -> Result<String, Error> {
    //277 Asking the user input
    println!("Please enter the name of the file you'd like to read: ");
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    fs::read_to_string(input.trim())
}