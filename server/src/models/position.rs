#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position{
    position: usize,
    is_home: bool,
  }