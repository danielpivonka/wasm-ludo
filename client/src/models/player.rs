use serde::{Deserialize, Serialize};

use crate::types::FieldType;

use super::color::Color;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Player {
  pub id: String,
  pub name: String,
  pub color: Color,
  pub pawns_at_start: usize,
  pub pawns_at_finish: usize,
  pub home: Vec<FieldType>,
  pub is_bot: bool,
}
