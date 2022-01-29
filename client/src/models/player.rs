use serde::{Deserialize, Serialize};

use crate::types::FieldType;

use super::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
  pub player_id: String,
  pub color: Color,
  pub pawns_at_start: u32,
  pub home: Vec<FieldType>,
}
