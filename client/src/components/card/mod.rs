use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub class: Classes,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
  let CardProps { children, class } = props;

  html! {
    <div class={classes!(String::from("shadow-md bg-white box-border border border-neutral-300 rounded"), class.clone())}>
      { for children.iter() }
    </div>
  }
}
