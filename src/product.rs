pub use category::Category;
pub use tax::Tax;

pub struct Product {
    id: u32,
    name: String,
    price: f64,
    category: Category,
    tax: Tax,
}
impl Product {
    pub fn new(id: u32, name: &str, price: f64, category: Category, tax: Tax) -> Product {
        Product {
            id,
            name: name.to_string(),
            price,
            category,
            tax,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_category(&self) -> &Category {
        &self.category
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_price(&self) -> f64 {
        match self.tax {
            Tax::VAT => self.price * 1.2,
            Tax::GST => self.price * 1.1,
        }
    }
}

mod category;
mod tax;
