use crate::types::Field;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
  pub id: String,
  pub started_at: DateTime,
  pub finished_at: Option<DateTime>,
  pub fields: Vec<Field>,
  pub players: Vec<Player>,
}
