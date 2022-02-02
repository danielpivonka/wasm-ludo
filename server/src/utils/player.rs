use crate::models::game::Game;
use crate::models::player::Player;
use crate::utils::enums::MoveResult;

pub fn make_a_move_player(game: &mut Game, position: usize, is_home: bool) -> MoveResult {
  let dice_value = game.dice_throws.iter().sum::<usize>();
  game.execute_move(position, dice_value, is_home)
}

pub fn get_available_moves(game: &Game, dice_value: usize) -> (Vec<usize>, Vec<usize>, bool) {
  let positions = game.get_players_pieces_positions(&game.current_player);
  let player = game.get_current_player();

  let mut positions_on_board: Vec<usize> = positions
    .clone()
    .into_iter()
    .filter(|position| game.can_jump(*position, dice_value))
    .collect();

  let mut piece_positions_to_jump_home: Vec<usize> = positions
    .into_iter()
    .filter(|position| game.can_jump_to_home(*position, dice_value))
    .collect();

  positions_on_board.append(&mut piece_positions_to_jump_home);

  let can_promote = player.pawns_at_start > 0 && game.can_promote_piece(dice_value);

  let piece_positions_in_home_row: Vec<usize> = player
    .home
    .clone()
    .into_iter()
    .enumerate()
    .filter(|(_position, field)| field.is_some())
    .map(|(position, _field)| position)
    .filter(|&position| game.can_jump_from_home(position, dice_value))
    .collect();

  (positions_on_board, piece_positions_in_home_row, can_promote)
}
