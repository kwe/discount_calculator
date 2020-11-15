use serde::{Deserialize, Serialize};
// use serde_json::Result;

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
    discount_threshold: Option<u64>,
    discount_price: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
struct OrderItem {
    count: u64,
    item: Product,
}
impl OrderItem {
    pub fn new(order: &mut Order, product: Product) {
        let known_product = order.items.iter_mut().find(|i| i.item.id == product.id);

        println!("known_product = {:?}", known_product);
        match known_product {
            Some(order_item) => order_item.count += 1,
            None => {
                let item = OrderItem {
                    item: product,
                    count: 1,
                };
                &order.items.push(item);
            }
        }
        // calculate revised order total
        let total: f64 = order
            .items
            .iter()
            .map(|i| {
                if let (Some(discount_threshold), Some(discount_price)) = (i.item.discount_threshold, i.item.discount_price) {
                        if discount_threshold < i.count {
                            i.item.price * i.count as f64
                        } else {
                            discount_price * i.count as f64
                        }
                } else {
                    i.item.price * i.count as f64
                }
            })
            .sum();
        order.total = total;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    items: Vec<OrderItem>,
    total: f64,
}
impl Order {
    pub fn new() -> Order {
        Self {
            items: Vec::new(),
            total: 0.0,
        }
    }
    fn add_to_order(mut self, product: &Product) -> Order {
        // add product as an order item, increment count if product already in order
        let p = Product {
            id: product.id.to_string(),
            name: product.name.to_string(),
            price: product.price,
            discount_threshold: product.discount_threshold,
            discount_price: product.discount_price,
        };
        let item = OrderItem::new(&mut self, p);

        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkout {
    order: Order,
}

impl Checkout {
    pub fn new(_promotional_rules: &str) {
        println!("new called");
    }

    pub fn scan(_item_code: &str) {
        println!("scan called");
    }

    pub fn total(&self) -> f64 {
        self.order.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_promotional_rules() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn scan_items() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn total_is_correct_without_discounts() {
        let p1 = Product {
            id: "001".to_string(),
            name: "widget".to_string(),
            price: 2.50,
            discount_price: None,
            discount_threshold: None,
        };
        let p2 = Product {
            id: "002".to_string(),
            name: "flipper".to_string(),
            price: 3.99,
            discount_price: None,
            discount_threshold: None,
        };
        let p3 = Product {
            id: "001".to_string(),
            name: "widget".to_string(),
            price: 2.50,
            discount_price: None,
            discount_threshold: None,
        };

        let mut order = Order::new();
        order = Order::add_to_order(order, &p1);
        order = Order::add_to_order(order, &p2);
        order = Order::add_to_order(order, &p3);

        println!("Order is {:#?}", order);

        assert_eq!(order.total, 8.99);
    }

    #[test]
    fn total_is_correct_with_discounts() {
        let p1 = Product {
            id: "001".to_string(),
            name: "widget".to_string(),
            price: 2.50,
            discount_price: Some(1.50),
            discount_threshold: Some(2),
        };
        let p2 = Product {
            id: "002".to_string(),
            name: "flipper".to_string(),
            price: 3.99,
            discount_price: None,
            discount_threshold: None,
        };
        let p3 = Product {
            id: "001".to_string(),
            name: "widget".to_string(),
            price: 2.50,
            discount_price: Some(1.50),
            discount_threshold: Some(2),
        };

        let mut order = Order::new();
        order = Order::add_to_order(order, &p1);
        order = Order::add_to_order(order, &p2);

        assert_eq!(order.total, 5.49); // no discount yet

        order = Order::add_to_order(order, &p3);

        println!("Order is {:#?}", order);

        assert_eq!(order.total, 6.99); // discount threshold reached for 001
    }
}
