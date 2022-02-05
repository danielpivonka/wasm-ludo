use crate::types::FieldType;
use serde::{Deserialize, Serialize};

use crate::models::color::Color;

use super::player::Player;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Game {
  pub started: bool,
  pub winner: Option<Color>,
  pub fields: Fields,
  pub players: Vec<Player>,
  pub current_player: Color,
  pub dice_throws: Vec<usize>,
  pub round_phase: RoundPhase,
}

impl Game {
  pub fn new() -> Self {
    Game {
      started: false,
      winner: None,
      fields: Fields::new(),
      players: vec![],
      current_player: Color::Green,
      round_phase: RoundPhase::Rolling,
      dice_throws: vec![],
    }
  }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Fields {
  values: Vec<FieldType>,
}
impl Fields {
  pub fn new() -> Fields {
    Fields {
      values: vec![None; 52],
    }
  }
  pub fn get(&self, i: usize) -> FieldType {
    self.values.get(i % 52).unwrap().clone()
  }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RoundPhase {
  Rolling,
  Moving,
}
