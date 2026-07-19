trait Invesment<T> {
    fn amount(&self) -> T;

    fn double_amount(&mut self);
}
trait Taxable: Invesment<f64> {
    const TAX_RATE: f64 = 0.25; // associated constant
    //get function
    fn tax_bill(&self) -> f64 {
        self.amount() * Self::TAX_RATE
    }
}
#[derive(Debug)]
struct Income {
    amount: f64,
}
impl Invesment<f64> for Income {
    fn amount(&self) -> f64 {
        self.amount
    }
    fn double_amount(&mut self) {
        self.amount *= 2.0;
    }
}
impl Taxable for Income {}
#[derive(Debug)]
struct Bonnus {
    value: f64,
}

impl Invesment<f64> for Bonnus {
    fn amount(&self) -> f64 {
        self.value
    }
    fn double_amount(&mut self) {
        self.value *= 2.0;
    }
}
impl Taxable for Bonnus {
    const TAX_RATE: f64 = 0.50;
}

struct QualiityTime {
    minutes: u32,
}
impl Invesment<u32> for QualiityTime {
    fn amount(&self) -> u32 {
        self.minutes
    }
    fn double_amount(&mut self) {
        self.minutes *= 2;
    }
}
fn main() {
    let mut income = Income { amount: 5000.50 };
    let mut bonus = Bonnus { value: 10000.23 };
    let mut rust_programinig_time = QualiityTime { minutes: 1000 };
    income.double_amount();
    bonus.double_amount();
    rust_programinig_time.double_amount();
    println!("Total tax owned: ${:.2}", income.tax_bill());
    println!("Bonus tax owned: ${:.2}", bonus.tax_bill());
    println!(
        "Total quality time: {} minutes",
        rust_programinig_time.amount()
    );
}
