use super::super::actor::GameServerState;
use crate::components::game_server::services::move_bot::move_bot;
use crate::utils::bot::make_a_move_bot;
use crate::{
  components::{
    game::database,
    game_server::utils::{send_message, send_message_to_room},
  },
  models::actor_messages::ClientActorMessage,
  utils::{
    dice::get_dice_value,
    enums::{MoveResult, MoveType, ServerMessage},
    game::play_round,
    player::get_available_positions,
  },
};

pub async fn promote_piece(state: GameServerState, msg: ClientActorMessage) {
  let db_game = database::find_game(&state.db, &msg.room_id).await;
  let mut game = match db_game {
    Ok(Some(game)) => game,
    _ => {
      let message =
        serde_json::to_string(&ServerMessage::Error("Cannot find game".into())).unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
      return;
    }
  };
  println!("Roll at start promote: {:?}", game.dice_throws.iter());
  let current_player_id = game
    .players
    .iter()
    .find(|player| player.color == game.current_player)
    .unwrap()
    .id
    .clone(); //TODO probably shouldn't unwrap
  if current_player_id != msg.player_id {
    let message =
      serde_json::to_string(&ServerMessage::Error("It is not your turn".into())).unwrap();
    send_message(message.as_str(), state.sessions, &msg.player_id);
    return;
  };
  let result = play_round(&mut game, MoveType::Promote).await;
  match result {
    MoveResult::Success(_) => {
      let mut game_state = database::update_game_state(&state.db, &msg.room_id, &game)
        .await
        .unwrap();
      let update_message =
        serde_json::to_string(&ServerMessage::GameUpdate(game_state.clone())).unwrap();
      send_message_to_room(
        update_message.as_str(),
        state.sessions.clone(),
        state.rooms.clone(),
        &msg.room_id,
      );

      // handle if next player is a bot
      move_bot(state.clone(), &msg, &mut game_state).await;
    }
    MoveResult::Error(e) => {
      let message =
        serde_json::to_string(&ServerMessage::Error(format!("Error executing move: {}",e))).unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
    }
    _ => {
      let message = serde_json::to_string(&ServerMessage::Error("Unknown error".into())).unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
    }

  }
}
