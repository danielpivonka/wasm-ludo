use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum Color {
  Red,
  Green,
  Blue,
  Yellow,
}
