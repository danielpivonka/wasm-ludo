use yew::prelude::*;

use crate::components::board::Board;

#[function_component(Game)]
pub fn game() -> Html {

  html! {
    <div class="h-full">
      <Board />
    </div>
  }
}