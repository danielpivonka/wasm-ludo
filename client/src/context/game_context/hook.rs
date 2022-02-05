use std::collections::HashMap;
use std::sync::Mutex;

use futures::channel::{mpsc, oneshot};
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::Message;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use gloo::console::log;
use gloo::storage::{SessionStorage, Storage};

use crate::context::snackbar::context::{SnackbarContext, SnackbarOptions, SnackbarVariant};
use crate::models::color::Color;
use crate::models::die_info::DieInfo;
use crate::models::game::Game;
use crate::models::messages::{ClientMessage, ServerMessage};
use crate::models::websocket::StateWebSocket;

use super::context::{GameContext, MsgSender};
use super::game_reducer::GameState;

#[derive(Properties, PartialEq, Clone)]
pub struct UseGameProps {
  pub game_id: String,
}

pub fn use_game(props: &UseGameProps) -> GameContext {
  let SnackbarContext { open } = use_context::<SnackbarContext>().expect("context not found");
  let game_state = use_reducer(GameState::default);
  // let player_color = use_state(|| Color::Yellow);
  let sender = use_state(|| None);
  let game_id = props.game_id.clone();
  let event_handler = use_state::<Option<Callback<ServerMessage>>, _>(|| None);
  // let game = use_state::<Game, _>(Game::new);

  // let dice_info = use_state::<HashMap<Color, DieInfo>, _>(|| {
  //   log!("setting dice_info");
  //   [
  //     (Color::Yellow, DieInfo::new()),
  //     (Color::Green, DieInfo::new()),
  //     (Color::Blue, DieInfo::new()),
  //     (Color::Red, DieInfo::new()),
  //   ]
  //   .iter()
  //   .cloned()
  //   .collect::<HashMap<_, _>>()
  // });

  let handle_message = {
    // let player_color = player_color.clone();
    // let game = game.clone();
    let game_state = game_state.clone();
    // let dice_info = dice_info.clone();
    Callback::from(move |message: ServerMessage| {
      match message.clone() {
        // ServerMessage::GameUpdate(new_game) => game.set(new_game),
        // ServerMessage::GameStarted(new_game) => game.set(new_game),
        // ServerMessage::DiceValue(number, can_roll) => {
        //   log!(format!("old map: {:?}", &*dice_info));
        //   let mut new_map = (*dice_info).clone();
        //   new_map.insert((*game).current_player.clone(), DieInfo { number, can_roll });
        //   log!(format!("new map: {:?}", &new_map));
        //   dice_info.set(new_map);
        // }
        ServerMessage::Error(message) => {
          open.emit(SnackbarOptions {
            message,
            variant: SnackbarVariant::Error,
          });
        }
        // ServerMessage::ConnectResponse(returned_game, color) => {
        //   game.set(returned_game);
        //   player_color.set(color);
        // }
        _ => {}
      }
      game_state.dispatch(message);
    })
  };

  {
    let sender = sender.clone();
    let event_handler = event_handler.clone();
    use_effect_with_deps::<_, Box<dyn FnOnce()>, _>(
      move |callback| {
        let callback = (**callback).clone();
        let handle_message = handle_message.clone();
        let player_id: String = SessionStorage::get("player_id").unwrap();
        log!(format!(
          "ws://127.0.0.1:8080/games/websocket/{}/{}",
          game_id, player_id
        ));

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
            log!(text.clone());
            if let Ok(message) = serde_json::from_str::<ServerMessage>(text.as_str()) {
              handle_message.emit(message.clone());
              if let Some(callback) = callback.clone() {
                callback.emit(message.clone());
              };
            } else {
              log!("Parsing of message failed:\n", text);
            }
          }
          log!("WEBSOCKET CLOSED")
        });

        spawn_local(async move {
          while let Some(msg) = rx.next().await {
            let json = serde_json::to_string(&msg).unwrap();
            write.send(Message::Text(json)).await.unwrap();
          }
        });

        Box::new(|| {})
      },
      event_handler,
    );
  }

  let subscribe = {
    Callback::from(move |function: Callback<ServerMessage>| {
      event_handler.set(Some(function));
    })
  };

  // let players = game.players.iter().fold(HashMap::new(), |mut acc, player| {
  //   acc.insert(player.color.clone(), player.clone());
  //   acc
  // });

  GameContext {
    game: game_state.game.clone(),
    player_color: game_state.player_color.clone(),
    player_count: 0,
    subscribe,
    sender: (*sender).clone(),
    current_player: game_state.game.current_player.clone(),
    dice_info: game_state.dice_info.clone(),
  }
}
