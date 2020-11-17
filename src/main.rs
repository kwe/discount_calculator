use shop;

fn main() {
    let promotional_rules = r#"
{
    "version" : 1,
    "total_discount_threshold": 60.00,
    "total_discount_percentage": 10.00,
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

    println!("Test data\n-----------");

    let mut basket = vec!["001", "002", "003"];
    println!("Basket: {:?}", basket);

    let mut co = shop::Checkout::new(promotional_rules);
    // checkout::scan("001");
    // checkout::scan("003");

    basket = vec!["001", "002", "003"];
    println!("Basket: {:#?}", basket);

    for item in basket {
        co.scan(item)
    }

    println!("{:#?}", co.total());

    let mut co = shop::Checkout::new(promotional_rules);

    basket = vec!["001", "003", "001"];
    println!("Basket: {:#?}", basket);

    for item in basket.iter() {
        co.scan(item);
    }
    println!("{:#?}", co.total());

    let mut co = shop::Checkout::new(promotional_rules);

    basket = vec!["001", "002", "001", "003"];
    println!("Basket: {:#?}", basket);

    for item in basket.iter() {
        co.scan(item);
    }
    println!("{:#?}", co.total());
}
