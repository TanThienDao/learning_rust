fn longest<'a, 'b>(first: &'a str, secound: &'b str) -> &'a str {
    println!("The secound is {}", secound);
    first
}

fn main() {
    let orlando = String::from("Orlando");
    let result = {
        let san_francisco = String::from("San Francisco");
        longest(&orlando, &san_francisco)
    };
    println!("{}", orlando);
}
