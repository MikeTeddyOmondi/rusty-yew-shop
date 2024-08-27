use yew::prelude::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::components::CheckoutButton;
use crate::data::{CheckoutMethod, Currency};

#[derive(Clone, PartialEq)]
pub struct CartItem {
    pub id: u32,
    pub name: &'static str,
    pub price: f64,
    pub quantity: u32,
}

#[derive(Properties, PartialEq)]
pub struct CartProps {
    pub items: Vec<CartItem>,
    pub on_close: Callback<MouseEvent>,
    pub on_checkout: Callback<MouseEvent>
}

#[function_component(Cart)]
pub fn cart(props: &CartProps) -> Html {
    let total = props.items.iter().fold(0.0, |acc, item| acc + item.price * item.quantity as f64);

    html! {
        <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full" id="my-modal">
            <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
                <div class="mt-3 text-center">
                    <h3 class="text-lg leading-6 font-medium text-gray-900">{"Your Cart"}</h3>
                    <div class="mt-2 px-7 py-3">
                        {for props.items.iter().map(|item| {
                            html! {
                                <div class="flex justify-between items-center mb-2">
                                    <span>{format!("{} x {}", item.quantity, item.name)}</span>
                                    <span>{format!("${:.2}", item.price * item.quantity as f64)}</span>
                                </div>
                            }
                        })}
                        <div class="border-t pt-2 mt-2">
                            <div class="flex justify-between items-center">
                                <span class="font-bold">{"Total:"}</span>
                                <span class="font-bold">{format!("${:.2}", total)}</span>
                            </div>
                        </div>
                    </div>
                    <div class="flex flex-col space-y-3">
                      <div class="px-4 py-3">
                          <button
                              class="px-4 py-2 bg-blue-500 text-white text-base font-medium rounded-md w-full shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300"
                              onclick={props.on_close.clone()}
                          >
                              {"Close"}
                          </button>
                      </div>
                      <CheckoutButton
                        first_name = {None::<String>}
                        last_name = {None::<String>}
                        email = {None::<String>}
                        method = {None::<CheckoutMethod>}
                        amount = {Decimal::from_f64(total).unwrap_or(Decimal::TEN)}
                        currency = {Some(Currency::Kes)}
                        on_checkout = {props.on_checkout.clone()} // {let on_checkout = props.on_checkout.clone(); Callback::from(move |_| on_checkout)}
                      />
                      // <div class="px-4 py-3">
                      //   <button
                      //       class="px-4 py-2 bg-gray-700 text-white text-base font-medium rounded-md w-full shadow-sm hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-300"
                      //       onclick={props.on_checkout.clone()}
                      //   >
                      //       {"Checkout"}
                      //   </button>
                      // </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
