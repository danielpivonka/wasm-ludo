use crate::models::game::Game;
use crate::utils::dice::throw_dice;
use crate::utils::enums::MoveResult;

pub fn make_a_move_player(game: &mut Game, position: usize, is_home: bool) -> MoveResult {
  let dice_value = game.dice_throws.iter().sum::<usize>();
  game.execute_move(position, dice_value, is_home)
}
