#[derive(PartialEq, Clone, Debug)]
pub struct DieInfo {
  pub number: usize,
  pub can_roll: bool,
}

impl DieInfo {
  pub fn new() -> Self {
    DieInfo {
      number: 1,
      can_roll: false,
    }
  }
}
