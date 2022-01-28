use yew::prelude::*;

use crate::bindings::copy_to_clipboard;
use crate::components::icon::Icon;

#[derive(Properties, PartialEq, Clone)]
pub struct CopyBarProps {
  pub content: String,
}

#[function_component(CopyBar)]
pub fn copy_bar(props: &CopyBarProps) -> Html {
  let CopyBarProps { content } = props.clone();

  let onclick = {
    let content = content.clone();
    Callback::from(move |_| copy_to_clipboard(content.clone()))
  };

  html! {
    <button onclick={onclick} class="w-full rounded hover:bg-neutral-100 text-neutral-500 border border-neutral-300 p-3 flex items-center justify-between gap-4">
      <span class="font-semibold">{ content }</span>
      <Icon class="far fa-copy" />
    </button>
  }
}
