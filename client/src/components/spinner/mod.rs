use yew::{function_component, html};

#[function_component(Spinner)]
pub fn spinner() -> Html {
  html! {
    <div class="animate-spin h-10 w-10 border-8 border-gray-100 border-t-gray-600 rounded-full" />
  }
}
