use shop;

fn main() {

let promotional_rules = r#"
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

    println!("Test data\n-----------");

    let mut basket = vec!["001", "002", "003"];
    println!("Basket: {:?}", basket);

    let co = shop::Checkout::new(promotional_rules);
    // checkout::scan("001");
    // checkout::scan("003");

    basket = vec!["001","003","001"];
    println!("Basket: {:#?}", basket);
    
    basket = vec!["001","002","001", "003"];
    println!("Basket: {:#?}", basket);


}