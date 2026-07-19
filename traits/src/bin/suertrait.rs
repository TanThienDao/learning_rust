trait Invesment {
    fn amount(&self) -> f64;

    fn set_amount(&mut self, amount: f64);

    fn double_amount(&mut self) {
        self.set_amount(self.amount() * 2.0);
    }
}
trait Taxable: Invesment {
    const TAX_RATE: f64 = 0.25; // associated constant
    //get function
    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}

struct Income {
    amount: f64,
}
impl Invesment for Income {
    fn amount(&self) -> f64 {
        self.amount
    }
    fn set_amount(&mut self, amount: f64) {
        self.amount = amount;
    }
}
impl Taxable for Income {}
#[derive(Debug)]
struct Bonnus {
    value: f64,
}

impl Invesment for Bonnus {
    fn amount(&self) -> f64 {
        self.value
    }
    fn set_amount(&mut self, amount: f64) {
        self.value = amount;
    }
}
impl Taxable for Bonnus {
    const TAX_RATE: f64 = 0.50;
}

struct QualiityTime {
    minutes: f64,
}
impl Invesment for QualiityTime {
    fn amount(&self) -> f64 {
        self.minutes
    }
    fn set_amount(&mut self, amount: f64) {
        self.minutes = amount;
    }
}
fn main() {
    /// A supertrait is a trait from which another trait inherits functionality.
    /// The parrent is called the supertrait and the child is called the subtrait.
    let mut income = Income { amount: 5000.50 };
    println!("Total tax owned: ${:.2}", income.tax_bill());
    income.double_amount();
    println!("Total tax owned: ${:.2}", income.tax_bill());

    let mut bonus = Bonnus { value: 10000.23 };
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
    bonus.double_amount();
    println!("Bonus tax owned: ${:.3}", bonus.tax_bill());

    let weekend = QualiityTime { minutes: 120.0 };
    println!("Total quality time: {} minutes", weekend.amount());
}
