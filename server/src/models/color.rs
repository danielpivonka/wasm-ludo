use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum Color {
  Red,
  Green,
  Blue,
  Yellow,
}

impl Color {
  pub fn ordered() -> Vec<Color> {
    vec![Color::Green, Color::Yellow, Color::Blue, Color::Red]
  }
}
