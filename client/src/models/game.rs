use crate::types::FieldType;
use chrono::{DateTime, Utc, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::models::color::Color;

use super::player::Player;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Game {
  // TODO: server uses bson::DateTime
  pub started_at: DateTime<FixedOffset>,
  pub finished_at: Option<DateTime<Utc>>,
  pub fields: Vec<FieldType>,
  pub players: Vec<Player>,
  pub current_player: Color,
  pub dice_throws: Vec<usize>,
}


impl Game {
  pub fn new() -> Self {
    Game {
      started_at: DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200").unwrap(),
      finished_at: None,
      fields: vec![Some(Color::Yellow); 52],
      players: vec![],
      current_player: Color::Green,
      dice_throws: vec![],
    }
  }
}