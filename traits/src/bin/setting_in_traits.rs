trait Taxable {
    const TAX_RATE: f64 = 0.25; // associated constant
    //get function
    fn amount(&self) -> f64;

    fn set_amount(&mut self, amount: f64);

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }
    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

struct Income {
    amount: f64,
}
impl Taxable for Income {
    fn amount(&self) -> f64 {
        self.amount
    }
    fn set_amount(&mut self, amount: f64) {
        self.amount = amount;
    }
}
#[derive(Debug)]
struct Bonnus {
    value: f64,
}
impl Taxable for Bonnus {
    const TAX_RATE: f64 = 0.50;
    fn amount(&self) -> f64 {
        self.value
    }
    fn set_amount(&mut self, amount: f64) {
        self.value = amount;
    }
}
fn main() {
    let mut income = Income { amount: 5000.50 };
    println!("Total tax owned: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owned: ${:.2}", income.tax_bill());

    let mut bonus = Bonnus { value: 10000.23 };
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
}
