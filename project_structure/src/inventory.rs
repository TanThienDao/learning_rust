const FLOOR_SOACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

#[derive(Debug)]
enum ProductCategory {
    Ladder,
    Hammer,
}
#[derive(Debug)]
struct Item {
    name: String,
    category: ProductCategory,
    quality: f64,
}

fn talk_to_manager() {
    println!("Hey {}, how's your coffee?", MANAGER);
}
