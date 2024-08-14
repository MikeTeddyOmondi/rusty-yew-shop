use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CheckoutMethod {
    Mpesa,
    #[serde(rename = "CARD-PAYMENT")]
    CardPayment,
    Bitcoin,
    #[serde(rename = "BANK-ACH")]
    Bank,
    #[serde(rename = "COOP_B2B")]
    CoopB2b,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    /// Kenya Shillings
    #[serde(rename = "KES")]
    Kes,
    /// US Dollars
    #[serde(rename = "USD")]
    Usd,
    /// Euros
    #[serde(rename = "EUR")]
    Eur,
    /// British Pounds
    #[serde(rename = "GBP")]
    Gbp,
}

// pub struct Product {
//     id: u32,
//     name: String,
//     description: Option<String>,
//     price: f64,
//     image_url: String,
// }

#[derive(Properties, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: &'static str,
    pub description: Option<&'static str>,
    pub price: f64,
    pub image_url: &'static str,
}

pub const PRODUCTS: &[Product] = &[
    Product {
        id: 1,
        name: "Amazing Widget",
        description: Some("This widget will change your life"),
        price: 19.99,
        image_url: "http://localhost:9003/soko247/ecommerce_products/watch-1.jpg",
    },
    Product {
        id: 2,
        name: "Super Gadget",
        description: None,
        price: 29.99,
        image_url: "http://localhost:9003/soko247/ecommerce_products/watch-2.jpg",
    },
    Product {
        id: 3,
        name: "Fantastic Doohickey",
        description: Some("You never knew you needed this"),
        price: 69.99,
        image_url: "http://localhost:9003/soko247/ecommerce_products/watch-3.jpg",
    },
    Product {
        id: 1,
        name: "Amazing Widget",
        description: Some("This widget will change your life"),
        price: 19.99,
        image_url: "http://localhost:9003/soko247/ecommerce_products/watch-4.jpg",
    },
    Product {
        id: 2,
        name: "Super Gadget",
        description: None,
        price: 29.99,
        image_url: "http://localhost:9003/soko247/ecommerce_products/watch-5.jpg",
    },
    Product {
        id: 3,
        name: "Fantastic Doohickey",
        description: Some("You never knew you needed this"),
        price: 69.99,
        image_url: "http://localhost:9003/soko247/ecommerce_products/watch-6.jpg",
    },
];

pub fn get_products() -> &'static [self::Product] {
    PRODUCTS
}