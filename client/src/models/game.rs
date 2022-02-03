use crate::types::FieldType;
use serde::{Deserialize, Serialize};

use crate::models::color::Color;

use super::player::Player;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Game {
  pub started: bool,
  pub winner: Option<Color>,
  pub fields: Vec<FieldType>,
  pub players: Vec<Player>,
  pub current_player: Color,
  pub dice_throws: Vec<usize>,
}

impl Game {
  pub fn new() -> Self {
    Game {
      started: false,
      winner: None,
      fields: vec![None; 52],
      players: vec![],
      current_player: Color::Green,
      dice_throws: vec![],
    }
  }
}
