use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct OutlinedItemProps {
  pub label: String,
  pub item: Html,
}

#[function_component(OutlinedItem)]
pub fn outlined_item(props: &OutlinedItemProps) -> Html {
  let OutlinedItemProps { label, item } = props.clone();

  html! {
    <div class="w-full font-semibold rounded border border-neutral-300 p-3 flex justify-between items-center">
      <span>{ label }</span>
      <span>{ item }</span>
    </div>
  }
}
