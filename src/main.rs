use shop;
use rand::seq::SliceRandom;

fn main() {
    // define promotional rules as a static string. Normally this would come from
    // dynamic endpoint such as a REST/Graphql endpoint.
    
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

    // Create a Vecor of items in a 'basket'
    let mut basket = vec!["001", "002", "003"];
    println!("Basket: {:?}", basket);

    // Create a new Checkout
    let mut co = shop::Checkout::new(promotional_rules);
    // loop through the basket, scanning items
    for item in basket {
        co.scan(item)
    }
    // print out the total
    println!("£{:#?}", co.total());

    // and repeat...
    let mut co = shop::Checkout::new(promotional_rules);

    basket = vec!["001", "003", "001"];
    println!("Basket: {:?}", basket);

    for item in basket.iter() {
        co.scan(item);
    }
    println!("£{:?}", co.total());

    let mut co = shop::Checkout::new(promotional_rules);

    let mut basket = vec!["001", "002", "001", "003"];
    println!("Basket: {:?}", basket);

    for item in basket.iter() {
        co.scan(item);
    }
    println!("£{:?}", co.total());

    let temp_total = co.total();

    // randomise the basket

    let mut rng = rand::thread_rng();
    basket.shuffle(&mut rng);

    let mut co = shop::Checkout::new(promotional_rules);

    println!("\nOnce more for luck, randomise the last basket order\n\nBasket: {:?}", basket);
    for item in basket.iter() {
        co.scan(item);
    }
    println!("£{:?} which is equal to £{:?}", co.total(), temp_total);
}