use yew::prelude::*;
use gloo::timers::callback::{Timeout, Interval};

use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::die::Die;
use crate::components::icon::Icon;
use crate::context::game_context::model::GameContext;
use crate::models::color::Color;

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
  #[prop_or_default]
  pub on_roll: Option<Callback<MouseEvent>>,
  pub color: Color,
}

#[function_component(Player)]
pub fn player(props: &PlayerProps) -> Html {
  let PlayerProps {
    name,
    position,
    on_roll,
    color,
  } = props.clone();
  let GameContext {current_player, game, ..} = use_context::<GameContext>().expect("context not found");
  let die_number = use_state::<usize, _>(|| 1);
  let throws = game.dice_throws;

  let icon = html! { <Icon class="fas fa-sync-alt" /> };

  let button = if let Some(on_roll) = on_roll {
    html! { <Button {icon} onclick={on_roll}>{"Roll the die"}</Button> }
  } else {
    html! {}
  };

  {
    let die_number = die_number.clone();
    let throws = throws.clone();
    use_effect_with_deps::<_, Box<dyn FnOnce()>, _>(move |throws| {
      if current_player == color {
        let timeouts = throws.clone().into_iter().enumerate().map(|(index, throw)| {
          let die_number = die_number.clone();
          Interval::new((1500 * index) as u32, move || {
            die_number.set(throw);
          })
        }).collect::<Vec<_>>();

        return Box::new(|| {
          for timeout in timeouts {
            drop(timeout)
          }
         })
      }

      Box::new(|| {})
    }, throws);
  }

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
          <Die number={*die_number} />
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
