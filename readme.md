# Challenge

This repository is a possible solution for this [challenge](https://github.com/Syft-Application/coding_challenge/blob/master/checkout.md).

The aim is to simulate an online marketplace, which allows for promotions and total spending discounts.

## Assumptions
I assumed that a CMS of some type would allow the marketing department to easily create promotions and discounts. 
It was assumed for this exercise that this data would be made available as a valid json endpoint, modelled in this
application as a static string for demonstration purposes. 

### Sample promotion_rules

```json
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
```

## Approach
As specified, the application assumes the promotional rules are used to initialise the checkout object. Items are 'scanned' from a basket and added to a view model called 'Order'. This collates the scanned products and records the number of unique items. If the rules specify a discount level has been reached, the order total takes into account a products' discount price. On completion the checkout object returns the order total, while taking into account any percentage discount specified in the rules json.

## How to run this application
If you have a local install of [Docker](https://www.docker.com/products/docker-desktop), you can use the supplied Dockerfile to run a local copy of the project without needing to install Rust.

for example:

```bash
docker build . -t discounter
```
This will build the project and also run its test suite.

```bash
docker run discounter
```
Will run the application, with the test data specified in the challenge document

### Output
```bash
Test data
-----------
Basket: ["001", "002", "003"]
£66.78
Basket: ["001", "003", "001"]
£36.95
Basket: ["001", "002", "001", "003"]
£73.76

Once more for luck, randomise the last basket order

Basket: ["001", "001", "002", "003"]
£73.76 which is equal to £73.76
```

*nb: the final test randomises the scanning order as mentioned in the challenge.*

## Rust notes
I took the words 'challenge' and 'modern language' literally and ended up with this my first Rust application. The 'checkout' functionality is developed as a testable library (in the shop folder). With a local install of Rust one can run the library tests with...

```bash
cd shop
cargo test
```

This will output...

```bash
running 4 tests
test tests::total_is_correct_without_discounts ... ok
test tests::read_in_rules ... ok
test tests::check_total_order_applies_discount ... ok
test tests::scan_works_correctly ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
