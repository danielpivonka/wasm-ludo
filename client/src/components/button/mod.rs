use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
  #[prop_or_default]
  pub children: Children,
  #[prop_or(false)]
  pub disabled: bool,
  pub onclick: Callback<MouseEvent>,
  #[prop_or_default]
  pub icon: Html,
  #[prop_or_default]
  pub class: String,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
  let ButtonProps {
    children,
    disabled,
    onclick,
    icon,
    class,
  } = props.clone();

  html! {
    <button {onclick} {disabled} class={classes!(String::from("rounded bg-primary-600 hover:bg-primary-700 text-white
      p-3 shadow-md font-semibold flex justify-center items-center gap-4"), disabled.then(|| "bg-neutral-600" ), class)}
    >
      { icon }
      { for children.iter() }
    </button>
  }
}
