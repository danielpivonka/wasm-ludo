use super::super::actor::GameServerState;
use crate::{
  components::{
    game::database,
    game_server::utils::{send_message, send_message_to_room},
  },
  models::actor_messages::ClientActorMessage,
  utils::enums::ServerMessage,
};

pub async fn start_game(state: GameServerState, msg: ClientActorMessage) {
  let res = database::start_game(&state.db, &msg.room_id).await;

  if res.is_err() {
    let message =
      serde_json::to_string(&ServerMessage::Error("Cannot start the game".into())).unwrap();
    send_message(message.as_str(), state.sessions, &msg.player_id);
    return;
  };

  let message = serde_json::to_string(&ServerMessage::GameStarted).unwrap();

  send_message_to_room(message.as_str(), state.sessions, state.rooms, &msg.room_id);
}
