use crate::components::game::database;
use crate::components::game_server::actor::GameServerState;
use crate::components::game_server::utils::{send_message, send_message_to_room};
use crate::models::actor_messages::ClientActorMessage;
use crate::models::game::Game;
use crate::utils::bot::make_a_move_bot;
use crate::utils::enums::{MoveResult, ServerMessage};

pub async fn move_bot(state: GameServerState, msg: &ClientActorMessage, game_state: &mut Game) {
  while game_state.is_current_player_ai() {
    let result = make_a_move_bot(game_state);
    match result {
      MoveResult::Success(_) => {
        let game_state = database::update_game_state(&state.db, &msg.room_id, game_state)
          .await
          .unwrap();
        let update_message = serde_json::to_string(&ServerMessage::GameUpdate(game_state)).unwrap();
        send_message_to_room(
          update_message.as_str(),
          state.sessions.clone(),
          state.rooms.clone(),
          &msg.room_id,
        );
      }
      MoveResult::Winner(_) => {
        game_state.finish_game();
        let game_state = database::update_game_state(&state.db, &msg.room_id, game_state)
          .await
          .unwrap();
        let update_message = serde_json::to_string(&ServerMessage::GameUpdate(game_state)).unwrap();
        send_message_to_room(
          update_message.as_str(),
          state.sessions.clone(),
          state.rooms.clone(),
          &msg.room_id,
        );

        // to break while loop
        return;
      }
      _ => {
        println!("Bot move failed.");
        return;
        // let message =
        //     serde_json::to_string(&ServerMessage::Error("Error executing move".into())).unwrap();
        // send_message(message.as_str(), state.sessions.clone(), &msg.player_id);
      }
    }
  }
}
