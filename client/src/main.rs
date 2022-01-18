use yew::{function_component, html};

mod components;

use components::spinner::Spinner;

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <div class="h-full grid place-items-center">
      <div class="flex flex-col justify-center items-center">
        <Spinner />
        <h1 class="mt-4 font-semibold">{"Hello from the client, page is under construction"}</h1>
      </div>
    </div>
  }
}

fn main() {
  yew::start_app::<App>();
}
