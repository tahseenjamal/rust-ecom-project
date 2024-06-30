use ecom::Category;
use ecom::Customer;
use ecom::Discount;
use ecom::Order;
use ecom::Product;
use ecom::Tax;

fn main() {
    let tahseen: Customer = Customer::new("Tahseen", 48, "tahseen.jamal@gmail.com", "India");

    let product1: Product = Product::new(1, "Laptop", 500.0, Category::Electronics, Tax::VAT);
    let product2: Product = Product::new(2, "Shampoo", 20.0, Category::Cosmetics, Tax::GST);
    let product3 = Product::new(2, "Shampoo", 20.0, Category::Cosmetics, Tax::GST);

    let mut order: Order = Order::new(tahseen);

    order.add_product(product1);
    order.add_product(product2);
    order.add_product(product3);

    order.set_discount(Discount::Medium);

    println!(
        "Total unique products: {}",
        order.get_total_unique_products()
    );
    println!("Total products: {}", order.get_total_products());
    println!("Total cost: {}", order.get_total_price());
}
