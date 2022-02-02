use yew::{function_component, html};

mod bindings;
mod components;
mod context;
mod models;
mod pages;
mod routes;
mod types;
mod utils;

use routes::Routes;

#[function_component(App)]
pub fn app() -> Html {
  html! {<Routes />}
}

fn main() {
  yew::start_app::<App>();
}
