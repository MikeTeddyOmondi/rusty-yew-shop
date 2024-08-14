use yew::prelude::*;

use crate::components::{Cart, CartItem};

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
      <div class="min-h-screen flex flex-col">
        <header class="bg-gray-700 text-white p-4 shadow">
            <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                <h1 class="text-3xl font-bold text-gray-900">{"Yew Shop"}</h1>
            </div>
        </header>
        <main class="flex-grow">
            <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                { for props.children.iter() }
            </div>
        </main>
        <footer class="bg-gray-700 text-white p-4">
            <div class="max-w-7xl mx-auto py-4 px-4 sm:px-6 lg:px-8">
                <p class="text-center text-gray-500">{"© 2024 My App. All rights reserved."}</p>
            </div>
        </footer>
      </div>
    }
}
