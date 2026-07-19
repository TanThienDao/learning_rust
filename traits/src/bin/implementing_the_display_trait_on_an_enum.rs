use std::fmt::{Display, Formatter, Result};

enum AppleType {
    RedDelicious,
    GrannySmith,
}

impl Display for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(f, "Red Delicious"),
            AppleType::GrannySmith => write!(f, "Granny Smith"),
        }
    }
}
struct Apple {
    kind: AppleType,
    price: f64,
}
impl Display for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Kind: {}, Price: ${}", self.kind, self.price)
    }
}
fn main() {
    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };
    println!("{}", lunch_snack);

    let dinner_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.15,
    };
    println!("{}", dinner_snack);
}
