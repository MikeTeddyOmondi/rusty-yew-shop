use yew::prelude::*;

#[function_component(Banner)]
pub fn banner() -> Html {
    let on_cta_click = Callback::from(|_| {
        // Implement call-to-action functionality here
        log::info!("CTA clicked");
    });

    html! {
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
                        onclick={on_cta_click}
                        class="bg-white text-blue-600 px-6 py-3 rounded-md font-semibold text-lg hover:bg-blue-50 transition duration-300"
                    >
                        {"Shop Now"}
                    </button>
                </div>
            </div>
        </div>
    }
}