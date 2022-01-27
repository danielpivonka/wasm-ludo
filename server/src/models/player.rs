use serde::{Deserialize, Serialize};

use crate::types::Field;

use super::color::Color;

const PIECES_COUNT: usize = 4;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
  pub id: String,
  pub color: Color,
  pub pawns_at_start: usize,
  pub pawns_at_finish: usize,
  pub home: Vec<Field>,
  pub is_bot: bool,
}

// TODO change id to name when db is fixed
impl Player {
  pub fn new(name: String, color: Color, is_bot: bool) -> Self {
    Player {
      id: name,
      color,
      pawns_at_start: PIECES_COUNT,
      pawns_at_finish: 0,
      home: vec![None; 5],
      is_bot,
    }
  }

  // returns whether all player's pieces are in home (occupy fields of home)
  // we assume there are 4 pieces for each player
  pub fn check_winner(&self) -> bool {
    // let occupied_fields: Vec<Field> = self.home.iter().filter(|&field| field.is_some()).collect();
    // occupied_fields.len() >= self.pieces_count()

    self.pawns_at_finish == PIECES_COUNT
  }

  pub fn increase_pieces_at_start(&mut self) {
    self.pawns_at_start += 1
  }

  pub fn decrease_pieces_at_start(&mut self) {
    self.pawns_at_start -= 1
  }
}
