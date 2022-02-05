use crate::models::game::Game;
use crate::utils::dice::throw_dice;
use crate::utils::enums::MoveResult;
use rand::Rng;

pub fn create_bot_name() -> String {
  let names = [
    "Wade",
    "Dave",
    "Seth",
    "Ivan",
    "Riley",
    "Gilbert",
    "Jorge",
    "Dan",
    "Brian",
    "Roberto",
    "Ramon",
    "Miles",
    "Liam",
    "Nathaniel",
    "Ethan",
    "Lewis",
    "Milton",
    "Claude",
    "Joshua",
    "Glen",
  ];
  let surnames = [
    "Williams", "Harris", "Thomas", "Robinson", "Walker", "Scott", "Nelson", "Mitchell", "Morgan",
    "Cooper", "Howard", "Davis", "Miller", "Martin", "Smith", "Anderson", "White", "Perry",
    "Clark", "Richards",
  ];
  let mut rng = rand::thread_rng();
  format!(
    "{} {}",
    names[rng.gen_range(0..21)],
    surnames[rng.gen_range(0..21)]
  )
}

// since make_a_move_bot() doesn't call play_round() - which takes care of game.update_next_player(),
//   we have to take care of it in make_a_move_bot() separately
// if bot made a successful move, we should move onto the next player
// TODO: can be removed if move_bot_2() is used
fn handle_bot_move_result(game: &mut Game, move_result: MoveResult) -> MoveResult {
  match move_result.clone() {
    MoveResult::Success(_) => game.update_current_player(),
    MoveResult::Winner(color) => game.finish_game(color),
    MoveResult::Error(msg) => println!("handle_botove_result: MoveResult::Error: {}", msg),
  }
  move_result
}

/// Algorithm:
/// 1. try to jump to home row
/// 2. try to jump to finish
/// 3. try to add new piece if only 1 is in game
/// 4. try to remove enemy's piece
/// 5. add new piece to game
/// 6. move any piece in game
/// 7. move piece in home row
// TODO: can be removed if move_bot_2() is used
pub fn make_a_move_bot(game: &mut Game) -> MoveResult {
  let player = game.get_player(game.current_player);

  let dice_value = throw_dice();

  // skip bot's move
  if dice_value == 18 {
    game.update_current_player();
    return MoveResult::Success("Player skipped.".into());
  }

  let positions = game.get_players_pieces_positions(game.current_player);

  let piece_positions_to_jump_home: Vec<usize> = positions
    .clone()
    .into_iter()
    .filter(|position| game.can_jump_to_home(*position, dice_value))
    .collect();

  // we can choose first piece to move, since all of them will end up in home
  if !piece_positions_to_jump_home.is_empty() {
    let move_result = game.execute_move(piece_positions_to_jump_home[0], dice_value, false);
    return handle_bot_move_result(game, move_result);
  }

  let piece_positions_to_jump_to_finish: Vec<usize> = positions
    .clone()
    .into_iter()
    .filter(|position| game.can_jump_to_finish(*position, dice_value))
    .collect();

  // we can choose first piece to move, since all of them will end up in home
  if !piece_positions_to_jump_to_finish.is_empty() {
    let move_result = game.execute_move(piece_positions_to_jump_to_finish[0], dice_value, false);
    return handle_bot_move_result(game, move_result);
  }

  // prioritize moving other pieces to promoting new one if bot has more pieces in game
  if player.pawns_at_start - player.pawns_at_finish < 2
    && dice_value > 6
    && game.can_promote_piece(dice_value)
  {
    let move_result = game.promote_piece(dice_value);
    return handle_bot_move_result(game, move_result);
    // return MoveResult::Success(String::from("Piece promoted."));
  }

  //let piece_positions_to_remove_enemy = game.get_players_pieces_positions(&game.current_player).clone();
  let piece_positions_to_remove_enemy: Vec<usize> = positions
    .clone()
    .into_iter()
    .filter(|position| game.will_remove_enemy(*position, dice_value))
    .collect();

  // we can choose a random piece to move (i.e. is not blocked).. or a piece that's closest to home for example
  if !piece_positions_to_remove_enemy.is_empty() {
    let move_result = game.execute_move(piece_positions_to_remove_enemy[0], dice_value, false);
    return handle_bot_move_result(game, move_result);
  }

  if dice_value > 6 && game.can_promote_piece(dice_value) {
    let move_result = game.promote_piece(dice_value);
    return handle_bot_move_result(game, move_result);
    // return MoveResult::Success(String::from("Piece promoted."));
  }

  let piece_positions_to_move: Vec<usize> = positions
    .into_iter()
    .filter(|position| game.can_jump(*position, dice_value))
    .collect();

  // choose piece closest to finish that is not in home
  if !piece_positions_to_move.is_empty() {
    let move_result =
      game.execute_move(*piece_positions_to_move.last().unwrap(), dice_value, false);
    return handle_bot_move_result(game, move_result);
  }

  let piece_positions_in_home_row: Vec<usize> = player
    .home
    .clone()
    .into_iter()
    .enumerate()
    .filter(|(_position, field)| game.is_occupied_by(field, game.current_player))
    .map(|(position, _field)| position)
    .filter(|&position| game.can_jump_from_home(position, dice_value))
    .collect();

  if !piece_positions_in_home_row.is_empty() {
    let move_result = game.execute_move(
      *piece_positions_in_home_row.last().unwrap(),
      dice_value,
      true,
    );
    return handle_bot_move_result(game, move_result);
  }

  // no moves found for player/bot, should be skipped - we have to return MoveResult::Success,
  //   for move_bot() in move_bot.rs to work properly
  // i.e. we just switch to the next player without making any changes (has no way of giving this
  //   info to front-end as of now ?)
  game.update_current_player();
  MoveResult::Success("Player skipped.".into())
  // MoveResult::Error(String::from("Can't move"))
}
