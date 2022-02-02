use stylist::css;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::{models::color::Color, utils::resolve_text_color_class};

#[derive(Properties, PartialEq, Clone)]
pub struct PawnProps {
  pub color: Color,
  #[prop_or_default]
  pub onclick: Option<Callback<MouseEvent>>,
}

#[function_component(Pawn)]
pub fn pawn(props: &PawnProps) -> Html {
  let PawnProps { color, onclick } = props.clone();

  let text_class = resolve_text_color_class(&color);
  let hover_anim: Option<String> = onclick.is_some().then(|| "hover:scale-110".into());

  html! {
    <button {onclick} class={classes!(onclick.is_none().then(|| "cursor-default"))}>
      <Icon class={classes!(String::from("fas text-xl sm:text-3xl md:text-4xl w-min fa-chess-pawn drop-shadow-md hue-rotate-15 saturate-50"), text_class, hover_anim, css!("text-shadow: 2px 2px 2px gray;"))} />
    </button>
  }
}
