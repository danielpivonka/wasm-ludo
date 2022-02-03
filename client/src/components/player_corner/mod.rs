use yew::prelude::*;

use crate::components::pawn::Pawn;
use crate::models::color::Color;
use crate::utils::{clamp, resolve_bg_color_class};

#[derive(Properties, PartialEq, Clone)]
pub struct PlayerCornerProps {
  pub color: Color,
  pub pawn_count: u32,
  #[prop_or_default]
  pub on_promote: Option<Callback<Color>>,
}

#[function_component(PlayerCorner)]
pub fn player_corner(props: &PlayerCornerProps) -> Html {
  let PlayerCornerProps { color, pawn_count, on_promote} = props.clone();

  let pawn_count = clamp(pawn_count, 0, 4);

  let color_class = resolve_bg_color_class(&color);
  let callback_color = color.clone();
  let on_click = {move |_|on_promote.clone().unwrap().emit(callback_color.clone())};
  html! {
    <div class={classes!(String::from("h-full w-full grid place-items-center drop-shadow-lg"), color_class)}>
      <div class="w-1/2 h-1/2 rounded-full bg-neutral-100 grid grid-cols-2 grid-rows-2 p-4 drop-shadow-lg border border-neutral-300" onclick={on_click} >
        {
          (0..pawn_count).map(|index| html! {
            <div class="grid place-items-center h-full w-full">
              <Pawn key={index} color={color.clone()}/>
            </div>
          }).collect::<Vec<Html>>()
        }
      </div>
    </div>
  }
}
