use std::env;
use std::rc::Rc;

use log::info;
use reqwest::Client;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use yew::{function_component, html, Callback, Html};
use yew_router::hooks::use_navigator;

use crate::components::{Banner, Cart, CartItem, Loading, ProductCard};
use crate::data::{self, get_products, CheckoutMethod, Currency};
use crate::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    // let loading = use_state(|| true);
    // let loading = Rc::new(use_state(|| true));

    let navigator = use_navigator().unwrap();
    // let onclick = Callback::from(move |_| navigator.push(&Route::About));

    // Mock products data // TODO: use use_effect()
    let products = get_products();

    let cart_visible = use_state(|| false);
    let cart_items = use_state(Vec::<CartItem>::new);

    let toggle_cart = {
        let cart_visible = cart_visible.clone();
        Callback::from(move |_| {
            cart_visible.set(!*cart_visible);
        })
    };

    let add_to_cart = {
        let cart_items = cart_items.clone();
        Callback::from(move |product: &data::Product| {
            let mut new_cart = (*cart_items).clone();
            if let Some(item) = new_cart.iter_mut().find(|item| item.id == product.id) {
                item.quantity += 1;
            } else {
                new_cart.push(CartItem {
                    id: product.id,
                    name: product.name, // product.name.clone()
                    price: product.price,
                    quantity: 1,
                });
            }
            cart_items.set(new_cart);
        })
    };

    let initiate_checkout_session = {
        let navigator = navigator.clone();
        let cart_items = cart_items.clone();
        let total = cart_items
            .iter()
            .fold(0.0, |acc, item| acc + item.price * item.quantity as f64);
        // let total_in_decimal = Decimal::from_f64(total).unwrap();
        //     //.unwrap_or(Decimal::ZERO)
        //     //.round_dp(2);

        let checkout_payload = json!({
          "amount": format!("{:.2}", total),
          "email": "alice.bobs@gmail.com".to_string(),
          "first_name": "Alice".to_string(),
          "last_name": "Bobs".to_string(),
          "currency": "KES".to_string(),
          "method": "M-PESA".to_string()
        });

        Callback::from(move |_: MouseEvent| {
            let checkout_payload = checkout_payload.clone();
            spawn_local(async move {
                let checkout_response = checkout_session(checkout_payload.clone()).await.unwrap();
                info!("CheckoutsAPI request");
                web_sys::console::log_1(&JsValue::from_str(
                    &serde_json::to_string(&checkout_payload).unwrap(),
                ));
                info!("CheckoutsAPI response");
                web_sys::console::log_1(&JsValue::from_str(
                    &serde_json::to_string(&checkout_response).unwrap(),
                ));
                info!("Total Amount");
                web_sys::console::log_1(&JsValue::from_str(
                    &serde_json::to_string(&total).unwrap(),
                ));
                // info!("Total Amount (Decimal)");
                // web_sys::console::log_1(&JsValue::from_str(
                //     &serde_json::to_string(&total_in_decimal).unwrap(),
                // ));
                // window()
                //     .unwrap()
                //     .location()
                //     .set_href(&checkout_response.url)
                //     .unwrap();
            });
        })
    };

    use_effect_with((), move |_| {
        spawn_local(async move {
            if let Some(token) = get_token_from_local_storage() {
                match verify_user_token(&token).await {
                    Ok(response) => {
                        if !response.success {
                            navigator.push(&Route::Login);
                        }
                    }
                    Err(_) => {
                        navigator.push(&Route::Login);
                    }
                }
            } else {
                navigator.push(&Route::Login);
            }
        });

        || ()
    });

    // if **loading {
    //       html! { <Loading /> }
    // } else {
    html! {
        <div>
            <div class="fixed top-0 right-0 m-4 z-50">
                <button
                    onclick={toggle_cart.clone()}
                    class="bg-blue-500 text-white p-2 rounded-full hover:bg-blue-600"
                >
                    // // Cart icon (you can replace this with an SVG icon for better styling)
                    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24">
                      <path d="M7 4h14l-1.5 7H7V4zm0 2v6h13.5L19 6H7zM4 2h2l1.5 2h12l1.5-2h2v2H4V2zm3 16c0 1.1.9 2 2 2s2-.9 2-2-1-2-2-2-2 .9-2 2zm10 0c0 1.1.9 2 2 2s2-.9 2-2-1-2-2-2-2 .9-2 2zM4 6l1.5 7H19l1.5-7H4z"/>
                    </svg>
                </button>
            </div>
            <Banner />
            <div class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                <h2 class="text-2xl font-extrabold tracking-tight text-gray-900 mb-6">
                    {"Featured Products"}
                </h2>
                <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                    {products.iter().map(|product| {
                        // let add_to_cart = add_to_cart.clone(); // func props
                        // let product_clone = product.clone();
                        html! {
                            <ProductCard
                                key={product.id}
                                image_url={product.image_url}
                                name={product.name}
                                description={product.description}
                                price={product.price}
                                on_add_to_cart={let add_to_cart = add_to_cart.clone(); Callback::from(move |_| add_to_cart.emit(product))}
                            />
                        }
                    }).collect::<Html>()}
                </div>
            </div>
            <div class="bg-gray-700 text-white py-12 px-4 sm:px-6 lg:px-8 m-4 rounded-md">
                <div class="max-w-7xl mx-auto">
                    <h2 class="text-3xl font-extrabold tracking-tight sm:text-4xl">
                        {"Welcome to Our Store"}
                    </h2>
                    <p class="mt-4 text-xl">
                        {"Discover amazing products at unbeatable prices."}
                    </p>
                    <div class="mt-8">
                        <button
                            onclick={initiate_checkout_session.clone()}
                            class="bg-white text-blue-600 px-6 py-3 rounded-md font-semibold text-lg hover:bg-blue-50 transition duration-300"
                        >
                            {"Checkout"}
                        </button>
                    </div>
                </div>
            </div>
            if *cart_visible {
                <Cart items={(*cart_items).clone()} on_close={toggle_cart} on_checkout={initiate_checkout_session}/>
            }
        </div>
    }
}

async fn checkout_session(payload: Value) -> Result<CheckoutResponse, reqwest::Error> {
    let client = Client::new();
    let url = "https://sandbox.intasend.com/api/v1/checkout/";
    let intasend_public_key = "ISPubKey_test_347ee874-ac7b-476c-b846-0894c9e8bdf6";
    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("X-IntaSend-Public-API-Key", intasend_public_key)
        .send()
        .await?
        .json::<CheckoutResponse>()
        .await?;
    Ok(res)
}

async fn verify_user_token(token: &str) -> Result<AuthenticatedUserResponse, reqwest::Error> {
    let client = Client::new();
    let url = "http://localhost:8888/api/v1/user";
    let res = client
        .get(url)
        .header("Content-Type", "application/json")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await? // Await the future here
        .json::<AuthenticatedUserResponse>()
        .await?; // Await the text extraction as well
    Ok(res)
}

fn get_token_from_local_storage() -> Option<String> {
    let storage = window()?.local_storage().ok()??;
    storage.get_item("yew_shop_auth_token").ok().flatten()
}

#[derive(Deserialize)]
struct AuthenticatedUserResponse {
    success: bool,
    data: Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckoutResponse {
    pub id: String,
    pub url: String,
    pub signature: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub method: Option<CheckoutMethod>,
    pub amount: Decimal,
    pub currency: Currency,
    pub paid: bool,
}

// #[function_component(Home)]
// pub fn home() -> Html {
//     html! {
//       <div>
//         <h1 class="text-2xl font-bold">{"Home"}</h1>
//           <button {onclick}>{"About"}</button>
//         <div class="p-4">
//             <ProductCard
//                 image_url="https://example.com/product-image.jpg"
//                 name="Amazing Product"
//                 description={Some("This is a great product you'll love!".to_string())}
//                 price=19.99
//             />
//         </div>
//       </div>
//     }
// }
