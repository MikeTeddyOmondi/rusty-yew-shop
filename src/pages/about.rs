use yew::{function_component, html, Callback, Html};
use yew_router::hooks::use_navigator;

use crate::router::Route;

#[function_component(About)]
pub fn about() -> Html {
  
  let navigator = use_navigator().unwrap();
  let onclick = Callback::from(move|_| navigator.push(&Route::Home));
  
  html! {
    <div>
      <h1 class="text-2xl font-bold">{"Hello, Yew!"}</h1>
        <h1>{"About"}</h1>
        <button {onclick}>{"Home"}</button>
    </div>
  }
}
