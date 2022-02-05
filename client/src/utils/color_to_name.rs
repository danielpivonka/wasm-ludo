use crate::models::{color::Color, game::Game};

pub fn color_to_name(game: &Game, color: Color) -> String {
  let player = game
    .players
    .clone()
    .into_iter()
    .find(|player| player.color == color);
  match player {
    Some(player) => player.name,
    None => "".to_string(),
  }
}
