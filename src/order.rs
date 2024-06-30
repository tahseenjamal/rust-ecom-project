use crate::customer::Customer;
use crate::product::Product;
use std::collections::HashMap;
use uuid::Uuid;

// Making Discount public even when module discount is private. And now that discount is at public level to order,
// we don't have to use discount module in path, this is useful if you don't want to expose other
// items inside discount module. Note discount is inside Order, so first in the order file we have
// to use name space to access Discount, like Order::Discount and also make it public
//
pub use discount::Discount;

mod discount;

pub struct Order {
    id: String,
    products_list: Vec<Product>,
    products_unique_id_map: HashMap<u32, u32>,
    discount: Discount,
    customer: Customer,
}

impl Order {
    pub fn new(customer: Customer) -> Order {
        Order {
            id: Uuid::new_v4().to_string(),
            products_list: Vec::new(),
            products_unique_id_map: HashMap::new(),
            discount: Discount::None,
            customer,
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let count = self
            .products_unique_id_map
            .entry(product.get_id())
            .or_insert(0);
        *count += 1;
        self.products_list.push(product);
    }

    pub fn get_total_unique_products(&self) -> usize {
        self.products_unique_id_map.len()
    }
    pub fn get_total_products(&self) -> usize {
        self.products_list.len()
    }

    pub fn set_discount(&mut self, discount: Discount) {
        self.discount = discount;
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

    pub fn get_total_price(&self) -> f64 {
        self.products_list
            .iter()
            .map(|product| product.get_price())
            .sum::<f64>()
            * (1.0 - self.get_discount())
    }
}
