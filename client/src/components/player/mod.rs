use gloo::timers::callback::Interval;
use yew::prelude::*;

use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::die::Die;
use crate::components::icon::Icon;

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
}

#[function_component(Player)]
pub fn player(props: &PlayerProps) -> Html {
  let PlayerProps {
    name,
    position,
    on_roll,
  } = props.clone();
  let die_number = use_state::<u32, _>(|| 1);

  let icon = html! { <Icon class="fas fa-sync-alt" /> };

  let button = if let Some(on_roll) = on_roll {
    html! { <Button {icon} onclick={on_roll.clone()}>{"Roll the die"}</Button> }
  } else {
    html! {}
  };

  {
    let die_number = die_number.clone();
    use_effect(move || {
      let interval = Interval::new(10000, move || {
        die_number.set(*die_number + 1);
      });

      || {
        drop(interval);
      }
    });
  }

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
