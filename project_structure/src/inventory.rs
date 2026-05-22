pub const FLOOR_SOACE: i32 = 10000;
pub const MANAGER: &str = "Ivan Inventory";

pub fn talk_to_manager() {
    println!("Hey {}, how's your coffee?", MANAGER);
    println!("absolute path {}", crate::inventory::MANAGER);
}

pub mod product;
/*mod product {
    #[derive(Debug)]
    pub enum ProductCategory {
        Ladder,
        Hammer,
    }
    #[derive(Debug)]
    pub struct Item {
        pub name: String,
        pub category: ProductCategory,
        pub quality: f64,
    }
}*/
