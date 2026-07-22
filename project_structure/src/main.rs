/*mod inventory;
mod order;

use std::{
    fmt,
    io::{stdin,stdout},
};
use inventory::product::{Item, ProductCategory};
use inventory::{FLOOR_SOACE, MANAGER as INVENTORY_MANAGER, talk_to_manager};
use order::MANAGER as ORDER_MANAGER;
use fake::{Fake, Faker};*/

/*mod inventory {
    const FLOOR_SOACE:i32 = 10000;
    pub const MANAGER:&str = "Ivan Inventory";

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

    fn talk_to_manager(){
        println!("Hey {}, how's your coffee?",MANAGER);
    }
}*/

/*mod order {
    pub const MANAGER: &str = "Olga Orders";

    fn talk_to_manager() {
        println!("Hey {}, how's your coffee?", MANAGER);
    }
}*/
use fake::{Fake, Faker};
use project_structure::{Item,ProductCategory,FLOOR_SOACE,INVENTORY_MANAGER,ORDER_MANAGER,talk_to_manager};

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        INVENTORY_MANAGER, ORDER_MANAGER, FLOOR_SOACE
    );

    talk_to_manager();

    //let fav_category = inventory::product::ProductCategory::Hammer;
    let fav_category = ProductCategory::Hammer;
    println!("My favorite category is {:?}", fav_category);

    let tall_ladder = Item {
        name: String::from("Tall Ladder"),
        category: ProductCategory::Ladder,
        quality: 20,
    };

    println!("\nTall of Ladder is {:#?}", tall_ladder);

    let short_ladder = Item::new(String::from("Short Ladder"), ProductCategory::Ladder, 10);
    println!("\nShort of Ladder is {:#?}", short_ladder);

    let fake: Item = Faker.fake();
    println!("Fake is {:#?}", fake);

    let random_category: ProductCategory = Faker.fake();
    println!("Random of Faker is {:#?}", random_category);



}
