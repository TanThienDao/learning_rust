use std::fmt::{Debug, Display, Formatter, Result};

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
impl Debug for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(f, "AppleType::RedDelicious"),
            AppleType::GrannySmith => write!(f, "AppleType::GrannySmith"),
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
impl Debug for Apple {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("** Apple **")
            .field("Kind", &self.kind)
            .field("Price", &self.price)
            .finish()
    }
}
fn main() {
    let lunch_snack = Apple {
        kind: AppleType::GrannySmith,
        price: 1.04,
    };
    println!("{:?}", lunch_snack);

    let dinner_snack = Apple {
        kind: AppleType::RedDelicious,
        price: 1.15,
    };
    println!("{:?}", dinner_snack);
}
