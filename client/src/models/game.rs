use crate::types::FieldType;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use super::player::Player;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
  pub id: String,
  pub started_at: DateTime<Utc>,
  pub finished_at: Option<DateTime<Utc>>,
  pub fields: Vec<FieldType>,
  pub players: Vec<Player>,
}
