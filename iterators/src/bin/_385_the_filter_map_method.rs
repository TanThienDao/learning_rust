/// The filter_map method both filters and transform
/// a subset of elements from an iterator.

fn main() {
    let stocks = ["nvda", "", "amd", "", "tsla", "aapl"];
    let capitalized_stocks = stocks
        .iter()
        .filter(|st| !st.is_empty())
        .map(|st| st.to_uppercase())
        .collect::<Vec<String>>();
    println!("{:?}", capitalized_stocks);

    //filter_map method return an option enums not boolean.
    let capitalized_capitalized_stocks = stocks
        .iter()
        .filter_map(|st| {
            if !st.is_empty() {
                Some(st.to_uppercase())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    println!("{:?}", capitalized_capitalized_stocks);
}
