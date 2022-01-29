use yew::prelude::*;

use crate::utils::resolve_color_class;
use crate::models::color::Color;

#[derive(Properties, PartialEq, Clone)]
pub struct FieldProps {
  #[prop_or_default]
  pub children: Children,
  #[prop_or_default]
  pub color: Option<Color>,
}

#[function_component(Field)]
pub fn field(props: &FieldProps) -> Html {
  let FieldProps {children, color} = props.clone();

  let classes = match color {
    Some(color) => resolve_color_class(&color),
    None => "".into(),
  };

  html! {
    <div class={classes!(String::from("border border-neutral-300"), classes)}>
      {for children.iter()}
    </div>
  }
}