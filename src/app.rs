use yew::prelude::*;
use yew_router::{BrowserRouter,Switch};

use crate::router::{Route, switch};
use crate::components::Layout;

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <div>
      <BrowserRouter>
        <Layout>
            <Switch<Route> render={switch} />
        </Layout>
      </BrowserRouter>
    </div>
  }
}
