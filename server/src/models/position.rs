use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
  pub position: usize,
  pub is_home: bool,
}
