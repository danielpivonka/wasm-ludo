use yew::{function_component, html};

mod bindings;
mod components;
mod models;
mod pages;
mod routes;
mod types;
mod utils;
// mod context;

use routes::Routes;

#[function_component(App)]
pub fn app() -> Html {
  html! {<Routes />}
}

fn main() {
  yew::start_app::<App>();
}
