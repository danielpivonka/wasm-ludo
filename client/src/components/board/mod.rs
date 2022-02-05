use yew::prelude::*;

use crate::components::board_middle::BoardMiddle;
use crate::components::fields::{Fields, FieldsPosition};
use crate::components::player_corner::PlayerCorner;
use crate::models::color::Color;

#[function_component(Board)]
pub fn board() -> Html {
  let fields = vec![None; 18];

  html! {
    <div class="mx-auto max-w-3xl grid grid-cols-board grid-rows-board aspect-square rounded border-8 shadow-lg border-neutral-200">
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Yellow} />
      </div>
      <div class="border border-neutral-300">
        <Fields position={FieldsPosition::Top} color={Color::Blue} fields={fields.clone()} offset={13} />
      </div>
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Blue} />
      </div>
      <div class="border border-neutral-300">
        <Fields position={FieldsPosition::Left} color={Color::Yellow} fields={fields.clone()} offset={0} />
      </div>
      <div class="border border-neutral-300">
        <BoardMiddle />
      </div>
      <div class="border border-neutral-300">
        <Fields position={FieldsPosition::Right} color={Color::Red} fields={fields.clone()} offset={26} />
      </div>
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Green} />
      </div>
      <div class="border border-neutral-300">
        <Fields position={FieldsPosition::Bottom} color={Color::Green} fields={fields.clone()} offset={39} />
      </div>
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Red} />
      </div>
    </div>
  }
}
