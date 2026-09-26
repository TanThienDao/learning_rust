/// The enumerate adapter transforms an iterator so that it
/// yields the index position along with the current element.
fn main() {
    let applicants = vec!["Rob", "Bob", "Cob", "Alex", "Tommy"];
    for (index, applicant) in applicants.iter().enumerate() {
        println!("Applicant #{} is {}", index + 1, applicant);
    }
    /*    let winner = &applicants
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index % 3 == 0)
        .map(|(_, applicant)| applicant)
        .collect::<Vec<&str>>();

    println!("Winner is {:?}", winner);*/

    let winner2 = applicants
        .into_iter()
        .enumerate()
        .filter_map(|(index, applicant)| {
            if index % 3 == 0 {
                Some(applicant)
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();
    println!("Winner2 is {:?}", winner2);
}
