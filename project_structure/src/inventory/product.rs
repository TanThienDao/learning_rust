use fake::Dummy;
/// A cattegory of product that our business sells
#[derive(Debug, Dummy)]
pub enum ProductCategory {
    Ladder,
    Hammer,
}
/// A concrete item in stock with in our product strcuter
#[derive(Debug,Dummy)]
pub struct Item {
    pub name: String,
    pub category: ProductCategory,
    pub quality: u32,
}

///create new item
impl Item {
    pub fn new(name: String, category: ProductCategory, quality: u32) -> Self {
        super::talk_to_manager();
        Self {
            name,
            category,
            quality,
        }
    }
}
