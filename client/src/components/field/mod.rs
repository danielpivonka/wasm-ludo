use yew::prelude::*;

use crate::utils::{resolve_bg_color_class, resolve_text_color_class};
use crate::models::color::Color;

#[derive(Properties, PartialEq, Clone)]
pub struct FieldProps {
  #[prop_or_default]
  pub children: Children,
  pub color: Color,
  #[prop_or(false)]
  pub color_background: bool,
}

#[function_component(Field)]
pub fn field(props: &FieldProps) -> Html {
  let FieldProps {children, color, color_background} = props.clone();

  let bg_class = if color_background {
    resolve_bg_color_class(&color)
  } else {
    "".into()
  };

  let text_class = resolve_text_color_class(&color);

  html! {
    <div class={classes!(String::from("border border-neutral-300 shadow-inner grid place-items-center"), bg_class, text_class)}>
      {for children.iter()}
    </div>
  }
}