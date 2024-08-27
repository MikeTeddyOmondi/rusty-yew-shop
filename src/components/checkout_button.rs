use _CheckoutButtonProps::on_checkout;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::data::{CheckoutMethod, Currency};

#[derive(Properties, PartialEq)]
pub struct CheckoutButtonProps {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub method: Option<CheckoutMethod>,
    pub amount: Decimal,
    pub currency: Option<Currency>,
    pub on_checkout: Callback<MouseEvent>,
}

#[function_component(CheckoutButton)]
pub fn checkout_button(props: &CheckoutButtonProps) -> Html {
    html! {
        // <div class="px-4 py-2 bg-gray-700 text-white text-base font-medium rounded-md w-full shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300">
        <button class="px-4 py-2 bg-gray-700 text-white text-base font-medium rounded-md w-full shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300 intaSendPayButton" 
          data-amount="10" 
          data-currency="KES" 
          onclick={props.on_checkout.clone()}
        >
          {"Pay Now"}
        </button>
        // </div>
    }
}

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct CheckoutResponse {
//     pub id: String,
//     pub url: String,
//     pub signature: String,
//     pub first_name: Option<String>,
//     pub last_name: Option<String>,
//     pub email: Option<String>,
//     pub method: Option<CheckoutMethod>,
//     pub amount: Decimal,
//     pub currency: Currency,
//     pub paid: bool,
// }
