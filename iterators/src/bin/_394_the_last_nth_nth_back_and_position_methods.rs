fn main() {
    let performance = [
        "Rustful Five",
        "Rustful Four",
        "Rustful Three",
        "Rustful Two",
        "Rustful One",
    ];

    let last = performance.iter().last().unwrap_or(&"No performance");
    println!("Last : {:?}", last);

    //nth method extracts an element based on its index position.
    let third = performance.iter().nth(2).unwrap_or(&"No performance");
    println!("Third : {:?}", third);

    //nth_back method extracts an element based on its index position from the end of the iterator.
    let second_last = performance.iter().nth_back(1).unwrap_or(&"No performance");
    println!("Second Last : {:?}", second_last);

    //position method returns the index of the first occurrence of an element in the iterator.
    let position = performance
        .iter()
        .position(|&x| x == "Rustful Three")
        .unwrap_or(usize::MAX);
    if position != usize::MAX {
        println!("Position of 'Rustful Three' : {}", position);
    } else {
        println!("'Rustful Three' not found in the performance list.");
    }
}
