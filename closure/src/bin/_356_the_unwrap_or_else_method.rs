fn main() {
    let option = Some("Salami");
    let food = option.unwrap_or_else(|| "Pizza");
    println!("{}", food);

    let option: Option<&str> = None;
    let food = option.unwrap_or_else(|| "Pizza");
    println!("{}", food);

    let option: Option<&str> = None;
    let pizza_fan = false;
    let food = option.unwrap_or_else(|| if pizza_fan { "Pizza" } else { "Hot Pockets" });
    println!("{}", food);
}
