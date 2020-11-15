
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionalRules {
    version: u32,
    total_discount_threshold: f32,
    total_discount_percentage: u8,
    products: Vec<Product>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    id: String,
    name: String,
    price: f64,
    discount_threshold: i8,
    discount_price: f64
}
#[derive(Debug, Serialize, Deserialize)]
struct OrderItem {
    count: u8,
    item: Product
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    items: Vec<OrderItem>,
    total: f32
}


pub fn hello() {
  println!("Hello from a library");
}

pub fn new(promotional_rules: &str){
    unimplemented!();
}

pub fn scan(item_code: &str){
    unimplemented!();
}

pub fn total() -> f32 {
    4.50
}


#[cfg(test)]
mod tests {
    #[test]
    fn say_hello() {
        assert_eq!(2 + 2, 4);
    }
}
