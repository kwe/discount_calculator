use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionalRules {
    version: i64,
    total_discount_threshold: f64,
    total_discount_percentage: i64,
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
                order.items.push(item);
            }
        }
        // calculate revised order total
        let total: f64 = order
            .items
            .iter_mut()
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
    fn add_to_order(&mut self, product: &Product) {
        // add product as an order item, increment count if product already in order
        let p = Product {
            id: product.id.to_string(),
            name: product.name.to_string(),
            price: product.price,
            discount_threshold: product.discount_threshold,
            discount_price: product.discount_price,
        };
        OrderItem::new(self, p);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Checkout {
    order: Order,
    rules: PromotionalRules
}

impl Checkout {
    pub fn new(promotional_rules: &str) -> Checkout {
        let rules: PromotionalRules = serde_json::from_str(promotional_rules).unwrap();
        let order = Order::new();

        Self {
            order,
            rules
        }

    }

    pub fn scan(&mut self, item_code: &str) {
        println!("scan called for {:#?}", item_code);
        // find product details in loaded rules

        let known_product = self.rules.products
                .iter()
                .find(|p| p.id == item_code);
        match known_product {
            Some(product) => {
                self.order.add_to_order(product);
            },
            None => {
                unimplemented!() // assumption that only valid products for this excercise 
            }
        };
        // add product to order
    }

    pub fn total(&self) -> f64 {
        self.order.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_in_rules() {
        let rules = r#"
        {
            "version" : 1,
            "total_discount_threshold": 60.00,
            "total_discount_percentage": 10,
            "products": [
                {
                    "id":"001",
                    "name":"Lavender heart",
                    "price":9.25,
                    "discount_threshold":2,
                    "discount_price": 8.50
                },
                        {
                    "id":"002",
                    "name":"Personalised cufflinks",
                    "price":45.00
                },
                        {
                    "id":"003",
                    "name":"Kids T-shirt ",
                    "price":19.95
                }
            ]
        }
        "#;
        let co = Checkout::new(rules);

        // let's test checkout is valid
        assert_eq!(co.rules.version, 1);
        assert_eq!(co.rules.products.len(),3);
        assert_eq!(co.rules.products[0].name,"Lavender heart");
        assert_eq!(co.order.total,0.0);
    }
    #[test]
    fn scan_adds_item_to_order() {
        let rules = r#"
        {
            "version" : 1,
            "total_discount_threshold": 60.00,
            "total_discount_percentage": 10,
            "products": [
                {
                    "id":"001",
                    "name":"Lavender heart",
                    "price":9.25,
                    "discount_threshold":2,
                    "discount_price": 8.50
                },
                        {
                    "id":"002",
                    "name":"Personalised cufflinks",
                    "price":45.00
                },
                        {
                    "id":"003",
                    "name":"Kids T-shirt ",
                    "price":19.95
                }
            ]
        }
        "#;
        let mut co: Checkout = Checkout::new(rules);
       
        co.scan("002");
        assert_eq!(co.order.items.len(),1);

        println!("co is {:#?}", co);

        assert_eq!(co.order.total, 45.00);

        assert_eq!(co.total(), 45.00);
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
        order.add_to_order(&p1);
        order.add_to_order(&p2);
        order.add_to_order(&p3);

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
        order.add_to_order(&p1);
        order.add_to_order(&p2);

        assert_eq!(order.total, 5.49); // no discount yet

        order.add_to_order(&p3);

        println!("Order is {:#?}", order);

        assert_eq!(order.total, 6.99); // discount threshold reached for 001
    }
}
