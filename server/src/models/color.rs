use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Color {
  Red,
  Green,
  Blue,
  Yellow,
}
