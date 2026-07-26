///Third Elision Rule: in a method definition, if there are multiple reference
/// parameters but one of them is self, the borrow checker will assume the lifetime
/// of the instance is connected to the lifetime of the return value.

struct DentistAppoitment {
    docter: String,
}

impl DentistAppoitment {
    fn book<'a>(&self, check_in_time: &'a str, check_out_time: &str) -> &'a str {
        println!(
            "Your book from {} to {} with docter {}",
            check_in_time, check_out_time, self.docter
        );
        //&self.docter
        check_in_time
    }
}
fn main() {
    let appt = DentistAppoitment {
        docter: String::from("David"),
    };
    let result = appt.book("10:00", "11:00");
    drop(appt);
    println!("{}", result);
}
