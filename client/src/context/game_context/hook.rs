use futures::StreamExt;
use reqwasm::websocket::Message;
use reqwasm::websocket::futures::WebSocket;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use gloo::storage::{SessionStorage, Storage};
use gloo::console::log;

use crate::models::messages::ServerMessage;

use super::model::GameContext;

#[derive(Properties, PartialEq, Clone)]
pub struct UseGameProps {}

pub fn use_game(props: &UseGameProps) -> GameContext {
  let game_context = use_state(|| GameContext { game: None, player_count: 0 });

  use_effect_with_deps(
    move |_: &[u32; 0]| {
      log!("use effect triggered");
      let player_id: String = SessionStorage::get("player_id").unwrap();
      // let res = Request::get(format!("ws://127.0.0.1:8080/games/websocket/{}/{}", id, player_id)).send().await;

      let mut ws = WebSocket::open(
        format!("ws://127.0.0.1:8080/games/websocket/{}/{}", id, player_id).as_str(),
      )
      .unwrap();
      let (mut write, mut read) = ws.split();

      spawn_local(async move {
        // TODO: handle errors as well
        while let Some(Ok(Message::Text(text))) = read.next().await {
          if let Ok(message) = serde_json::from_str::<ServerMessage>(text.as_str()) {
            match message {
              ServerMessage::DiceValue(_) => todo!(),
              ServerMessage::SkipPlayer => todo!(),
              ServerMessage::MoveSuccessful(_) => todo!(),
              ServerMessage::MoveFailed(_) => todo!(),
              ServerMessage::PiecePromoted => todo!(),
              ServerMessage::Information(_) => todo!(),
              ServerMessage::GameUpdate(_) => todo!(),
              ServerMessage::PlayerConnected(_) => todo!(),
              ServerMessage::PlayerDisconnected(_) => todo!(),
              ServerMessage::PlayerJoined(players) => {
                player_count.set(players);
              }
              ServerMessage::GameStarted => todo!(),
            };
            log!(format!("1. {:?}", message))
          }
        }
        log!("WebSocket Closed")
      });

      spawn_local(async move {
        write.send(Message::Text("hello".into())).await.unwrap();
      });

      || {}
    },
    [],
  );

  (*game_context).clone()
}