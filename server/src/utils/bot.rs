use crate::models::game::Game;
use crate::models::player::Player;
use crate::utils::dice::get_dice_value;
use crate::utils::enums::MoveResult;

//TODO add fn for generating bot player names

pub fn throw_dice_bot(game: &Game) -> usize {

    match get_dice_value() {
        // if AI can promote, it should (always?) promote
        //   - or should we also consider getting our piece to home / jumping on opponents' pieces?
        6 => {
            match game.can_promote_piece() {
                true => 6,
                // if can't promote, keep throwing dice
                false => match get_dice_value() {
                    6 => match get_dice_value() {
                        // 3x6 => 0
                        6 => 0,
                        n => 6 + 6 + n,
                    },
                    n => 6 + n
                }
            }
        },
        n => n
    }
}


pub fn make_a_move_bot(game: &mut Game, player: &mut Player) -> MoveResult {

    let dice_value = throw_dice_bot(game);

    if dice_value == 6 {
        game.promote_piece();
        return MoveResult::Success(String::from("Piece promoted."))
    }

    // if dice_value = 6, we can promote (was checked)
    // otherwise, check available moves

    let piece_positions = game.get_players_pieces_positions(&game.current_player);

    let piece_positions_to_jump_home: Vec<usize> = piece_positions
        .iter()
        .filter(|&position| game.can_jump_to_home(*position, dice_value))
        .collect();

    // we can choose a random piece to move, since all of them will end up in home
    if !piece_positions_to_jump_home.is_empty() {
        return game.execute_move(piece_positions_to_jump_home[0], dice_value, false) // TODO false
    }

    // otherwise, we will check if we can move any piece at all (currently we won't try to remove
    //   opponents' pieces)
    let piece_positions_to_move: Vec<usize> = piece_positions
        .iter()
        .filter(|&position| game.can_jump(*position, dice_value))
        .collect();

    // we can choose a random piece to move (i.e. is not blocked).. or a piece that's closest to home for example
    match !piece_positions_to_move.is_empty() {
        true => game.execute_move(piece_positions_to_move[0], dice_value, false), // TODO false
        // no valid move
        false => MoveResult::Error(String::from("No valid move."))
    }

}