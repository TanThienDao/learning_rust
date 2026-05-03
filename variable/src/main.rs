use std::io;

#[cfg(test)]
mod test;
mod test2;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let a: char = 'a';
    let b: bool = true;
    let c: i64 = 255;
    let d: f64 = 1.0;
    let e: i32 = 5;
    let f: u8 = 255;
    println!("The value of a, b, c is: {}, {}, {}", a, b, c);

    let tup: (i32, f64, u8) = (e, d, f);

    println!("The value of tup is: {}", tup.0);

    let tup2 = (1, 2, 3);
    let (_x, y, z) = tup2;
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!(" array 0 : {}", array[0]);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    another_function("test".to_string());
    print_labeled_measurement(5, 'm');

    let five = five();
    println!("The value of five is: {}", five);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            counter = counter * 2;
            break counter;
        }
    };

    println!("The result is {}", result);

    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);

    let _s1 = String::from("hello");
    /*    let s2 = s1;
    println!("s1 = {}, s2 = {}", s1, s2);*/
}

/**
 Description:

# Argument:
    - 'input' : String
    -

 TODO need to do
*/
fn another_function(x: String) {
    println!("Another function {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    -5
}
