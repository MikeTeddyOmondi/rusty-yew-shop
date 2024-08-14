use log::info;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::data::Product;

// #[derive(Properties, PartialEq)]
// pub struct ProductCardProps {
//     pub image_url: String,
//     pub name: AttrValue,
//     pub description: Option<String>,
//     pub price: f64,
//     pub on_add_to_cart: Callback<MouseEvent>,
// }
 
#[derive(Properties, PartialEq)]
pub struct ProductCardProps {
    // pub product: Product,
    pub image_url: String,
    pub name: AttrValue,
    pub description: Option<String>,
    pub price: f64,
    pub on_add_to_cart: Callback<MouseEvent>,
}


#[function_component(ProductCard)]
pub fn product_card(product: &ProductCardProps) -> Html {
    // let on_add_to_cart = Callback::from(|_: MouseEvent| {
    //     // Implement add to cart functionality here
    //     // Log using the log crate
    //     info!("Added to cart");
    //     web_sys::console::log_1(&JsValue::from_str("Added to cart"));
    // });

    html! {
        <div class="w-full max-w-sm bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700 flex flex-col h-full">
            <div class="p-4">
                <img class="rounded-lg w-full h-48 object-cover" src={product.image_url.clone()} alt={product.name.clone()} />
            </div>
            <div class="px-5 pb-5 flex flex-col flex-grow">
                <h5 class="text-xl font-semibold tracking-tight text-gray-900 dark:text-white">
                    {&product.name}
                </h5>
                if let Some(description) = &product.description {
                    <p class="mt-2 text-sm text-gray-500 dark:text-gray-400">
                        {description}
                    </p>
                }
                <div class="mt-auto">
                    <div class="flex items-center justify-between mt-4">
                        <span class="text-3xl font-bold text-gray-900 dark:text-white">
                            {format!("${:.2}", product.price)}
                        </span>
                        <button
                            // passed props
                            onclick={product.on_add_to_cart.clone()}
                            class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                        >
                            {"Add to cart"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}
