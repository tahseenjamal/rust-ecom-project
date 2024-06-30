use ecom::Category;
use ecom::Customer;
use ecom::Discount;
use ecom::Order;
use ecom::Product;
use ecom::Tax;

fn main() {
    let tahseen: Customer = Customer::new("Tahseen", 48, "tahseen.jamal@gmail.com", "India");
    println!("Name: {}", tahseen.get_name());

    let product1: Product = Product::new(1, "Laptop", 500.0, Category::Electronics, Tax::VAT);
    let product2: Product = Product::new(2, "Shampoo", 20.0, Category::Cosmetics, Tax::GST);

    let mut order: Order = Order::new(1, vec![product1, product2], Discount::Medium, tahseen);

    println!("Total products: {}", order.get_total_products());
    println!("Total products: {}", order.get_total());

    let product3 = Product::new(2, "Shampoo", 20.0, Category::Cosmetics, Tax::GST);
    order.add_product(product3);

    println!("Total products: {}", order.get_total_products());
    println!("Total products: {}", order.get_total());
    println!("Product count: {}", order.product_wise_count());
}
