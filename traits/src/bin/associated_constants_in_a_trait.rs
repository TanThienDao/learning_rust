/// An Associated constant is a constant declared within the trait.
/// A constant is a name for a fixed, immutable value.

trait Taxable {
    const TAX_RATE: f64 = 0.25; // associated constant
    fn tax_bill(&self) -> f64;
}

struct Income {
    amount: f64,
}
impl Taxable for Income {
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}
#[derive(Debug)]
struct Bonnus {
    amount: f64,
}
impl Taxable for Bonnus {
    const TAX_RATE: f64 = 0.50;
    fn tax_bill(&self) -> f64 {
        self.amount * Self::TAX_RATE
    }
}
fn main() {
    let income = Income { amount: 5000.50 };
    println!("Total tax owned: ${:.2}", income.tax_bill());

    let bonus = Bonnus { amount: 10000.23 };
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
}
