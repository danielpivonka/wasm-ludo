use crate::types::FieldType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::color::Color;

use super::player::Player;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Game {
  // TODO: server uses bson::DateTime
  pub started_at: DateTime<Utc>,
  pub finished_at: Option<DateTime<Utc>>,
  pub fields: Vec<FieldType>,
  pub players: Vec<Player>,
  pub current_player: Color,
  pub dice_throws: Vec<usize>,
}
