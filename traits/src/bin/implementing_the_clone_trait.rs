use std::clone::Clone;
#[derive(Clone, Debug)]
struct Appoitment {
    doctor: String,
    start_time: String,
    end_time: String,
}
impl Appoitment {
    fn new(doctor: &str, start_time: &str, end_time: &str) -> Self {
        Self {
            doctor: doctor.to_string(),
            start_time: start_time.to_string(),
            end_time: end_time.to_string(),
        }
    }
}
/*impl Clone for Appoitment{
    fn clone(&self) -> Self {
        Self {
            doctor: self.doctor.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
        }
    }
}*/
/// The Clone trait models the ability to create a duplicate of an instance.
fn main() {
    let morning_appt = Appoitment::new("Dr. Andrews", "9:00AM", "10:00AM");
    let replacement_appt = morning_appt.clone();
    println!(
        "{} is the seeing the patient from {} to {}",
        replacement_appt.doctor, replacement_appt.start_time, replacement_appt.end_time
    );
    println!("{:#?}", morning_appt);
}
