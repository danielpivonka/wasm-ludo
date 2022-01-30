use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
  pub class: Classes,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
  let IconProps { class } = props.clone();

  html! {
    <span class={classes!(class)} />
  }
}
