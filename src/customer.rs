pub struct Customer {
    name: String,
    age: u8,
    email: String,
    address: String,
}

impl Customer {
    pub fn new(name: &str, age: u8, email: &str, address: &str) -> Customer {
        Customer {
            name: name.to_string(),
            age,
            email: email.to_string(),
            address: address.to_string(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_customer() {
        let tahseen: Customer = Customer::new("Tahseen", 48, "tahseen.jamal@gmail.com", "India");
    }
}
