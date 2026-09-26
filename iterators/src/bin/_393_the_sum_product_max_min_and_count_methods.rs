/// This category is consuming adapters because they exhaust the original iterator ot produce a new value.
///
fn main() {
    let numbers = [4, 8, 15, 16, 23, 42];

    /// The sum method returns the sum of all the values in the iterator.
    let total = numbers.iter().sum::<i32>();
    println!("Total: {}", total);

    /// The product method returns the product of all the values in the iterator.
    /// meaning it multiplies all the values together.
    let product = numbers.iter().product::<i32>();
    println!("Product: {}", product);

    /// The max method returns the maximum value in the iterator.
    let max = numbers.iter().max().unwrap();
    println!("Max: {}", max);

    /// The min method returns the minimum value in the iterator.
    let min = numbers.iter().min().unwrap();
    println!("Min: {}", min);

    /// The count method returns the number of elements in the iterator.
    let count = numbers.iter().count();
    println!("Count: {}", count);

    let invalid = 0.0 / 0.0;
    println!("Invalid: {}", invalid);

    let number_invalid = vec![4.6, 8.8, 0.0 / 0.0, 15.2, 16.1, 23.3, 42.9, f64::NAN];
    let total_invalid = number_invalid.iter().sum::<f64>();
    println!("Vector with invalid: {:?}", number_invalid);
    println!("Total with invalid: {}", total_invalid);

    let total_filter = number_invalid
        .iter()
        .filter(|x| !x.is_nan())
        .copied()
        .collect::<Vec<f64>>();
    println!("Total filter: {:?}", total_filter);
    println!("Total with filter: {}", total_filter.iter().sum::<f64>());

    let max_filter = number_invalid
        .iter()
        .filter(|x| !x.is_nan())
        .copied()
        .reduce(f64::max)
        .unwrap();
    println!("Max filter: {}", max_filter);

    let max_filter_2 = number_invalid
        .iter()
        .filter(|x| !x.is_nan())
        .copied()
        .reduce(|accu, curr| accu.max(curr))
        .unwrap();
    println!("Max filter 2: {}", max_filter_2);
}
