use futures::SinkExt;
use gloo::console::log;
use gloo::timers::callback::{Interval, Timeout};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::die::Die;
use crate::components::icon::Icon;
use crate::context::game_context::context::GameContext;
use crate::models::color::Color;
use crate::models::messages::ClientMessage;

#[derive(PartialEq, Clone)]
pub enum PlayerButtonPosition {
  Top,
  Bottom,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PlayerProps {
  pub name: String,
  #[prop_or(PlayerButtonPosition::Top)]
  pub position: PlayerButtonPosition,
  pub color: Color,
}

#[function_component(Player)]
pub fn player(props: &PlayerProps) -> Html {
  let PlayerProps {
    name,
    position,
    color,
  } = props.clone();
  let GameContext {
    current_player,
    game,
    dice_info,
    sender,
    ..
  } = use_context::<GameContext>().expect("context not found");
  let die_info = dice_info.get(&color).expect("die info not set for player");
  let is_rolling = use_state(|| false);
  log!("dice in state: ", die_info.number);

  let icon = html! { <Icon class="fas fa-sync-alt" /> };

  let roll = {
    let is_rolling = is_rolling.clone();
    Callback::from(move |_| {
      is_rolling.set(true);
      let sender = sender.clone();
      spawn_local(async move {
        if let Some(mut sender) = sender.clone() {
          sender.0.send(ClientMessage::ThrowDice).await.ok();
        };
      });
    })
  };

  {
    let is_rolling = is_rolling.clone();
    let is_rolling_value = (*is_rolling).clone();
    use_effect_with_deps::<_, Box<dyn FnOnce()>, _>(
      |is_rolling_value| {
        if !is_rolling_value {
          return Box::new(|| {});
        };

        let timeout = Timeout::new(2000, move || {
          is_rolling.set(false);
        });

        Box::new(|| drop(timeout))
      },
      is_rolling_value,
    );
  }

  let button = if current_player == color {
    html! { <Button {icon} onclick={roll} disabled={!die_info.can_roll}>{"Roll the die"}</Button> }
  } else {
    html! {}
  };

  // {
  //   let die_number = die_number.clone();
  //   let throws = throws.clone();
  //   use_effect_with_deps::<_, Box<dyn FnOnce()>, _>(
  //     move |throws| {
  //       if current_player == color {
  //         let timeouts = throws
  //           .clone()
  //           .into_iter()
  //           .enumerate()
  //           .map(|(index, throw)| {
  //             let die_number = die_number.clone();
  //             Interval::new((1500 * index) as u32, move || {
  //               die_number.set(throw);
  //             })
  //           })
  //           .collect::<Vec<_>>();

  //         return Box::new(|| {
  //           for timeout in timeouts {
  //             drop(timeout)
  //           }
  //         });
  //       }

  //       Box::new(|| {})
  //     },
  //     throws,
  //   );
  // }

  // {
  //   let die_number = die_number.clone();
  //   use_effect(move || {
  //     let interval = Interval::new(10000, move || {
  //       die_number.set(*die_number + 1);
  //     });

  //     || {
  //       drop(interval);
  //     }
  //   });
  // }

  html! {
    <div class="flex flex-col gap-4">
      {
        if position == PlayerButtonPosition::Top {
          button.clone()
        } else { html! {} }
      }
      <Card>
        <div class="flex justify-between items-center p-4">
          <span class="text-lg font-semibold text-neutral-700">{ name }</span>
          <Die is_rolling={*is_rolling} number={die_info.number} />
        </div>
      // TODO: add timeline
      </Card>
      {
        if position == PlayerButtonPosition::Bottom {
          button.clone()
        } else { html! {} }
      }
    </div>
  }
}
