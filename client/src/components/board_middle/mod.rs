use std::collections::HashMap;

use stylist::{css, yew::styled_component};
use yew::prelude::*;

use crate::{context::game_context::context::GameContext, models::color::Color};

#[styled_component(BoardMiddle)]
pub fn board_middle() -> Html {
  let GameContext { game, .. } = use_context::<GameContext>().expect("context not found");

  let pawns_at_finish = game.players.iter().fold(HashMap::new(), |mut acc, player| {
    acc.insert(player.color.clone(), player.pawns_at_finish);
    acc
  });

  let text_shadow = css!("text-shadow: 0 3px 5px rgba(0,0,0,0.20);");

  html! {
    <div class="h-full w-full relative text-white font-bold text-lg md:text-xl lg:text-2xl">
      <div class={classes!(String::from("absolute top-0 left-0 right-0 bg-blue-400 h-1/2 w-full"), css!("clip-path: polygon(0% 0%, 100% 0%, 50% 100%);"))}>
        <div class={classes!(String::from("absolute p-3 grid place-items-center top-0 left-0 right-0"))}>
          <span class={text_shadow.clone()}>{pawns_at_finish.get(&Color::Blue).unwrap_or(&0)}</span>
        </div>
      </div>
      <div class={classes!(String::from("absolute bottom-0 left-0 right-0 bg-green-400 h-1/2 w-full"), css!("clip-path: polygon(0% 100%, 50% 0%, 100% 100%);"))}>
        <div class={classes!(String::from("absolute p-3 grid place-items-center bottom-0 left-0 right-0"))}>
          <span class={text_shadow.clone()}>{pawns_at_finish.get(&Color::Green).unwrap_or(&0)}</span>
        </div>
      </div>
      <div class={classes!(String::from("absolute top-0 bottom-0 right-0 bg-red-400 w-1/2 h-full"), css!("clip-path: polygon(100% 0%, 0% 50%, 100% 100%);"))}>
        <div class={classes!(String::from("absolute p-3 grid place-items-center top-0 bottom-0 right-0"))}>
          <span class={text_shadow.clone()}>{pawns_at_finish.get(&Color::Red).unwrap_or(&0)}</span>
        </div>
      </div>
      <div class={classes!(String::from("absolute top-0 bottom-0 left-0 bg-yellow-400 w-1/2 h-full"), css!("clip-path: polygon(0% 0%, 100% 50%, 0% 100%);"))}>
        <div class={classes!(String::from("absolute p-3 grid place-items-center top-0 bottom-0 left-0"))}>
          <span class={text_shadow}>{pawns_at_finish.get(&Color::Yellow).unwrap_or(&0)}</span>
        </div>
      </div>
    </div>
  }
}
