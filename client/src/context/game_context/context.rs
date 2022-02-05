use std::collections::HashMap;

use crate::models::{
  color::Color,
  die_info::DieInfo,
  game::Game,
  messages::{ClientMessage, ServerMessage},
};
use futures::channel::mpsc::Sender;
use yew::Callback;

#[derive(Clone, Debug, PartialEq)]
pub struct GameContext {
  pub game: Game,
  pub player_color: Color,
  pub player_count: u32,
  pub subscribe: Callback<Callback<ServerMessage>>,
  pub sender: Option<MsgSender>,
  // pub players: HashMap<Color, Player>,
  pub current_player: Color,
  pub dice_info: HashMap<Color, DieInfo>,
}

#[derive(Clone, Debug)]
pub struct MsgSender(pub Sender<ClientMessage>);

impl PartialEq for MsgSender {
  fn eq(&self, _other: &Self) -> bool {
    true
  }
}
