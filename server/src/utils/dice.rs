use rand::Rng;

pub fn get_dice_value() -> usize {
  let mut rng = rand::thread_rng();
  rng.gen_range(5..7)
}

// TODO: can be removed if we switch to new version of move_bot()
pub fn throw_dice() -> usize {
  let mut throw_sum: usize = 0;
  match get_dice_value() {
    6 => {
      throw_sum += 6;
      // send ServerMessage::DiceValue(6, false)
      match get_dice_value() {
        6 => {
          throw_sum += 6;
          let roll = get_dice_value();
          // << message exchange >>;
          throw_sum += roll;
        }
        n => throw_sum += n,
      }
    }
    n => throw_sum += n,
  }

  throw_sum
}
