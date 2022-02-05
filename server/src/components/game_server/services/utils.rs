use crate::components::game::database;
use crate::components::game_server::actor::GameServerState;
use crate::components::game_server::utils::send_message_to_room;
use crate::models::actor_messages::ClientActorMessage;
use crate::models::game::Game;
use crate::utils::enums::ServerMessage;

// update game, sends SkipPlayer message and GameUpdate message to room,
pub async fn skip_player(
  state: GameServerState,
  msg: &ClientActorMessage,
  game: &mut Game,
) -> Game {
  game.update_current_player();
  game.dice_throws.clear();

  let skip_message = serde_json::to_string(&ServerMessage::SkipPlayer).unwrap();
  send_message_to_room(
    skip_message.as_str(),
    state.sessions.clone(),
    state.rooms.clone(),
    &msg.room_id,
  );

  send_game_update_message(state.clone(), msg, game).await
}

pub async fn send_roll_message(
  state: GameServerState,
  msg: &ClientActorMessage,
  roll: usize,
  can_roll_again: bool,
) {
  let roll_message =
    serde_json::to_string(&ServerMessage::DiceValue(roll, can_roll_again)).unwrap();
  send_message_to_room(
    roll_message.as_str(),
    state.sessions.clone(),
    state.rooms,
    &msg.room_id,
  );
}

pub async fn send_game_update_message(
  state: GameServerState,
  msg: &ClientActorMessage,
  game: &Game,
) -> Game {
  let game = database::update_game_state(&state.db, &msg.room_id, game)
    .await
    .unwrap(); //TODO handle errors
  let update_message = serde_json::to_string(&ServerMessage::GameUpdate(game.clone())).unwrap();

  send_message_to_room(
    update_message.as_str(),
    state.sessions.clone(),
    state.rooms.clone(),
    &msg.room_id,
  );
  game
}
