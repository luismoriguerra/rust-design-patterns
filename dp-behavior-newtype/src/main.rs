use std::fmt::Display;

struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "********")
    }
}

fn main() {
    println!("Newtype pattern example");

    let unsecure_password = "Thisismypassword".to_string();
    let secure_password = Password(unsecure_password.clone());

    println!("Unsecure password: {}", unsecure_password);
    println!("Secure password: {}", secure_password.0);
}
