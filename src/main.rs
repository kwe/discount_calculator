use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    id: i8,
    title: String,
    price: f64,
    discount_threshold: i8,
    discount_price: f64,
}


fn main() {
    println!("Hello, world!");
    let _r = typed_example();
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "id":1,
            "title":"Widgets",
            "price": 6.50,
            "discount_threshold" : 4,
            "discount_price" : 5.00
        }"#;

    let p: Product = serde_json::from_str(data)?;

    println!("p is {:?}",p);

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.title, p.price);

    Ok(())
}