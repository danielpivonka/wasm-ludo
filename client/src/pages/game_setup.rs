use yew::prelude::*;
use gloo::timers::callback::Interval;

use crate::components::content::Content;
use crate::components::card::Card;
use crate::components::copy_bar::CopyBar;
use crate::components::button::Button;
use crate::components::icon::Icon;
use crate::components::outlined_item::OutlinedItem;

#[derive(Properties, PartialEq, Clone)]
pub struct GameSetupProps {}

#[function_component(GameSetup)]
pub fn game_setup(props: &GameSetupProps) -> Html {
  let seconds = use_state(|| 0);
  let noop = Callback::from(|_| {});
  
  {
    let seconds = seconds.clone();
    use_effect(move || {
      let interval = Interval::new(1000, move || {
        seconds.set(*seconds + 1)
      });
      
      move || {
        drop(interval);
      }
    });
  }

  let start_icon = html! {
    <Icon class="fas fa-play"/>
  };

  let players_item = html! {
    {"3 / 4"}
  };

  let time_item = html! {
    {format!("{} seconds", *seconds)}
  };

  html! {
    <Content class="py-12 h-full">
      <div class="flex items-center mb-6 w-full">
        <div class="flex flex-col gap-2 w-full justify-between">
          <p class="text-5xl font-bold">{"Ludo"}</p>
          <p class="text-2xl text-neutral-600 font-bold">{"Board game for up to 4 players online"}</p>
        </div>
        <img class="h-28" src="/assets/ludo.svg" alt="" />
      </div>
      <Card class="w-full px-8 py-14 lg:px-40">
        <p class="text-xl text-neutral-600 font-bold">{"Share the link with your friends and start the game"}</p>
        <div class="flex items-center gap-3">
        <div class="flex-grow"><CopyBar content="Clipboard example" /></div>
        <div><Button onclick={noop.clone()}>{"Generate new link"}</Button></div>
        </div>
        <div class="flex items-center gap-3 text-neutral-600 mt-16">
          <Icon class="fas fa-info-circle" />
          <p class="text-xl font-bold">{"Starting the game without all 4 players will fill the remaining spots with bots"}</p>
        </div>
        <div class="flex flex-col gap-3">
          <OutlinedItem label="Players connected" item={players_item} />
          <OutlinedItem label="Time in lobby" item={time_item} />
        </div>
        <div class="w-full flex justify-end">
          <span>{"Waiting for other players to join"}</span>
        </div>
        <Button class="w-full mt-16" onclick={noop.clone()} icon={start_icon}>{"Start the game!"}</Button>
      </Card>
    </Content>
  }
}