/// An associated type us a placeholder for a type that is required within a trait.
///

use std::ops::Add;
#[derive(Debug)]
struct Lunch{
    cost: f64,
}
impl Add for Lunch{
/*    type Output = f64; // require add must return a f64
    fn add(self, rhs: Self) -> Self::Output {
        self.cost + rhs.cost
    }*/

    type Output = Lunch;
    fn add(self, rhs: Self) -> Self::Output {
        Lunch{cost: self.cost + rhs.cost}
    }

}
fn main() {
    let one = Lunch{cost: 19.997};
    let two = Lunch{cost: 20.9960};
    //println!("Total cost: {:.2}", one + two);
    println!("New Lunch {:?}", one + two);
}