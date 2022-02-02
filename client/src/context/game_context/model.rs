use crate::models::{game::Game, messages::ServerMessage};
use yew::Callback;

#[derive(Clone, Debug, PartialEq)]
pub struct GameContext {
  pub game: Option<Game>,
  pub player_count: u32,
  pub subscribe: Callback<Callback<ServerMessage>>,
}
