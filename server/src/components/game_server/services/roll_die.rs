use super::super::actor::GameServerState;
use crate::{
  components::{
    game::database,
    game_server::utils::{send_message, send_message_to_room},
  },
  models::actor_messages::ClientActorMessage,
  utils::{dice::get_dice_value, enums::ServerMessage},
};

pub async fn roll_dice(state: GameServerState, msg: ClientActorMessage) {
  let roll = get_dice_value();
  let db_game = database::find_game(&state.db, &msg.room_id).await;
  let game = match db_game {
    Ok(Some(game)) => game,
    _ => {
      let message =
        serde_json::to_string(&ServerMessage::Error("Cannot find game".into())).unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
      return;
    }
  };
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

  let res = database::add_dice_roll(&state.db, &msg.room_id, roll).await;

  if res.is_err() {
    let message =
      serde_json::to_string(&ServerMessage::Error("Error while rolling dice".into())).unwrap();
    send_message(message.as_str(), state.sessions, &msg.player_id);
    return;
  };

  let message = serde_json::to_string(&ServerMessage::DiceValue(roll, false)).unwrap();

  send_message_to_room(message.as_str(), state.sessions, state.rooms, &msg.room_id);
}
