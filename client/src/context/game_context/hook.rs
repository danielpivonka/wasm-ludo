use std::collections::HashMap;

use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::Message;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log;
use yew::prelude::*;

use gloo::console::log;
use gloo::storage::{SessionStorage, Storage};

use crate::models::game::Game;
use crate::models::messages::{ClientMessage, ServerMessage};

use super::model::{GameContext, MsgSender};

#[derive(Properties, PartialEq, Clone)]
pub struct UseGameProps {
  pub game_id: String,
}

pub fn use_game(props: &UseGameProps) -> GameContext {
  let sender = use_state(|| None);
  let game_id = props.game_id.clone();
  let event_handler = use_state::<Option<Callback<ServerMessage>>, _>(|| None);
  let game = use_state::<Game, _>(|| Game::new());

  let handle_message = {
    let game = game.clone();
    Callback::from(move |message: ServerMessage| {
      match message {
        ServerMessage::GameUpdate(new_game) => game.set(new_game),
        ServerMessage::GameStarted(new_game) => {
          log!("game started recieved from server");
          game.set(new_game)
        },
        ServerMessage::Error(msg) => log!(msg),
        _ => {
        },
      }
    })
  };

  {
    let sender = sender.clone();
    let event_handler = event_handler.clone();
    use_effect_with_deps::<_, Box<dyn FnOnce()>, _>(
      move |[callback]| {
        let callback = (**callback).clone();
        let callback = match callback {
          Some(callback) => callback,
          None => return Box::new(|| {}),
        };
        let player_id: String = SessionStorage::get("player_id").unwrap();

        let ws = WebSocket::open(
          format!(
            "ws://127.0.0.1:8080/games/websocket/{}/{}",
            game_id, player_id
          )
          .as_str(),
        )
        .unwrap();

        let (mut write, mut read) = ws.split();
        let (tx, mut rx) = mpsc::channel::<ClientMessage>(1000);
        sender.set(Some(MsgSender(tx)));

        spawn_local(async move {
          // TODO: handle errors as well
          while let Some(Ok(Message::Text(text))) = read.next().await {
            log!("message: ", text.clone());
            if let Ok(message) = serde_json::from_str::<ServerMessage>(text.as_str()) {
              handle_message.emit(message.clone());
              callback.emit(message.clone());
              log!(format!("1. {:?}", message.clone()));
              log!("parsing ok");
            } else {
              log!("parsing failed");
            }
          }
          log!("WebSocket Closed")
        });

        spawn_local(async move {
          while let Some(msg) = rx.next().await {
            let json = serde_json::to_string(&msg).unwrap();
            write.send(Message::Text(json)).await.unwrap();
          }
        });

        Box::new(|| {})
      },
      [event_handler],
    );
  }

  let subscribe = {
    Callback::from(move |function: Callback<ServerMessage>| {
      event_handler.set(Some(function));
    })
  };

  let players = game.players.iter().fold(HashMap::new(), |mut acc, player| {
    acc.insert(player.color.clone(), player.clone());
    acc
  });

  GameContext {
    game: (*game).clone(),
    player_count: 0,
    subscribe,
    sender: (*sender).clone(),
    players,
    current_player: (*game).current_player.clone(),
  }
}
