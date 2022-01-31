use crate::models::game::Game;
use crate::utils::dice::{throw_dice};
use crate::utils::enums::MoveResult;
use rand::Rng;

pub fn create_bot_name() -> String {
    let names = ["Wade", "Dave", "Seth", "Ivan", "Riley", "Gilbert", "Jorge", "Dan",
        "Brian", "Roberto", "Ramon", "Miles", "Liam", "Nathaniel", "Ethan", "Lewis", "Milton", "Claude",
        "Joshua", "Glen"];
    let surnames = ["Williams", "Harris", "Thomas", "Robinson", "Walker", "Scott",
        "Nelson", "Mitchell", "Morgan", "Cooper", "Howard", "Davis", "Miller", "Martin", "Smith",
        "Anderson", "White", "Perry", "Clark", "Richards"];
    let mut rng = rand::thread_rng();
    format!("{} {}", names[rng.gen_range(0..21)], surnames[rng.gen_range(0..21)])
}

/// Algorithm:
/// 1. try to jump to home row
/// 2. try to add new piece if only 1 is in game
/// 3. try to remove enemy's piece
/// 4. add new piece to game
/// 5. move any piece in game
/// 6. move piece in home row
pub fn make_a_move_bot(game: &mut Game) -> MoveResult {

    let player = game.get_player(game.current_player);

    let dice_value = throw_dice();

    let positions = game.get_players_pieces_positions(&game.current_player);

    let piece_positions_to_jump_home: Vec<usize> = positions
        .clone()
        .into_iter()
        .filter(|position| game.can_jump_to_home(*position, dice_value))
        .collect();

    // we can choose first piece to move, since all of them will end up in home
    if !piece_positions_to_jump_home.is_empty() {
        return game.execute_move(piece_positions_to_jump_home[0], dice_value, false)
    }

    // prioritize moving other pieces to promoting new one if bot has more pieces in game
    if player.pawns_at_start - player.pawns_at_finish < 2 && dice_value > 6 && game.can_promote_piece(dice_value) {
        game.promote_piece(dice_value);
        return MoveResult::Success(String::from("Piece promoted."))
    }

    //let piece_positions_to_remove_enemy = game.get_players_pieces_positions(&game.current_player).clone();
    let piece_positions_to_remove_enemy: Vec<usize> = positions
        .clone()
        .into_iter()
        .filter(|position| game.will_remove_enemy(*position, dice_value))
        .collect();

    // we can choose a random piece to move (i.e. is not blocked).. or a piece that's closest to home for example
    if !piece_positions_to_remove_enemy.is_empty() {
        return game.execute_move(piece_positions_to_remove_enemy[0], dice_value, false);
    }

    if dice_value > 6 && game.can_promote_piece(dice_value) {
        game.promote_piece(dice_value);
        return MoveResult::Success(String::from("Piece promoted."));
    }

    let piece_positions_to_move: Vec<usize> = positions
        .clone()
        .into_iter()
        .filter(|position| game.can_jump(*position, dice_value))
        .collect();

    // choose piece closest to finish that is not in home
    if !piece_positions_to_move.is_empty() {
        return game.execute_move(*piece_positions_to_move.last().unwrap(), dice_value, false);
    }

    //TODO add method to check if piece can promote to finish

    let piece_positions_in_home_row: Vec<usize> = player.home
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_position, field)| game.is_occupied_by(field, &game.current_player))
        .map(|(position, _field)| position)
        .filter(|&position| game.can_jump(position.clone(), dice_value))
        .collect();

    if !piece_positions_in_home_row.is_empty() {
        return game.execute_move(*piece_positions_in_home_row.last().unwrap(), dice_value, true);
    }

    // cannot move
    return MoveResult::Error(String::from(""));

}