#[derive(Debug, Clone)]
struct Duration {
    hour: u32,
    minute: u32,
    second: u32,
}
impl Duration {
    fn new(hour: u32, minute: u32, second: u32) -> Self {
        Self {
            hour,
            minute,
            second,
        }
    }
}
impl Copy for Duration {} // no need to implement any method for copy trait
/// The Copy trait is a subtrait of the Clone supertrait that we covered in the previous lesson.
fn main() {
    let one_hour = Duration::new(1, 0, 0);
    let another_hours = one_hour;

    print!("{:?}", one_hour);
}
