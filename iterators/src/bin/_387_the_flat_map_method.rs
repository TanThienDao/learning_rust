fn main() {
    let attendees = ["Bob,Marry,Keven", "Mike,Robbie,Matt,Austin", "Piers,Liam"];

    let attendees_2: Vec<&str> = attendees
        .iter()
        .map(|group| group.split(','))
        .flatten()
        .collect();
    println!("{:?}", attendees_2);

    let attendees_3: Vec<&str> = attendees
        .iter()
        .flat_map(|group| group.split(','))
        .collect();
    println!("{:?}", attendees_3);
}
