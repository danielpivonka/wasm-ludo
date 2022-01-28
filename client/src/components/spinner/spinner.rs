use yew::prelude::*;

#[function_component(Spinner)]
pub fn spinner() -> Html {
  html! {
    <div class="animate-spin h-10 w-10 border-8 border-neutral-200 border-t-primary-600 rounded-full" />
  }
}
