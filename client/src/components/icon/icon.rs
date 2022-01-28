use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
  pub class: String,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
  let IconProps { class } = props;

  html! {
    <span class={class} />
  }
}