use crate::models::{
  game::Game,
  messages::{ClientMessage, ServerMessage},
};
use futures::channel::mpsc::Sender;
use yew::Callback;

#[derive(Clone, Debug, PartialEq)]
pub struct GameContext {
  pub game: Option<Game>,
  pub player_count: u32,
  pub subscribe: Callback<Callback<ServerMessage>>,
  pub sender: Option<MsgSender>,
}

#[derive(Clone, Debug)]
pub struct MsgSender(pub Sender<ClientMessage>);

impl PartialEq for MsgSender {
  fn eq(&self, _other: &Self) -> bool {
    true
  }

  // fn ne(&self, _other: &Self) -> bool {
  //   false
  // }
}
