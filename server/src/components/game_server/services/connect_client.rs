use crate::{
  components::{
    game::database,
    game_server::{actor::GameServerState, utils::send_message},
  },
  models::actor_messages::Connect,
  utils::enums::ServerMessage,
};

pub async fn connect_client(state: GameServerState, msg: &Connect) {
  let game = database::find_game(&state.db, &msg.room_id).await;

  let game = match game {
    Ok(Some(game)) => game,
    Ok(None) => {
      let message = serde_json::to_string(&ServerMessage::Error("Game not found".into())).unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
      return;
    }
    Err(_) => {
      let message = serde_json::to_string(&ServerMessage::Error(
        "Server couldn't connect to database".into(),
      ))
      .unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
      return;
    }
  };

  let player = game.get_player_by_id(&msg.player_id);

  let player = match player {
    Some(player) => player,
    None => {
      let message = serde_json::to_string(&ServerMessage::Error(
        "Player with given id not found".into(),
      ))
      .unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
      return;
    }
  };

  let message =
    serde_json::to_string(&ServerMessage::ConnectResponse(game.clone(), player.color)).unwrap();
  send_message(message.as_str(), state.sessions, &msg.player_id);
}
