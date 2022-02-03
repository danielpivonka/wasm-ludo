use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Position {
  pub position: usize,
  pub is_home: bool,
}
