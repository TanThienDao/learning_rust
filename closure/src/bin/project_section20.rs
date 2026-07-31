#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermarketItem>,
}

impl ShoppingCart {
    fn traverse_items<F>(&mut self, mut operation: F)
    where
        F: FnMut(&mut SupermarketItem),
    {
        let mut count = 0;
        while count <= self.items.len() - 1 {
            operation(self.items.get_mut(count).unwrap());
            count += 1;
        }
    }
    fn checkout<F>(self, operation: F)
    where
        F: FnOnce(Self),
    {
        operation(self);
    }
}
fn main() {
    let mut shopping_cart = ShoppingCart {
        items: vec![
            SupermarketItem {
                name: String::from("APPLE"),
                price: 3.99,
            },
            SupermarketItem {
                name: String::from("BANANA"),
                price: 2.99,
            },
        ],
    };
    shopping_cart.traverse_items(|item| {
        item.price *= 0.85;
    });

    shopping_cart.traverse_items(|item| {
        item.name = item.name.to_lowercase();
    });

    let mut total_price = 0.0;
    shopping_cart.checkout(|mut shopping_cart| {
        println!("shopping_cart: {:?}", shopping_cart);
        shopping_cart.traverse_items(|item| {
            total_price += item.price;
        })
    });

    println!("total_price: {:.2}", total_price);
}
