mod _368_the_intoiterator_trait_in_action;

/// To iterate means to perform repeatedly.
/// Iteration is repeating the same operation on a sequence of items,
/// one item at a time.
///
/// The Loop keyword continually executes a block until we force termination
/// with the break keyword.
fn main() {
    let numnbers = vec![4, 8, 15, 16, 23, 42];

    let mut current_index = 0;
    let final_index = numnbers.len() - 1;

    while current_index < final_index {
        println!("while loop: {}", numnbers[current_index]);
        current_index += 1;
    }

    for n in numnbers.iter() {
        println!("for loop: {}", n);
    }

    loop {
        if current_index > final_index {
            break;
        }

        println!("loop: {}", numnbers[current_index]);

        current_index += 1;
    }
    println!("done");
}
