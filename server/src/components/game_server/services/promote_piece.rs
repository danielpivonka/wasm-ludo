use super::super::actor::GameServerState;
use crate::{
  components::{
    game::database,
    game_server::utils::{send_message, send_message_to_room},
  },
  models::actor_messages::ClientActorMessage,
  utils::{dice::get_dice_value, enums::ServerMessage,player::get_available_positions},
};

pub async fn promote_piece(state: GameServerState, msg: ClientActorMessage) {
}