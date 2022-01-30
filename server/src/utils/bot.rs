use crate::models::game::Game;
use crate::models::player::Player;
use crate::utils::dice::{get_dice_value, throw_dice};
use crate::utils::enums::MoveResult;
use rand::prelude::SliceRandom;

const BOT_NAMES: Vec<&str> = vec!["Wade", "Dave", "Seth", "Ivan", "Riley", "Gilbert", "Jorge", "Dan",
                                  "Brian", "Roberto", "Ramon", "Miles", "Liam", "Nathaniel", "Ethan",
                                  "Lewis", "Milton", "Claude", "Joshua", "Glen"];
const BOT_SURNAMES: Vec<&str> = vec!["Williams", "Harris", "Thomas", "Robinson", "Walker", "Scott",
                                     "Nelson", "Mitchell", "Morgan", "Cooper", "Howard", "Davis",
                                     "Miller", "Martin", "Smith", "Anderson", "White", "Perry",
                                     "Clark", "Richards"];

pub fn create_bot_name() -> String {
    format!("{} {}", *BOT_NAMES.choose(&mut rand::thread_rng()), *BOT_SURNAMES.choose(&mut rand::thread_rng()))
}

pub fn make_a_move_bot(game: &mut Game, player: &mut Player) -> MoveResult {

    let dice_value = throw_dice();

    let piece_positions = game.get_players_pieces_positions(&game.current_player);

    let piece_positions_to_jump_home: Vec<usize> = piece_positions
        .iter()
        .filter(|&position| game.can_jump_to_home(*position, dice_value))
        .collect();

    // we can choose first piece to move, since all of them will end up in home
    if !piece_positions_to_jump_home.is_empty() {
        return game.execute_move(piece_positions_to_jump_home[0], dice_value, false) // TODO false
    }

    // prioritize moving other pieces to promoting new one if bot has more pieces in game
    if player.pawns_at_start - player.pawns_at_finish < 2 && dice_value > 6 && game.is_start_empty() {
        game.promote_piece();
        return MoveResult::Success(String::from("Piece promoted."))
    }

    let piece_positions_to_remove_enemy: Vec<usize> = piece_positions
        .iter()
        .filter(|&position| game.will_remove_enemy(*position, dice_value))
        .collect();

    // we can choose a random piece to move (i.e. is not blocked).. or a piece that's closest to home for example
    if !piece_positions_to_remove_enemy.is_empty() {
        return game.execute_move(piece_positions_to_remove_enemy[0], dice_value, false); // TODO false
    }

    let piece_positions_to_move: Vec<usize> = piece_positions
        .iter()
        .filter(|&position| game.can_jump(*position, dice_value))
        .collect();

    // choose piece closest to finish that is not in home
    if !piece_positions_to_move.is_empty() {
        return game.execute_move(*piece_positions_to_move.last().unwrap(), dice_value, false); // TODO false
    }

    if dice_value > 6 && game.is_start_empty() {
        game.promote_piece();
        return MoveResult::Success(String::from("Piece promoted."));
    }

    // TODO move in home column
    // let piece_positions_in_home_row Vec<usize> = player.home
    //     .iter()
    //     .enumerate()
    //     .filter(|&(position, field)| game.is_occupied_by(field, &game.current_player))
    //     .map(|&(position, field)| position)
    //     .filter(|&position| game.can_jump(*position, dice_value))
    //     .collect();

    return MoveResult::Error(String::from(""));

}