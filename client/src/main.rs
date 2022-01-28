use yew::{function_component, html};

mod components;
mod pages;
mod routes;
mod macros;
mod bindings;

use routes::Routes;

#[function_component(App)]
pub fn app() -> Html {
  html! {<Routes />}
}

fn main() {
  yew::start_app::<App>();
}
