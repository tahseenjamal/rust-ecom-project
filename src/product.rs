use category::Category;
use tax::Tax;

pub struct Product {
    name: String,
    price: f64,
    category: Category,
    tax: Tax,
}
impl Product {
    pub fn new(name: &str, price: f64, category: Category, tax: Tax) -> Product {
        Product {
            name: name.to_string(),
            price,
            category,
            tax,
        }
    }

    pub fn get_price(&self) -> f64 {
        match self.tax {
            Tax::VAT => self.price * 1.2,
            Tax::GST => self.price * 1.1,
        }
    }
}

pub mod category;

pub mod tax;
