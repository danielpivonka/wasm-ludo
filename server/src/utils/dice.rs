use rand::Rng;

pub fn get_dice_value() -> usize {
  let mut rng = rand::thread_rng();
  rng.gen_range(1..7)
}
