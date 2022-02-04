use futures::SinkExt;
use gloo::console::log;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::board::Board;
use crate::components::player::{Player, PlayerButtonPosition};
use crate::context::game_context::model::GameContext;
use crate::models::color::Color;
use crate::models::messages::{ClientMessage, ServerMessage};

#[derive(Properties, PartialEq, Clone)]
pub struct GameProps {
  pub id: String,
}
#[function_component(Game)]
pub fn game(props: &GameProps) -> Html {
  let context = use_context::<GameContext>().expect("provider is not a parent");
  let GameProps { id } = props.clone();
  let subscribe = context.subscribe;
  let sender = context.sender;
  {
    let id = id.clone();
    use_effect_with_deps(
      move |_: &[u32; 0]| {
        subscribe.emit(Callback::from(
          move |message: ServerMessage| match message {
            ServerMessage::DiceValue(roll, repeat) => {
              log!(roll)
            },
            ServerMessage::GameStarted(_) => {
              log!("game started recieved from server in subscribe callback");
            },
            ServerMessage::Error(msg) => log!(msg),
            _ => {},
          },
        ));

        || {}
      },
      [],
    );
  }

  let roll = {
    let sender = sender.clone();
    Callback::from(move |_| {
      let sender = sender.clone();
      spawn_local(async move {
        if let Some(mut sender) = sender {
          sender.0.send(ClientMessage::ThrowDice).await.ok();
        };
      });
    })
  };

  let promote = {
    let sender = sender.clone();
    Callback::from(move |_color:Color| {
      let sender = sender.clone();
      spawn_local(async move {
        if let Some(mut sender) = sender {
          sender.0.send(ClientMessage::PromotePiece).await.ok();
        };
      });
    })
  };

  html! {
    <div class="py-4 flex">
      <div class="flex flex-col justify-between item-center p-4 max-w-md flex-grow">
        <Player name={"John"} position={PlayerButtonPosition::Bottom} color={Color::Yellow} />
        <Player name={"John"} position={PlayerButtonPosition::Top} on_roll={roll} color={Color::Blue} />
      </div>
      <div class="flex-grow">
        <Board on_promote={promote}/>
      </div>
      <div class="flex flex-col justify-between item-center p-4 max-w-md flex-grow">
        <Player name={"John"} position={PlayerButtonPosition::Bottom} color={Color::Green} />
        <Player name={"John"} position={PlayerButtonPosition::Top} color={Color::Red} />
      </div>
    </div>
  }
}
