use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromotionalRules {
    version: i64,
    total_discount_threshold: f64,
    total_discount_percentage: f64,
    products: Vec<Product>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    id: String,
    name: String,
    price: f64,
    discount_threshold: f64,
    discount_price: f64,
}
#[derive(Debug, Serialize, Deserialize)]
struct OrderItem {
    count: i64,
    item: Product,
}
impl OrderItem {
    pub fn new(order: &mut Order, product: Product) {
        //find out if it's a product that's already ordered
        let known_product = order.items.iter_mut().find(|i| i.item.id == product.id);

        match known_product {
            Some(order_item) => order_item.count += 1,
            None => {
                let item = OrderItem {
                    item: product,
                    count: 1,
                };
                // it's not a product already ordered, add it to the order.
                order.items.push(item);
            }
        }
        // calculate revised order total
        let total: f64 = order
            .items
            .iter()
            .map(|i| {
                OrderItem::calculate_cost(i)
            })
            .sum();
        order.total = total;
    }

    fn calculate_cost(product : &OrderItem) -> f64 {
        if product.item.discount_threshold != 0.0_f64 {
            if product.count as f64  >= product.item.discount_threshold {
                return product.count as f64 * product.item.discount_price
            }
        } 
        product.count as f64 * product.item.price
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
    rules: PromotionalRules,
}

impl Checkout {
    pub fn new(promotional_rules: &str) -> Checkout {
        let rules: PromotionalRules = serde_json::from_str(promotional_rules).unwrap();
        let order = Order::new();

        Self { order, rules }
    }

    pub fn scan(&mut self, item_code: &str) {
        // find product details in loaded rules
        let known_product = self.rules.products.iter().find(|p| p.id == item_code);
        match known_product {
            Some(product) => {
                self.order.add_to_order(product);
            }
            None => {
                unimplemented!() // assumption that only valid products for this excercise
            }
        };
    }

    pub fn total(&self) -> f64 {
        // is there a total discount available?
        if self.rules.total_discount_threshold > 0.0 {
            // has the total reached or exceeded the threshold to apply a discount?
            if self.order.total >= self.rules.total_discount_threshold {
                // take off the discount percentage
                let amount = self.order.total - (self.order.total * (self.rules.total_discount_percentage / 100.00));
                return (amount * 100.00).round() / 100.00; // round to 2 decimal places.
            } else {
                // not discount on total
                return (self.order.total * 100.00).round() / 100.00; // round to two decimal places
            }
        } 
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
                    "discount_threshold":2.0,
                    "discount_price": 8.50
                },
                        {
                    "id":"002",
                    "name":"Personalised cufflinks",
                    "price":45.00,
                    "discount_threshold":0.0,
                    "discount_price": 0.0
                },
                        {
                    "id":"003",
                    "name":"Kids T-shirt ",
                    "price":19.95,
                    "discount_threshold":0.0,
                    "discount_price": 0.0
                }
            ]
        }
        "#;
        let co = Checkout::new(rules);

        // let's test Checkout is valid
        assert_eq!(co.rules.version, 1);
        assert_eq!(co.rules.products.len(), 3);
        assert_eq!(co.rules.products[0].name, "Lavender heart");
        assert_eq!(co.order.total, 0.0);
    }
    #[test]
    fn scan_works_correctly() {
        let rules = r#"
        {
            "version" : 1,
            "total_discount_threshold": 70.00,
            "total_discount_percentage": 10.00,
            "products": [
                {
                    "id":"001",
                    "name":"Lavender heart",
                    "price":19.25,
                    "discount_threshold":2,
                    "discount_price": 9.25
                },
                        {
                    "id":"002",
                    "name":"Personalised cufflinks",
                    "price":25.00,
                    "discount_threshold":0,
                    "discount_price": 0
                },
                        {
                    "id":"003",
                    "name":"Kids T-shirt ",
                    "price":19.95,
                    "discount_threshold":0,
                    "discount_price": 0
                }
            ]
        }
        "#;
        let mut co: Checkout = Checkout::new(rules);
        co.scan("002");
        assert_eq!(co.order.items.len(), 1);
        assert_eq!(co.total(), 25.00); 

        co.scan("003"); // Check total is incrementing
        assert_eq!(co.order.items.len(), 2);
        assert_eq!(co.total(), 44.95);

        co.scan("001");
        assert_eq!(co.order.items.len(), 3);

        assert_eq!(co.total(), 64.20);

        // test discount is applied for two 001

        co.scan("001");
        assert_eq!(co.order.items.len(), 3); 
        assert_eq!(co.total(), 63.45);

    }
    #[test]
    fn check_total_order_applies_discount() {
        let rules = r#"
        {
            "version" : 1,
            "total_discount_threshold": 18.00,
            "total_discount_percentage": 10.00,
            "products": [
                {
                    "id":"001",
                    "name":"Lavender heart",
                    "price":9.25,
                    "discount_threshold": 2.0,
                    "discount_price": 8.50
                },
                {
                    "id":"002",
                    "name":"Personalised cufflinks",
                    "price":4.99,
                    "discount_threshold":0,
                    "discount_price": 0
                },
                        {
                    "id":"003",
                    "name":"Kids T-shirt ",
                    "price":19.25,
                    "discount_threshold":0,
                    "discount_price": 0
                }
            ]
        }
        "#;
        let mut co: Checkout = Checkout::new(rules);
        co.scan("001");

        assert_eq!(co.order.items.len(), 1);
        assert_eq!(co.total(), 9.25); // test that 1 item is not discounted

        co.scan("002");
         assert_eq!(co.order.items.len(), 2);       
        assert_eq!(co.total(), 14.24);
        
        co.scan("003");
        assert_eq!(co.order.items.len(), 3); // now have two products in the order
        assert_eq!(co.total(), 30.14);
    }
    #[test]
    fn total_is_correct_without_discounts() {
        let p1 = Product {
            id: "001".to_string(),
            name: "widget".to_string(),
            price: 2.50,
            discount_price: 0.0,
            discount_threshold: 0.0,
        };
        let p2 = Product {
            id: "002".to_string(),
            name: "flipper".to_string(),
            price: 3.99,
            discount_price: 0.0,
            discount_threshold: 0.0,
        };
        let p3 = Product {
            id: "003".to_string(),
            name: "shoe".to_string(),
            price: 7.99,
            discount_price: 0.0,
            discount_threshold: 0.0,
        };

        let mut order = Order::new();
        order.add_to_order(&p1);
        order.add_to_order(&p2);
        order.add_to_order(&p3);

        assert_eq!(order.items.len(), 3); //check there are three order items
        assert_eq!(order.items[0].count, 1); // ensure it's recorded one of 001  scanned
        assert_eq!(order.items[0].item.id, "001".to_string()); // check it's recorded the item

        assert_eq!(order.total, 14.48);
    }
}
