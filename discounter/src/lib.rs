
use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    id: String,
    title: String,
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
