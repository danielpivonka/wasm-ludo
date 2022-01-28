use yew::{function_component, html};

mod bindings;
mod components;
mod macros;
mod pages;
mod routes;

use routes::Routes;

#[function_component(App)]
pub fn app() -> Html {
  html! {<Routes />}
}

fn main() {
  yew::start_app::<App>();
}
