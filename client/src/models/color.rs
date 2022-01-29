use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Color {
  Red,
  Green,
  Blue,
  Yellow,
}
