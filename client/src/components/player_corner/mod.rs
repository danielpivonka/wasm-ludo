use futures::SinkExt;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::pawn::Pawn;
use crate::context::game_context::context::GameContext;
use crate::models::color::Color;
use crate::models::messages::ClientMessage;
use crate::utils::{clamp, resolve_bg_color_class};

#[derive(Properties, PartialEq, Clone)]
pub struct PlayerCornerProps {
  pub color: Color,
}

#[function_component(PlayerCorner)]
pub fn player_corner(props: &PlayerCornerProps) -> Html {
  let PlayerCornerProps { color } = props.clone();
  let GameContext { game, sender, .. } = use_context::<GameContext>().expect("context not found");

  let pawn_count = game
    .players
    .iter()
    .find(|player| player.color == color)
    .and_then(|player| Some(player.pawns_at_start))
    .unwrap_or(0);
  let pawn_count = clamp(pawn_count, 0, 4);

  let onclick = {
    Callback::from(move |_| {
      let sender = sender.clone();
      spawn_local(async move {
        if let Some(mut sender) = sender.clone() {
          sender.0.send(ClientMessage::PromotePiece).await.ok();
        };
      });
    })
  };

  let color_class = resolve_bg_color_class(&color);
  html! {
    <div class={classes!(String::from("h-full w-full grid place-items-center drop-shadow-lg"), color_class)}>
      <div class="w-1/2 h-1/2 hover:cursor-pointer rounded-full bg-neutral-100 grid grid-cols-2 grid-rows-2 p-4 drop-shadow-lg border border-neutral-300" onclick={onclick} >
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
