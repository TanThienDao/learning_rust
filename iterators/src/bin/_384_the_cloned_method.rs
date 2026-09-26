/// The copied method convert an iterator
/// from storing &T elements to T elements.
/// It make a copy of each T element.
/// The T data type must implement the Copy trait.
///
/// The cloned method similarly converts an
/// iterator from stroring &T elements to T elements.
/// It makes a clone of each T element.
/// The T data type must implement the Clone trait.
/// -> cloned is heap base data.
fn main() {
    let teas = [
        String::from("ProgrammingTutorials"),
        String::from("Iced Green Tea"),
        String::from("Hot Macha Tea"),
    ];
    // iterator has &String --> String
    let more_teas: Vec<String> = teas.clone().into_iter().collect();
    println!("{:?}", more_teas);

    let more_teas1: Vec<String> = teas.iter().map(|t| t.clone()).collect();
    println!("{:?}", more_teas1);

    let more_teas2: Vec<String> = teas
        .iter()
        .filter(|tea| tea.contains("Tea"))
        .cloned()
        .collect();
    println!("{:?}", more_teas2);

    let more_teas3: Vec<&String> = teas
        .iter()
        .clone()
        .filter(|tea| tea.contains("Tea"))
        .collect();
    println!("{:?}", more_teas3);
}
