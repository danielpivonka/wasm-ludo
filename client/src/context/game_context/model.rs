use crate::models::game::Game;

#[derive(Clone, Debug, PartialEq)]
pub struct GameContext {
  pub game: Option<Game>,
  pub player_count: u32,
}