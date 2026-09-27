# Rust Iterators: Complete Learning Guide

**A comprehensive guide covering all iterator methods, traits, and practical examples from the Idemy Rust course.**

---

## Table of Contents
1. [Introduction to Iterators](#introduction-to-iterators)
2. [Core Iterator Traits](#core-iterator-traits)
3. [Iterator Methods Reference Table](#iterator-methods-reference-table)
4. [Custom Functions from E-Commerce Project](#custom-functions-from-e-commerce-project)
5. [Best Practices](#best-practices)

---

## Introduction to Iterators

An **iterator** is an object that traverses through a collection of elements one at a time. Rust iterators are:
- **Lazy**: They don't do work until you consume them
- **Efficient**: They have minimal overhead
- **Composable**: You can chain multiple iterator methods together

### Three Ways to Iterate

| Method | Ownership | Use Case |
|--------|-----------|----------|
| `.iter()` | Borrows immutably | Read-only access to collection items |
| `.iter_mut()` | Borrows mutably | Modify items in the collection |
| `.into_iter()` | Takes ownership | Consume the collection (items move) |

---

## Core Iterator Traits

### The `Iterator` Trait

```rust
pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
    
    // ... 80+ other methods provided by default
}
```

Every iterator must implement the `next()` method. All other methods are provided for free!

**Example:**
```rust
let vec = vec![1, 2, 3];
let mut iter = vec.iter();

println!("{:?}", iter.next());  // Some(&1)
println!("{:?}", iter.next());  // Some(&2)
println!("{:?}", iter.next());  // Some(&3)
println!("{:?}", iter.next());  // None
```

### The `IntoIterator` Trait

Implements how a collection converts into an iterator. This is what enables `for` loops to work!

```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    
    fn into_iter(self) -> Self::IntoIter;
}
```

**Example:**
```rust
let vec = vec![1, 2, 3];

// These all work because Vec implements IntoIterator
for x in vec {}           // consumes vec (IntoIterator)
for x in &vec {}          // borrows vec (uses &Vec's IntoIterator)
for x in &mut vec {}      // borrows mutably (&mut Vec's IntoIterator)
```

---

## Iterator Methods Reference Table

| Method | Input | Output | Purpose | Example |
|--------|-------|--------|---------|---------|
| **`iter()`** | Collection | Iterator over `&T` | Creates immutable iterator | `vec.iter()` |
| **`iter_mut()`** | Collection | Iterator over `&mut T` | Creates mutable iterator | `vec.iter_mut()` |
| **`into_iter()`** | Collection | Iterator over `T` | Consumes collection | `vec.into_iter()` |
| **`filter()`** | Iterator | Iterator | Keeps items matching predicate | `.filter(\|x\| x > 5)` |
| **`map()`** | Iterator | Iterator | Transforms each item | `.map(\|x\| x * 2)` |
| **`filter_map()`** | Iterator | Iterator | Filter AND map in one step | `.filter_map(\|x\| ...)` |
| **`for_each()`** | Iterator | `()` | Executes closure for each item | `.for_each(\|x\| println!("{}", x))` |
| **`collect()`** | Iterator | Collection | Gathers items into a collection | `.collect::<Vec<_>>()` |
| **`find()`** | Iterator | `Option<T>` | Returns first matching item | `.find(\|x\| x == &5)` |
| **`any()`** | Iterator | `bool` | True if any item matches predicate | `.any(\|x\| x > 10)` |
| **`all()`** | Iterator | `bool` | True if all items match predicate | `.all(\|x\| x > 0)` |
| **`flatten()`** | Iterator of iterables | Iterator | Flattens nested iterables | `.flatten()` |
| **`flat_map()`** | Iterator | Iterator | Maps then flattens | `.flat_map(\|x\| ...)` |
| **`take()`** | Iterator, `n: usize` | Iterator | Takes first n items | `.take(3)` |
| **`skip()`** | Iterator, `n: usize` | Iterator | Skips first n items | `.skip(2)` |
| **`enumerate()`** | Iterator | Iterator | Adds index to each item | `.enumerate()` |
| **`zip()`** | Two iterators | Iterator of tuples | Pairs up items from two iterators | `iter1.zip(iter2)` |
| **`rev()`** | Iterator | Iterator | Reverses direction (if possible) | `.rev()` |
| **`fold()`** | Iterator, init, closure | Final value | Reduces to single value | `.fold(0, \|acc, x\| acc + x)` |
| **`reduce()`** | Iterator, closure | `Option<T>` | Like fold but no initial value | `.reduce(\|acc, x\| acc + x)` |
| **`sum()`** | Iterator of numbers | Number | Sums all items | `.sum::<u32>()` |
| **`product()`** | Iterator of numbers | Number | Multiplies all items | `.product::<u32>()` |
| **`max()`** | Iterator | `Option<T>` | Returns maximum item | `.max()` |
| **`min()`** | Iterator | `Option<T>` | Returns minimum item | `.min()` |
| **`count()`** | Iterator | `usize` | Counts remaining items | `.count()` |
| **`partition()`** | Iterator, predicate | Tuple of collections | Splits into two collections | `.partition(\|x\| x > 5)` |
| **`sort()`** | Iterator | Sorted iterator | Sorts items (requires Vec) | `vec.sort()` |
| **`sort_by_key()`** | Iterator, key fn | Sorted iterator | Sorts by key function | `vec.sort_by_key(\|x\| x.age)` |
| **`cloned()`** | Iterator of `&T` | Iterator of `T` | Clones each reference | `.cloned()` |
| **`last()`** | Iterator | `Option<T>` | Returns last item | `.last()` |
| **`nth()`** | Iterator, n | `Option<T>` | Returns item at index n | `.nth(2)` |
| **`nth_back()`** | Iterator, n | `Option<T>` | Returns nth from end | `.nth_back(1)` |
| **`position()`** | Iterator, predicate | `Option<usize>` | Finds index of matching item | `.position(\|x\| x == &5)` |
| **`step_by()`** | Iterator, step | Iterator | Steps through every nth item | `.step_by(2)` |
| **`lines()`** | String | Iterator of lines | Splits string by newlines | `"a\nb".lines()` |

---

## Detailed Method Examples

### 1. **filter() - Filter Items**
```rust
let numbers = vec![1, 2, 3, 4, 5, 6];
let even: Vec<i32> = numbers
    .iter()
    .filter(|x| x % 2 == 0)
    .copied()
    .collect();

println!("{:?}", even);  // [2, 4, 6]
```

### 2. **map() - Transform Items**
```rust
let numbers = vec![1, 2, 3];
let doubled: Vec<i32> = numbers
    .iter()
    .map(|x| x * 2)
    .copied()
    .collect();

println!("{:?}", doubled);  // [2, 4, 6]
```

### 3. **filter_map() - Filter AND Transform**
```rust
let words = vec!["1", "apple", "3", "4"];
let numbers: Vec<i32> = words
    .iter()
    .filter_map(|word| word.parse().ok())
    .collect();

println!("{:?}", numbers);  // [1, 3, 4]
```

### 4. **flatten() - Flatten Nested Collections**
```rust
let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
let flat: Vec<i32> = nested
    .into_iter()
    .flatten()
    .collect();

println!("{:?}", flat);  // [1, 2, 3, 4, 5]
```

### 5. **fold() - Accumulate Values**
```rust
let numbers = vec![1, 2, 3, 4];
let sum = numbers
    .iter()
    .fold(0, |acc, x| acc + x);

println!("{}", sum);  // 10
```

### 6. **zip() - Combine Two Iterators**
```rust
let names = vec!["Alice", "Bob", "Charlie"];
let ages = vec![25, 30, 35];

let pairs: Vec<_> = names
    .into_iter()
    .zip(ages)
    .collect();

println!("{:?}", pairs);
// [("Alice", 25), ("Bob", 30), ("Charlie", 35)]
```

### 7. **enumerate() - Get Index and Value**
```rust
let colors = vec!["red", "green", "blue"];
for (index, color) in colors.iter().enumerate() {
    println!("{}: {}", index, color);
}
// Output:
// 0: red
// 1: green
// 2: blue
```

### 8. **partition() - Split Into Two Collections**
```rust
let numbers = vec![1, 2, 3, 4, 5, 6];
let (even, odd): (Vec<_>, Vec<_>) = numbers
    .into_iter()
    .partition(|x| x % 2 == 0);

println!("Even: {:?}", even);  // [2, 4, 6]
println!("Odd: {:?}", odd);    // [1, 3, 5]
```

### 9. **any() and all() - Logical Checks**
```rust
let numbers = vec![1, 2, 3, 4, 5];

let has_even = numbers.iter().any(|x| x % 2 == 0);
println!("{}", has_even);  // true

let all_positive = numbers.iter().all(|x| x > &0);
println!("{}", all_positive);  // true
```

### 10. **take() and skip() - Slice Iterator**
```rust
let numbers = vec![1, 2, 3, 4, 5];

let first_three: Vec<_> = numbers
    .iter()
    .take(3)
    .copied()
    .collect();
println!("{:?}", first_three);  // [1, 2, 3]

let skip_two: Vec<_> = numbers
    .iter()
    .skip(2)
    .copied()
    .collect();
println!("{:?}", skip_two);  // [3, 4, 5]
```

---

## Custom Functions from E-Commerce Project

### Project Context
An e-commerce store managing orders with the following data structures:

```rust
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Product { Blender, Microwave, Toaster, Fridge }

#[derive(Debug, Clone)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

#[derive(Debug, Clone)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}
```

### 1. **extract_blender_orders** - Filter Specific Product
**Purpose:** Extract all orders for a specific product (Blender)

```rust
fn extract_blender_orders(order: &Vec<CustomerOrder>) -> Vec<&CustomerOrder> {
    order
        .iter()
        .filter(|o| o.product == Product::Blender)
        .collect()
}
```

**How it works:**
- `.iter()` - Creates iterator over order references
- `.filter(|o| o.product == Product::Blender)` - Keeps only Blender orders
- `.collect()` - Gathers results into a Vec

**Example Output:**
```
[
  CustomerOrder { product: Blender, quantity: 3, shipped: false },
  CustomerOrder { product: Blender, quantity: 1, shipped: false }
]
```

---

### 2. **find_total_microwave_quantity** - Sum Quantities (Method A: filter + map)
**Purpose:** Calculate total quantity of a specific product across all orders

```rust
fn find_total_microwave_quantity(orders: &Vec<CustomerOrder>) -> u32 {
    orders
        .iter()
        .filter(|order| order.product == Product::Microwave)
        .map(|order| order.quantity)
        .sum()
}
```

**How it works:**
- `.filter()` - Keep only Microwave orders
- `.map()` - Extract the quantity from each order
- `.sum()` - Add all quantities together

**Example:** `1 + 5 = 6`

---

### 3. **find_total_microwave_quantity_version_2** - Sum Quantities (Method B: filter_map)
**Purpose:** Same result, using filter_map for efficiency

```rust
fn find_total_microwave_quantity_version_2(orders: &Vec<CustomerOrder>) -> u32 {
    orders
        .iter()
        .filter_map(|order| {
            if order.product == Product::Microwave {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum()
}
```

**How it works:**
- `.filter_map()` - Combines filter + map in one step
- Returns `Some(quantity)` for Microwave orders, `None` otherwise
- `.sum()` - Adds all Some values

**Why use this?** More efficient - only one iterator operation instead of two.

---

### 4. **quantity_filter** - Filter by Command-Line Argument
**Purpose:** Filter orders by quantity threshold from command-line input

```rust
fn quantity_filter(order: &Vec<CustomerOrder>) -> Vec<&CustomerOrder> {
    let args_input = args().skip(1).take(1).collect::<Vec<String>>();
    let mut quantity = 2; // default value
    
    if args_input.len() == 1 {
        quantity = args_input[0].parse::<u32>().unwrap_or(2);
    }
    
    order
        .iter()
        .filter(|order| order.quantity >= quantity)
        .collect()
}
```

**How it works:**
- `args().skip(1).take(1)` - Get first command-line argument
- `.parse::<u32>()` - Convert to number, fallback to 2 if invalid
- `.filter()` - Keep orders with quantity >= threshold

**Usage:**
```bash
cargo run --bin iterators -- 5
# Returns orders with quantity >= 5
```

---

### 5. **quantity_filter_version_2** - Cleaner Argument Parsing
**Purpose:** Same functionality, using more idiomatic iterator chains

```rust
fn quantity_filter_version_2(order: &Vec<CustomerOrder>) -> Vec<&CustomerOrder> {
    let args_input = args()
        .skip(1)
        .take(1)
        .map(|quantity| quantity.parse::<u32>().unwrap_or(2))
        .next()
        .unwrap_or(2);
    
    order
        .iter()
        .filter(|order| order.quantity >= args_input)
        .collect()
}
```

**Key improvements:**
- Uses `.map()` on arguments before collecting
- Uses `.next()` to get single value
- More functional/less imperative style

---

### 6. **inventory_for_unshipped_orders** - HashMap Aggregation
**Purpose:** Calculate inventory needed for unshipped orders by product

```rust
fn inventory_for_unshipped_orders(
    order: &Vec<CustomerOrder>
) -> HashMap<&Product, u32> {
    let mut result: HashMap<&Product, u32> = HashMap::new();
    
    for order in order.iter().filter(|o| !o.shipped) {
        let entry = result.entry(&order.product).or_insert(0);
        *entry += order.quantity;
    }
    result
}
```

**How it works:**
- `.filter(|o| !o.shipped)` - Keep only unshipped orders
- `.entry(&order.product)` - Get or create HashMap entry
- `.or_insert(0)` - Insert 0 if key doesn't exist, return mutable reference
- `*entry += order.quantity` - Dereference and accumulate

**Example Output:**
```
{Fridge: 10, Toaster: 2, Blender: 4}
```

**What's a reference dereference?**
- `entry` is a `&mut u32` (address pointing to a u32)
- `*entry` follows the address to access the actual u32 value

---

### 7. **inventory_for_unshipped_orders_version2** - Using fold()
**Purpose:** Same result, using fold() for functional approach

```rust
fn inventory_for_unshipped_orders_version2(
    order: &Vec<CustomerOrder>
) -> HashMap<&Product, u32> {
    order
        .iter()
        .filter(|o| !o.shipped)
        .fold(HashMap::new(), |mut acc, order| {
            *acc.entry(&order.product).or_insert(0) += order.quantity;
            acc
        })
}
```

**How it works:**
- `.fold(initial_value, accumulator_closure)` - Reduces to single value
- `HashMap::new()` - Initial accumulator
- Closure takes `acc` (accumulator) and `order`, modifies `acc`, returns it
- **More functional** - no explicit mutable variables

---

### 8. **ship_next_unshipped_order** - Modify First Match
**Purpose:** Find and mark first unshipped order as shipped

```rust
fn ship_next_unshipped_order(orders: &mut Vec<CustomerOrder>) -> CustomerOrder {
    for (i, o) in orders.iter_mut().enumerate() {
        if !o.shipped {
            o.shipped = true;
            return o.clone();
        }
    }
    panic!("No unshipped orders found");
}
```

**How it works:**
- `orders.iter_mut()` - Get mutable references (to modify items)
- Loop finds first unshipped order
- `o.shipped = true` - Modify the field
- `.clone()` - Return a copy of the modified order

**Why mutable reference?** Need `&mut` to modify items in the vector.

---

### 9. **ship_next_unshipped_order_version2** - Using find()
**Purpose:** Same functionality, using iterator find() method

```rust
fn ship_next_unshipped_order_version2(
    orders: &mut Vec<CustomerOrder>
) -> CustomerOrder {
    if let Some(order) = orders.iter_mut().find(|o| !o.shipped) {
        order.shipped = true;
        order.clone()
    } else {
        panic!("No unshipped orders found");
    }
}
```

**Key improvements:**
- `.find()` - Returns `Option<&mut T>` for first match
- `if let` - Cleaner than looping
- **More idiomatic** - uses high-level iterator method

---

### 10. **aggregate_customers** - Complex Data Aggregation
**Purpose:** Combine orders with customer data into Customer structs

```rust
fn aggregate_customers(
    orders: &Vec<CustomerOrder>,
    customer_ids_by_order: &[u32],
) -> Vec<Customer> {
    let mut customers_map: HashMap<u32, Vec<CustomerOrder>> = HashMap::new();

    // Step 1: Pair orders with customer IDs using zip
    for (order, &customer_id) in orders.iter().zip(customer_ids_by_order.iter()) {
        customers_map
            .entry(customer_id)
            .or_insert_with(Vec::new())
            .push(order.clone());
    }

    // Step 2: Convert HashMap to Vec of Customers
    let mut customers: Vec<Customer> = customers_map
        .into_iter()
        .map(|(id, orders)| Customer { id, orders })
        .collect();

    // Step 3: Sort by customer ID
    customers.sort_by_key(|customer| customer.id);
    customers
}
```

**How it works:**
1. **`.zip()`** - Pairs each order with its customer ID
2. **`.entry().or_insert_with()`** - Get HashMap entry, create empty Vec if needed
3. **`.into_iter().map()`** - Convert HashMap entries to Customer structs
4. **`.sort_by_key()`** - Sort by customer ID

**Example Output:**
```rust
[
  Customer { id: 1, orders: [...] },
  Customer { id: 2, orders: [...] },
  Customer { id: 3, orders: [...] },
  Customer { id: 4, orders: [...] }
]
```

---

### 11. **aggregate_customers_version2** - Pure Functional Approach
**Purpose:** Same result, using only iterator chains and fold()

```rust
fn aggregate_customers_version2(
    orders: &Vec<CustomerOrder>,
    customer_ids_by_order: &[u32],
) -> Vec<Customer> {
    let mut customers = orders
        .into_iter()
        .zip(customer_ids_by_order)
        // Step 1: Fold into HashMap
        .fold(HashMap::new(), |mut ids_to_orders, (order, customer_id)| {
            ids_to_orders
                .entry(customer_id)
                .or_insert(Vec::new())
                .push(order);
            ids_to_orders
        })
        // Step 2: Convert to Customer struct
        .into_iter()
        .map(|(id, orders)| Customer {
            id: *id,
            orders: orders.into_iter().cloned().collect(),
        })
        .collect::<Vec<Customer>>();

    // Step 3: Sort by ID
    customers.sort_by_key(|customer| customer.id);
    customers
}
```

**Why this version?**
- **Fully functional** - no explicit loops
- **Single chain** - all operations in one fluent chain
- **More elegant** - demonstrates advanced iterator patterns

**Trade-off:** Less readable for beginners, but more efficient.

---

## Best Practices

### 1. **Choose the Right Iteration Method**

```rust
// ✅ Good: Use into_iter() when you don't need the original
for x in collection {}

// ✅ Good: Use iter() when you need to keep the original
for x in &collection {}

// ✅ Good: Use iter_mut() when modifying items
for x in &mut collection { x.modify(); }

// ❌ Avoid: Don't use .iter() then clone() unnecessarily
for x in collection.iter().cloned() {}  // Worse than into_iter
```

### 2. **Chain Methods for Efficiency**
```rust
// ✅ Good: Lazy evaluation, single pass
vec.iter()
   .filter(|x| x > &5)
   .map(|x| x * 2)
   .collect()

// ❌ Avoid: Multiple passes through data
let filtered: Vec<_> = vec.iter().filter(|x| x > &5).collect();
let result: Vec<_> = filtered.iter().map(|x| x * 2).collect();
```

### 3. **Avoid Unnecessary collect()**
```rust
// ✅ Good: Pass iterator directly
result.push(numbers.iter().sum());

// ❌ Avoid: Unnecessary collection
let sum = numbers.iter().sum::<i32>();
let result = vec![sum];
```

### 4. **Use filter_map() Instead of filter() + map()**
```rust
// ✅ Good: One operation
values.iter().filter_map(|x| parse_int(x)).collect()

// ❌ Avoid: Two operations
values.iter()
      .filter(|x| parse_int(x).is_ok())
      .map(|x| parse_int(x).unwrap())
      .collect()
```

### 5. **Pretty-Print Complex Output**
```rust
// ✅ Good: Use {:#?} for readable output
println!("{:#?}", complex_data);

// ❌ Avoid: Single-line debug output
println!("{:?}", complex_data);
```

### 6. **Use sort_by_key() for Custom Sorting**
```rust
// ✅ Good: Sort by specific field
customers.sort_by_key(|c| c.id);

// ❌ Avoid: Manual sorting logic
customers.sort_by(|a, b| a.id.cmp(&b.id));
```

### 7. **Handle Results from parse()**
```rust
// ✅ Good: Use unwrap_or() for defaults
let num = input.parse::<u32>().unwrap_or(10);

// ❌ Avoid: Panicking on error
let num = input.parse::<u32>().unwrap();
```

---

## Quick Reference: When to Use Each Method

| Goal | Method | Example |
|------|--------|---------|
| Keep items that match condition | `filter()` | `.filter(\|x\| x > 5)` |
| Transform each item | `map()` | `.map(\|x\| x * 2)` |
| Filter AND transform | `filter_map()` | `.filter_map(\|x\| ...)` |
| Find single matching item | `find()` | `.find(\|x\| x == 5)` |
| Check if any match condition | `any()` | `.any(\|x\| x > 10)` |
| Check if all match condition | `all()` | `.all(\|x\| x > 0)` |
| Flatten nested collections | `flatten()` | `.flatten()` |
| Flatten then map | `flat_map()` | `.flat_map(\|x\| ...)` |
| Add index to items | `enumerate()` | `.enumerate()` |
| Combine two iterators | `zip()` | `.zip(other)` |
| Reduce to single value | `fold()` or `reduce()` | `.fold(init, \|acc, x\| ...)` |
| Split into two collections | `partition()` | `.partition(\|x\| cond)` |
| Take first n items | `take()` | `.take(3)` |
| Skip first n items | `skip()` | `.skip(2)` |
| Copy collection | `cloned()` | `.cloned()` |
| Count items | `count()` | `.count()` |
| Sum numbers | `sum()` | `.sum::<i32>()` |
| Multiply numbers | `product()` | `.product::<i32>()` |
| Find max/min | `max()` / `min()` | `.max()` |
| Get last item | `last()` | `.last()` |
| Get item at index | `nth()` | `.nth(2)` |

---

## Summary

Iterators are the heart of Rust's functional programming capabilities. They are:
- **Powerful**: Chain multiple operations elegantly
- **Efficient**: Lazy evaluation avoids unnecessary work
- **Safe**: No index out-of-bounds errors
- **Expressive**: Code reads like what you want to do

Master iterators to write cleaner, faster Rust code!

---

**Last Updated:** September 26, 2026
**Course:** Rust Learning - Idemy Iterators Module

