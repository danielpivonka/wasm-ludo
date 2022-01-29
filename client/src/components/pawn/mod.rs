use yew::prelude::*;

use crate::{models::color::Color, utils::resolve_color_class};

#[derive(Properties, PartialEq, Clone)]
pub struct PawnProps {
  pub color: Color,
}

#[function_component(Pawn)]
pub fn pawn(props: &PawnProps) -> Html {
  let PawnProps {color} = props.clone();

  let color_class = resolve_color_class(&color);
  
  html! {
    <div class="p-2 w-full h-full">
      <div class={classes!(String::from("rounded-full w-full h-full shadow-md"), color_class)}></div>
    </div>
  }
}