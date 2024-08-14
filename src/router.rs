use yew::{html, Html};
use yew_router::Routable;

use crate::pages::home::Home;
use crate::pages::about::About;
use crate::pages::login::Login;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/about")]
  About,
  #[at("/login")]
  Login,
  #[at("/")]
  Home,
}

pub fn switch(route: Route) -> Html {
  match route {
    Route::Login => html!{ <Login/> },
    Route::About => html!{ <About/> },
    Route::Home => html!{ <Home/> }
  }
}