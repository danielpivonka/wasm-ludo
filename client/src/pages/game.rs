use yew::prelude::*;

use crate::components::board::Board;
use crate::components::player::{Player, PlayerButtonPosition};
use crate::models::color::Color;

#[function_component(Game)]
pub fn game() -> Html {
  html! {
    <div class="py-4 flex">
      <div class="flex flex-col justify-between item-center p-4 max-w-md flex-grow">
        <Player position={PlayerButtonPosition::Bottom} color={Color::Yellow} />
        <Player position={PlayerButtonPosition::Top} color={Color::Green} />
      </div>
      <div class="flex-grow">
        <Board />
      </div>
      <div class="flex flex-col justify-between item-center p-4 max-w-md flex-grow">
        <Player position={PlayerButtonPosition::Bottom} color={Color::Blue} />
        <Player position={PlayerButtonPosition::Top} color={Color::Red} />
      </div>
    </div>
  }
}
