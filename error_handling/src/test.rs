/// Error handling is the process of dealing with potential errors from
/// operations that can go wrong.
/// Recoverable errors are errors that we can define code to handle
/// Unrecoverable errors are errors that cause the program to be unable to proceed.
///
/// The backtrace is the list of files and functions that were running at the point that
/// the error occurred.
#[test]
fn the_panic_macro() {
    None.unwrap()
}
use std::process;
#[test]
fn the_process_module_and_the_exit_function() {
    // This will exit and terminate the program.
    //zero is gracefully exit the program without issue
    // all other number but 0 is error code. example is 1
    process::exit(1);
    println!("this will not be printed");
}
#[test]
fn standard_error_eprintln_macro() {
    println!("Some status update");
    //print the standard error !
    // we can choose which channel to send the message to
    eprintln!("Some error message"); // print in diff channel !
    // Run cargo test standard_error_eprintln_macro --show-output to see the output of the test
    // cargo run > example.log
}
use std::fs::File;
use std::io::Read;

#[test]
fn opening_a_file() {
    let mut file = File::open("example.log");
    let mut content = String::new();
    match &mut file {
        Ok(file) => {
            println!("{:#?}", &file);
            file.read_to_string(&mut content)
                .expect("Failed to read file");
        }
        Err(error) => {
            println!("{:#?}", error)
        }
    }
    println!("this will be printed: {:?}", content);
    println!("this is file {:#?}", file)
}
use std::io::stdin;
#[test]
fn asking_the_user_for_input() {
    println!("Please enter the name of the file you'd like to read: ");
    let mut input = String::new();
    let user_request_file = stdin().read_line(&mut input);
    if let Err(error) = user_request_file {
        eprintln!("Error reading user request file: {}", error);
        process::exit(1);
    }
    /*    match stdin().read_line(&mut String::new()){
        Ok(size) => println!("The size of the user in bytes is {}", size),
        Err(error) => {
            eprintln!("Something went wrong collection user input. The error  was: {}", error);
            process::exit(1);
        }
    };*/

    let file = match File::open(&input) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Failed to open file: {}", error);
            process::exit(1);
        }
    };
}
#[test]
fn reading_the_files_contents() {
    //work in main.rs
}

#[test]
fn the_question_mark_operator_the_try_operator() {

}
#[test]
fn using_question_mark_with_option(){
    let mut animals = vec!["Dog", "Cat", "Bird"];
    println!("{:?}", length_of_the_last_element(&mut animals));
    println!("{:?}", length_of_the_last_element(&mut animals));
    println!("{:?}", length_of_the_last_element(&mut animals));
    println!("{:?}", length_of_the_last_element(&mut animals));
    println!("{:?}", length_of_the_last_element(&mut animals));

}
fn length_of_the_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last_element = input.pop()?;
    Some(last_element.len())

}