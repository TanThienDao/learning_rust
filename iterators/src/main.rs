/*
NOTE: This exercise will (a) likely take some time
and (b) produces a lot of output. I've added the
#![allow(unused, dead_code)] directive at the top
of the file to silence some compiler warnings.
Feel free to comment out certain lines/solutions
to reduce output.

---

Let's imagine we're running an e-commerce store that
sells home appliances. Another developer has left
some starter code to work with.

The Product enum has 4 variants for the products
we sell: blender, microwave, toaster, and fridge.

The CustomerOrder struct represents an online order.
It stores the ordered Product, its quantity, and
whether we've shipped it to the customer. There is
a complementary 'new' constructor in the 'impl'
block.

The Customer struct represents a customer. Each
customer has a unique numeric ID and a vector of
their orders.

---

In `main`, we have an `orders` vector with the 6
orders in our system.

We also have a `customer_ids_by_order` array that
lists the customer ID of each customer who placed
each of the 6 orders.

Our boss needs help figuring out critical
business numbers! Help him!

----

Extract all the customer orders where the customer
ordered a Blender. Our goal is a vector of
&CustomerOrder values. Print out the vector.
It should have 2 total orders.

HINT: Pretty-printing the output will make it
easier to parse.

---

The boss would like to know the total quantity of
microwaves ordered across all customer orders. Filter
for the customer orders where the Product is a
Microwave, extract the 'quantity' field for each
customer order, then calculate the sum of those
values. The answer should be 6.

BONUS: Solve the challenge with both (a) filter + map
and (b) filter_map

---

The boss would like to pass in a quantity from the
command line. They want to see a printed vector of
all orders where the quantity is greater than or
equal to their input.

For example,
'cargo run -- 5'

should print a vector of the two customer orders
with a quantity greater than or equal to 5:

[
CustomerOrder::new(Product::Microwave, 5, true),
CustomerOrder::new(Product::Fridge, 10, false),
]

If the boss does not provide a command-line argument
OR provides an invalid numeric value, fallback to
printing customer orders with a quantity greater
than or equal to 2.

---

The boss would like to know how much inventory
of each product we need for unshipped orders.

Create a HashMap where each key will be a &Product
and each value will be the sum of that products's
quantities across unshipped orders. Make sure to
target only unshipped orders.

Print out the HashMap. It should be:
{Fridge: 10, Toaster: 2, Blender: 4}

---

Our warehouse worker informs us they've shipped
the next unshipped order. Find the first
unshipped order among the customer orders and
change its `shipped` field to `true`. Print out
the customer orders to confirm.

---

THIS IS A TOUGH ONE.

The boss would like to see a vector of Customer
structs. Each Customer strict will hold the user's
id and a vector of their orders. Find a way to merge
the customer orders with the customers who made them,
then aggregate the data into Customer structs,
then collect the Customers in a vector, then
sort the collection by customer id.

The resulting vector should look like this:

[

Customer {
  id: 1,
  orders: [
    CustomerOrder { product: Microwave, quantity: 1, shipped: true },
    CustomerOrder { product: Fridge, quantity: 10, shipped: false }
  ]
},

Customer {
  id: 2,
  orders: [
   CustomerOrder { product: Blender, quantity: 3, shipped: false },
   CustomerOrder { product: Toaster, quantity: 2, shipped: false }
  ]
},

Customer {
  id: 3,
  orders: [
   CustomerOrder { product: Microwave, quantity: 5, shipped: true }
  ]
},

Customer {
  id: 4,
  orders: [
    CustomerOrder { product: Blender, quantity: 1, shipped: false }
  ]
}

]
*/

#![allow(unused, dead_code)]
use std::collections::HashMap;
use std::env::args;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug, Clone)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug, Clone)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    println!(
        "Extract all the customer Blender orders: {:#?}",
        extract_blender_orders(&orders)
    );

    println!(
        "Total quantity of microwaves ordered: {}",
        find_total_microwave_quantity(&orders)
    );
    println!(
        "Total quantity of microwaves ordered version 2: {}",
        find_total_microwave_quantity_version_2(&orders)
    );
    println!(
        "Customer orders with quantity greater than or equal to input: {:#?}",
        quantity_filter(&orders)
    );
    println!(
        "Customer orders with quantity greater than or equal to input Version 2: {:#?}",
        quantity_filter_version_2(&orders)
    );
    println!(
        "Inventory needed for unshipped orders: {:#?}",
        inventory_for_unshipped_orders(&orders)
    );
    println!(
        "Inventory needed for unshipped orders version 2: {:#?}",
        inventory_for_unshipped_orders_version2(&orders)
    );
    println!(
        "Shipping the next unshipped order: {:#?}",
        ship_next_unshipped_order(&mut orders)
    );
    println!(
        "Shipping the next unshipped order version 2: {:#?}",
        ship_next_unshipped_order_version2(&mut orders)
    );
    println!(
        "Customer orders ID: {:#?}",
        aggregate_customers(&orders, &customer_ids_by_order)
    );
    println!(
        "Customer orders ID Version 2: {:#?}",
        aggregate_customers_version2(&orders, &customer_ids_by_order)
    );
}

/// Extract all the customer orders where the customer
/// ordered a Blender. Our goal is a vector of
/// &CustomerOrder values. Print out the vector.
/// It should have 2 total orders.
///
/// HINT: Pretty-printing the output will make it
/// easier to parse.

fn extract_blender_orders(order: &Vec<CustomerOrder>) -> Vec<&CustomerOrder> {
    let blender_orders: Vec<&CustomerOrder> = order
        .iter()
        .filter(|o| o.product == Product::Blender)
        .collect::<Vec<&CustomerOrder>>();
    blender_orders
}

/// The boss would like to know the total quantity of
/// microwaves ordered across all customer orders.
/// Filter for the customer orders where the Product is a
/// Microwave, extract the 'quantity' field for each
/// customer order, then calculate the sum of those
/// values. The answer should be 6.
///
/// BONUS: Solve the challenge with both (a) filter + map
/// and (b) filter_map
fn find_total_microwave_quantity(orders: &Vec<CustomerOrder>) -> u32 {
    let total_microwave_quantity: u32 = orders
        .iter()
        .filter(|order| order.product == Product::Microwave)
        .map(|order| order.quantity)
        .sum::<u32>();
    total_microwave_quantity
}
fn find_total_microwave_quantity_version_2(orders: &Vec<CustomerOrder>) -> u32 {
    let total_microwave_quantity: u32 = orders
        .iter()
        .filter_map(|order| {
            if order.product == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum::<u32>();
    total_microwave_quantity
}
/// The boss would like to pass in a quantity from the
/// command line. They want to see a printed vector of
/// all orders where the quantity is greater than or
/// equal to their input.
///
/// For example,
/// 'cargo run -- 5'
///
/// should print a vector of the two customer orderss
/// with a quantity greater than or equal to 5:
///
/// [
/// CustomerOrder::new(Product::Microwave, 5, true),
/// CustomerOrder::new(Product::Fridge, 10, false),
/// ]
///
/// If the boss does not provide a command-line argument
/// OR provides an invalid numeric value, fallback to
/// printing customer orders with a quantity greater
/// than or equal to 2.
fn quantity_filter(order: &Vec<CustomerOrder>) -> Vec<&CustomerOrder> {
    let args_input = args().skip(1).take(1).collect::<Vec<String>>();
    let mut quantity = 0;
    if args_input.len() != 1 {
        quantity = 2;
    } else {
        quantity = args_input[0].parse::<u32>().unwrap();
    }
    let filterd_orders = order
        .iter()
        .filter(|order| order.quantity >= quantity)
        .collect::<Vec<&CustomerOrder>>();
    filterd_orders
}
fn quantity_filter_version_2(order: &Vec<CustomerOrder>) -> Vec<&CustomerOrder> {
    let args_input = args()
        .skip(1)
        .take(1)
        .map(|quantity| quantity.parse::<u32>().unwrap_or(2))
        .next()
        .unwrap_or(2);
    let filterd_orders = order
        .iter()
        .filter(|order| order.quantity >= args_input)
        .collect::<Vec<&CustomerOrder>>();
    filterd_orders
}
/// The boss would like to know how much inventory
/// of each product we need for unshipped orders.
///
/// Create a HashMap where each key will be a &Product
/// and each value will be the sum of that products's
/// quantities across unshipped orders. Make sure to
/// target only unshipped orders.
///
/// Print out the HashMap. It should be:
/// {Fridge: 10, Toaster: 2, Blender: 4}
fn inventory_for_unshipped_orders(order: &Vec<CustomerOrder>) -> HashMap<&Product, u32> {
    let mut result: HashMap<&Product, u32> = HashMap::new();
    for order in order.iter().filter(|o| !o.shipped) {
        let entry = result.entry(&order.product).or_insert(0);
        *entry += order.quantity;
    }
    result
}
fn inventory_for_unshipped_orders_version2(order: &Vec<CustomerOrder>) -> HashMap<&Product, u32> {
    let product_quatities =
        order
            .iter()
            .filter(|o| o.shipped == false)
            .fold(HashMap::new(), |mut acc, order| {
                *acc.entry(&order.product).or_insert(0) += order.quantity;
                acc
            });
    product_quatities
}

/// Our warehouse worker informs us they've shipped
/// the next unshipped order. Find the first
/// unshipped order among the customer orders and
/// change its `shipped` field to `true`. Print out
/// the customer orders to confirm.
fn ship_next_unshipped_order(orders: &mut Vec<CustomerOrder>) -> CustomerOrder {
    for (i, o) in orders.iter_mut().enumerate() {
        if !o.shipped {
            o.shipped = true;
            return o.clone();
        }
    }
    panic!("No unshipped orders found");
}
fn ship_next_unshipped_order_version2(orders: &mut Vec<CustomerOrder>) -> CustomerOrder {
    if let Some(order) = orders.iter_mut().find(|o| o.shipped == false) {
        order.shipped = true;
        order.clone()
    } else {
        panic!("No unshipped orders found");
    }
}

/// The boss would like to see a vector of Customer
/// structs. Each Customer strict will hold the user's
/// id and a vector of their orders. Find a way to merge
/// the customer orders with the customers who made them,
/// then aggregate the data into Customer structs,
/// then collect the Customers in a vector, then
/// sort the collection by customer id.

pub fn aggregate_customers(
    orders: &Vec<CustomerOrder>,
    customer_ids_by_order: &[u32],
) -> Vec<Customer> {
    let mut customers_map: HashMap<u32, Vec<CustomerOrder>> = HashMap::new();

    for (order, &customer_id) in orders.iter().zip(customer_ids_by_order.iter()) {
        customers_map
            .entry(customer_id)
            .or_insert_with(Vec::new)
            .push(order.clone());
    }

    let mut customers: Vec<Customer> = customers_map
        .into_iter()
        .map(|(id, orders)| Customer { id, orders })
        .collect();

    customers.sort_by_key(|customer| customer.id);
    customers
}

pub fn aggregate_customers_version2(
    orders: &Vec<CustomerOrder>,
    customer_ids_by_order: &[u32],
) -> Vec<Customer> {
    let mut customers = orders
        .into_iter()
        .zip(customer_ids_by_order)
        .fold(HashMap::new(), |mut ids_to_orders, (order, customer_id)| {
            ids_to_orders
                .entry(customer_id)
                .or_insert(Vec::new())
                .push(order);
            ids_to_orders
        })
        .into_iter()
        .map(|(id, orders)| Customer {
            id: *id,
            orders: orders.into_iter().cloned().collect(),
        })
        .collect::<Vec<Customer>>();

    customers.sort_by_key(|customer| customer.id);

    customers
}
