use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
  pub class: Classes,
  #[prop_or_default]
  pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Icon)]
pub fn icon(props: &IconProps) -> Html {
  let IconProps { class, onclick } = props.clone();

  html! {
    <span class={classes!(class, onclick.is_some().then(|| "hover:cursor-pointer"))} {onclick} />
  }
}
