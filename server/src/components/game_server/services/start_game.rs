use crate::{models::actor_messages::ClientActorMessage, components::{game::database, game_server::utils::send_message_to_room}, utils::enums::ServerMessage};
use super::super::actor::GameServerState;

pub async fn start_game(state: GameServerState, msg: ClientActorMessage) {
  let res = database::start_game(&state.db, &msg.room_id).await;
  
  if res.is_err() {
    return
  };

  let message = serde_json::to_string(&ServerMessage::GameStarted).unwrap();

  send_message_to_room(message.as_str(), state.sessions, state.rooms, &msg.player_id);
}