use crate::customer::Customer;
use crate::product::Product;

// Making Discount public even when module discount is private. And now that discount is at public level to order,
// we don't have to use discount module in path, this is useful if you don't want to expose other
// items inside discount module. Note discount is inside Order, so first in the order file we have
// to use name space to access Discount, like Order::Discount and also make it public
//
pub use discount::Discount;

mod discount;

pub struct Order {
    id: u32,
    products: Vec<Product>, // This should be a hashmap with product id as key and count as value
    discount: Discount,
    customer: Customer,
}

impl Order {
    pub fn new(id: u32, products: Vec<Product>, discount: Discount, customer: Customer) -> Order {
        Order {
            id,
            products,
            discount,
            customer,
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    pub fn get_total_products(&self) -> usize {
        self.products.len()
    }
    pub fn product_wise_count(&self) -> usize {
        //find total unique product ids
        let mut unique_product_ids: Vec<u32> = Vec::new();
        for product in &self.products {
            if !unique_product_ids.contains(&product.get_id()) {
                unique_product_ids.push(product.get_id());
            }
        }
        unique_product_ids.len()
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
