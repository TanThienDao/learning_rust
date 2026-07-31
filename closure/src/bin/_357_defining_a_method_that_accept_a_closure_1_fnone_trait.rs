use std::io::stdin;

#[derive(Debug)]
struct Vault {
    password: String,
    treasure: String,
}
impl Vault {
    fn unlock<F>(self, procedure: F) -> Option<String>
    where
        F: FnOnce() -> String,
    {
        let user_password = procedure();
        if user_password == self.password {
            Some(self.treasure)
        } else {
            None
        }
    }
}
fn main() {
    let vault = Vault {
        password: "1234".to_string(),
        treasure: "Gold".to_string(),
    };

    let hack = || {
        let mut user_input = String::new();
        println!("Please provide password to crack the vault");
        stdin().read_line(&mut user_input);
        user_input.trim().to_string()
    };
    let extraction = vault.unlock(hack);
    println!("{:?}", extraction);
}
