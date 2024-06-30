pub use category::Category;
pub use tax::Tax;

mod category;
mod tax;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product() {
        let product = Product::new(1, "Product 1", 100.0, Category::Electronics, Tax::VAT);
        assert_eq!(product.get_id(), 1);
        assert_eq!(product.get_name(), "Product 1");
        assert_eq!(product.get_category(), &Category::Electronics);
        assert_eq!(product.get_price(), 120.0);
    }
}
