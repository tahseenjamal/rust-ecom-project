use crate::customer::Customer;
use crate::product::Product;
use discount::Discount;

pub struct Order {
    id: u32,
    products: Vec<Product>,
    discount: Discount,
    customer: Customer,
}

pub mod discount;

impl Order {
    pub fn new(id: u32, products: Vec<Product>, discount: Discount, customer: Customer) -> Order {
        Order {
            id,
            products,
            discount,
            customer,
        }
    }

    pub fn get_total_products(&self) -> usize {
        self.products.len()
    }

    fn get_discount(&self) -> f64 {
        match self.discount {
            Discount::None => 0.0,
            Discount::Low => 0.05,
            Discount::Medium => 0.1,
            Discount::High => 0.15,
            Discount::Deal => 0.2,
        }
    }

    pub fn get_total(&self) -> f64 {
        self.products.iter().map(|p| p.get_price()).sum::<f64>() * (1.0 - self.get_discount())
    }
}
