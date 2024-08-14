use yew::prelude::*;

#[function_component(Loading)]
pub fn loading() -> Html {
    html! {
        <div class="loading-container">
            <svg xmlns="http://www.w3.org/2000/svg" width="50" height="50" viewBox="0 0 50 50">
              <path fill="none" stroke="#000" stroke-width="5" d="M25 5 A20 20 0 1 1 24.999 5" stroke-linecap="round" stroke-dasharray="31.41592653589793 31.41592653589793" transform="rotate(-90 25 25)">
                <animateTransform attributeName="transform" type="rotate" from="0 25 25" to="360 25 25" dur="1s" repeatCount="indefinite" />
              </path>
            </svg>
        </div>
    }
}
