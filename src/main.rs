use ecom::customer::Customer;
use ecom::order::discount::Discount;
use ecom::order::Order;
use ecom::product::category::Category;
use ecom::product::tax::Tax;
use ecom::product::Product;

fn main() {
    let tahseen: Customer = Customer::new("Tahseen", 48, "tahseen.jamal@gmail.com", "India");
    println!("Name: {}", tahseen.get_name());

    let product1: Product = Product::new("Laptop", 500.0, Category::Electronics, Tax::VAT);
    let product2: Product = Product::new("Toothpaste", 1.0, Category::Cosmetics, Tax::GST);

    let order: Order = Order::new(1, vec![product1, product2], Discount::Medium, tahseen);

    println!("Total products: {}", order.get_total_products());
    println!("Total products: {}", order.get_total());
}
