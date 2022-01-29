use yew::prelude::*;

use crate::components::player_corner::PlayerCorner;
use crate::models::color::Color;
use crate::components::fields::{Fields, FieldsPosition};
use crate::components::board_middle::BoardMiddle;

#[derive(Properties, PartialEq, Clone)]
pub struct BoardProps {}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {

  let fields = vec![None; 18];

  html! {
    <div class="mx-auto max-w-3xl grid grid-cols-board grid-rows-board aspect-square rounded border-8 shadow-lg border-neutral-200">
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Yellow} pawn_count={1} />
      </div>
      <div class="border border-neutral-300">
        <Fields position={FieldsPosition::Top} color={Color::Blue} fields={fields.clone()} />
      </div>
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Blue} pawn_count={2} />
      </div>
      <div class="border border-neutral-300">
        <Fields position={FieldsPosition::Left} color={Color::Yellow} fields={fields.clone()} />
      </div>
      <div class="border border-neutral-300">
        <BoardMiddle />
      </div>
      <div class="border border-neutral-300">
        <Fields position={FieldsPosition::Right} color={Color::Red} fields={fields.clone()} />
      </div>
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Green} pawn_count={3} />
      </div>
      <div class="border border-neutral-300">
        <Fields position={FieldsPosition::Bottom} color={Color::Green} fields={fields.clone()} />
      </div>
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Red} pawn_count={4} />
      </div>
    </div>
  }
}