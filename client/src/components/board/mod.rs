use yew::prelude::*;

use crate::components::board_middle::BoardMiddle;
use crate::components::fields::{Fields, FieldsPosition};
use crate::components::player_corner::PlayerCorner;
use crate::context::game_context::model::GameContext;
use crate::models::color::Color;
#[derive(Properties, PartialEq, Clone)]
pub struct BoardProps {
  #[prop_or_default]
  pub on_promote: Option<Callback<Color>>,
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
  let BoardProps { on_promote } = props.clone();
  let fields = vec![None; 18];
  let GameContext { game, ..} = use_context::<GameContext>().expect("context not found");

  html! {
    <div class="mx-auto max-w-3xl grid grid-cols-board grid-rows-board aspect-square rounded border-8 shadow-lg border-neutral-200">
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Yellow} on_promote={on_promote.clone()}/>
      </div>
      <div class="border border-neutral-300">
        <Fields position={FieldsPosition::Top} color={Color::Blue} fields={fields.clone()} offset={13} />
      </div>
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Blue} on_promote={on_promote.clone()}/>
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
        <PlayerCorner color={Color::Green} on_promote={on_promote.clone()}/>
      </div>
      <div class="border border-neutral-300">
        <Fields position={FieldsPosition::Bottom} color={Color::Green} fields={fields.clone()} offset={39} />
      </div>
      <div class="border border-neutral-300">
        <PlayerCorner color={Color::Red} on_promote={on_promote.clone()}/>
      </div>
    </div>
  }
}
