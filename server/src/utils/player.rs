use crate::models::game::Game;
use crate::models::player::Player;
use crate::utils::dice::throw_dice;
use crate::utils::enums::MoveResult;

// we could check if any moves are possible for the player - that has to be done on FE?
// if player has no valid moves, he should be skipped (as it's done for AI),
//    but when/how we inform the client ?
// player/client can keep sending us positions, but if he has no valid moves at all,
//   we will keep giving MoveResult::Error
// idealne by client/player hned vedel, ze hrac nemoze tiahnut figurkou - teoreticky by
//    od clienta mohla prist poziadavka GetValidPositions - pozicie figuriek, s ktorymi moze
//    hrac tiahnut, a server ich posle na clienta (alebo message NoValidPositions)
pub fn make_a_move_player(game: &mut Game, player: &mut Player) -> MoveResult {
    let dice_value = throw_dice(); // TODO await
    let position: usize = 0; // TODO await message_from_client/player(); promote new piece = position 100
    game.execute_move(position, dice_value, false) // TODO set home column
}