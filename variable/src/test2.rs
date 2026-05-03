#[test]
fn test() {
    let mut s = String::from("hello");
    s = String::from("world");
    println!("check s:  {}", s);
}

#[test]
fn test2() {
    let x = 5;
    let y = x;
    println!("check x: {}, check y: {}", x, y);
    println!("x address {:p}", &x);
    println!("y address {:p}", &y);
}

#[test]
fn test3() {
    let s = String::from("hello");
    take_ownership(s);
    ///println!("check ownership: {}", s);
    /// s has move to take_ownership can print here !
    let x = 5;
    makes_copy(x);
    println!("check move: {}", x);
}

fn take_ownership(s: String) {
    println!("take_ownership: {}", s);
}

fn makes_copy(i: i32) {
    println!("makes_copy: {}", i);
}

#[test]
fn test4() {
    let s1 = give_ownership();
    println!("s1 give_ownership: {}", s1);
    let s2 = String::from("hello");
    println!("s2 give_ownership: {}", s2);
    let s3 = takes_and_gives_back(s2);
    println!("s3 takes_and_gives_back: {}", s3);
}

fn give_ownership() -> String {
    let some_string = String::from("your");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

#[test]
fn test5() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

#[test]
fn test6() {
    let s1 = String::from("hello");
    println!("The length of '{}' is {}.", s1.len(), s1);
}
fn calculate_length_2(s: &String) -> usize {
    s.len()
}

#[test]
fn test7() {
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("The length of '{}' is {}.", s1.len(), s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[test]
fn test8() {
    let mut s1 = String::from("hello");
    let r1 = &mut s1;
    //let r2 = &mut s1; //you can not let 2 borrow the same mutable variable.
    //change(&mut s1);
}

#[test]
fn test9() {
    let mut s = String::from("hello");
    // we can create scope to borrow  2 time but stupid yeah.
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;
    println!("r1 r2: {}", r2);
}

#[test]
fn test10() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!(" {} and {}", r1, r2);
    let r3 = &mut s;
    println!(" {} ", r3);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

#[test]
fn test11() {
    let mut s = String::from("hello world");
    let l = first_word(&s);
    println!("The length of '{}' is {}.", s, l);
    //s.clear();
    eprintln!("check l:  {}", l);
}

#[test]
fn test12() {
    let mut s = String::from("hello world");
    println!("{}", s); // Display
    println!("{:?}", s); //Debug
    println!("{:?}", first_word(&s));
    println!("{:#?}", s);

    let sesason: [&str; 4] = ["Sprint", "Summer", "Fall", "Winter"];
    println!("{:?}", sesason);
    println!("{:?}", first_word(&s));
    println!("{:#?}", sesason); // pretty printing array

    /*
    hello world
    "hello world"
    "hello"
    "hello world"
    ["Sprint", "Summer", "Fall", "Winter"]
    "hello"
    [
        "Sprint",
        "Summer",
        "Fall",
        "Winter",
    ]

        */
}

#[test]
fn test13() {
    let pi: f64 = 3.141592653589793;
    println!("pi is {:.4}", pi); //1.1416
}

#[test]
fn test14() {
    let miles_away: i32 = 50;
    let miles_away_i8 = miles_away as i8;
    println!("miles away i31: {}", miles_away);
    println!("miles away i8: {}", miles_away);
    let miles_away_f64: f64 = 100.3215864;
    let miles_away_int: i32 = miles_away_f64 as i32;
    println!("miles away f64: {}", miles_away_f64);
    println!("miles away int: {}", miles_away_int);
    let string_num = String::from("100");
    let number: i32 = string_num.parse().expect("Not a number!");
    println!("number: {}", number);
}

/// The DBG! Macro
#[test]
fn test15() {
    let sesason: [&str; 4] = ["Sprint", "Summer", "Fall", "Winter"];
    println!("{:#?}", sesason);

    dbg!(&sesason);
    /*
        [src/test2.rs:192:5] &sesason = [
        "Sprint",
        "Summer",
        "Fall",
        "Winter",
    ]
        */
}

#[test]
fn test16() {
    let employee = ("John Doe", 30, "Engineering");
    let (name, age, department) = employee;
    println!("name: {}, age: {}, department: {}", name, age, department);
}

#[test]
fn test17() {
    let months_day = 0..31;
    for n in months_day {
        println!("Month days: {}", n);
    }
    let weekday = 1..=7;
    for d in weekday {
        println!("Week days: {}", d);
    }
}

#[test]
fn test18() {
    let season = "something";
    match season {
        "summer" => println!("Summer"),
        "fall" => println!("Fall"),
        "engineering" => println!("Engineering"),
        _ => println!("Sorry..."),
    }
}

#[test]
fn test19() {
    let numbers = 8;
    match numbers {
        2 | 3 | 5 | 7 => println!("Number of days: {}", numbers),
        4 | 5 | 7 => println!("Number of days: {}", numbers),
        _ => println!("Sorry..."),
    }

    match numbers {
        values if values % 2 == 0 => println!("Even number: {}", values),
        x if x % 2 != 0 => println!("Odd number: {}", x),
        _ => println!("Sorry..."),
    }
}

#[test]
fn test20() {
    let mut secound = 10;
    loop {
        if (secound < 0) {
            println!("Blastoff!...");
            break;
        }
        println!("{} seconds to blastoff...", secound);
        secound -= 1;
    }
}

#[test]
fn test21() {
    let mut secound = 21;
    loop {
        if (secound <= 0) {
            println!("Blastoff!...{}", secound);
            break;
        }
        if (secound % 2 == 0) {
            println!("{} seconds (even number), skipping 3 seconds...", secound);
            secound -= 3;
            continue;
        }
        println!("{} seconds to blastoff...", secound);
        secound -= 1;
    }
}

#[test]
fn test22() {
    let mut secound = 21;
    while secound > 0 {
        if (secound % 2 == 0) {
            println!("{} seconds (even number), skipping 3 seconds...", secound);
            secound -= 3;
            continue;
        }
        println!("{} seconds to blastoff...", secound);
        secound -= 1;
    }
    println!("{} Blastoff...", secound);
}

#[test]
fn test23() {
    count_down(5);
}

fn count_down(secound: i32) -> i32 {
    if secound < 0 {
        return -1;
    }
    println!("{} seconds to blastoff...", secound);
    count_down(secound - 1)
}

#[test]
fn test24() {
    let time = 2025;
    let year = time;
    println!("The time is {}. It is the year {}", time, year);
    println!("Memory address of time: {:p}", &time);
    println!("Memory address of year: {:p}", &year);

    let mut s = String::from("hello_world");
    println!("Mutable stack string address: {:p}", &s); // stack address of the String struct
    println!("The heap s address {:p} ", s.as_ptr()); // heap address of actual data
    println!("The  s as_byte {:p} ", s.as_bytes()); // heap address of actual data as byte slice.  Ex: -> Pointer { addr: 0x6000013e4a40, metadata: 11 }
    println!("The s len {} ", s.len()); // number of bytes used
    println!("The s capacity {} ", s.capacity()); // total bytes allocated on heap for s
    let s2 = s;
    println!("Mutable stack string address: {:p}", &s2);
    println!("The heaps s2 address {:p}", s2.as_ptr());
    println!("The  s2 as_byte {:p} ", s2.as_bytes());
    println!("The s2 len {} ", s2.len());
    println!("The s2 capacity {} ", s2.capacity());

    /*
    Binary
        The compiled executable file (or library), e.g. what cargo build produces.
        It contains machine code + embedded data sections.
    Static memory
        Memory region loaded from the binary when program starts.
        Lifetime is usually the whole program.
        Not allocated/freed like heap; not pushed/popped like stack.
    In Rust, these usually live in binary-backed sections:
        String literals like "hello" (&'static str)
        static items (global variables with fixed address)
        const values are inlined by compiler (not always a real memory location)

    Example:
        let a: &str = "hello"; // points to bytes in static/binary memory
        let s = String::from("hello"); // heap-allocated owned string
    So:
        "hello" bytes -> static/binary memory
        String buffer -> heap
        local variable bindings (a, s) -> stack (typically)
    Quick memory map mindset:
        Stack: local variables, fast, automatic scope cleanup
        Heap: dynamic allocations (String, Vec, etc.)
        Static/Binary memory: program-lifetime data embedded/loaded with executable
        */
}

#[test]
fn test25() {
    let mut person = String::from("John Doe");
    let genius = person;
    //println!("The person's genius is {}", person);
}

#[test]
fn test26() {
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;
    println!("My stack value: {}", my_stack_value);
    println!("My stack reference: {}", my_integer_reference);

    println!("My stack value address: {:p}", &my_integer_reference);
    println!("My stack reference address: {:p}", &my_integer_reference);

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
    println!("My heap value: {}", my_heap_value);
    println!("My heap reference: {}", my_heap_reference);

    println!("My heap value address: {:p}", &my_heap_value);
    println!("My heap reference address: {:p}", &my_heap_value);
}

#[test]
fn test27() {
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value;
    println!("dereference operator: {}", *my_integer_reference);
    println!("dereference stack value: {}", my_stack_value);
    println!("dereference stack reference: {}", my_integer_reference);

    let my_heap_value = String::from("Toyota");
    let my_heap_reference = &my_heap_value;
    println!("dereference operator: {}", *my_heap_reference);
    println!("dereference stack value: {}", my_heap_value);
    println!("dereference stack reference: {}", my_heap_reference);
}

#[test]
fn test28() {
    let apples = 6;
    print_my_value(apples);
    println!("apples value: {}", apples);
    println!("apples address: {:p}", &apples);
}
fn print_my_value(value: i32) {
    println!("Your value is: {}", value);
    println!("Your value address: {:p}", &value);
}

#[test]
fn test29() {
    let orange = String::from("Orange");
    println!("orange value: {}", orange);
    println!("orange address: {:p}", &orange);
    print_my_string(orange);
    //println!("orange value: {}", orange);
    //println!("orange address: {:p}", &orange);
}
fn print_my_string(s: String) {
    println!("Your string is: {}", s);
    println!("Your string address: {:p}", &s);
}

#[test]
fn test30() {
    let burger = String::from("Burger");
    println!("burger value: {}", burger);
    let burger = add_fries(burger);
    println!("new_burger value: {}", burger);
}
fn add_fries(mut meal: String) -> String {
    meal.push_str(" Fries");
    println!("meal push : {}", meal);
    return meal;
}

#[test]
fn test31() {
    let cake = bake_cake();
    println!("cake value: {}", cake);
}

fn bake_cake() -> String {
    String::from("Chocolate Mousse")
}

#[test]
fn test32() {
    let mut current_meal=  String::new();
    current_meal = add_fries(current_meal);
}

fn add_flour (mut meal: String) -> String  {
    meal.push_str(" Add Flour");
    return meal;
}

#[test]
fn project_section_6() {
    let is_concert = true;
    let is_even = is_concert;
    println!("is_concert: {}, is_even: {}", is_concert,is_even);

    let sushi = "Salman";
    let dinner = sushi;
    println!("sushi: {}, dinner: {}", sushi, dinner);

    let s1 = String::from("Salman");
    let s2= s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    let fish = eat_meal(dinner.to_string());
    println!("fish: {}", fish);
}

fn eat_meal(mut meal: String) -> String {
    meal.clear();
    return meal;

}
