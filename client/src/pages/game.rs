use yew::prelude::*;

use crate::components::board::Board;
use crate::components::player::{Player, PlayerButtonPosition};

#[function_component(Game)]
pub fn game() -> Html {

  let noop = Callback::from(|_| {});

  html! {
    <div class="py-4 flex">
      <div class="flex flex-col justify-between item-center p-4 max-w-md flex-grow">
        <Player name={"John"} position={PlayerButtonPosition::Bottom} />
        <Player name={"John"} position={PlayerButtonPosition::Top} on_roll={noop.clone()} />
      </div>
      <div class="flex-grow">
        <Board />
      </div>
      <div class="flex flex-col justify-between item-center p-4 max-w-md flex-grow">
        <Player name={"John"} position={PlayerButtonPosition::Bottom} />
        <Player name={"John"} position={PlayerButtonPosition::Top} />
      </div>
    </div>
  }
}