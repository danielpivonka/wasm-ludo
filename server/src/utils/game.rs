use crate::models::color::Color;
use crate::models::game::Game;
use crate::models::player::Player;
use crate::utils::bot::{create_bot_name, make_a_move_bot};
use crate::utils::enums::MoveResult;
use crate::utils::player::make_a_move_player;
use std::sync::{Arc, Mutex};
use mongodb::Database;
use crate::components::game::database::{find_game, finish_game, update_board, update_player};

pub fn initialize_players(player_names: Vec<String>) -> Vec<Player> {
  let mut colors = [Color::Red, Color::Green, Color::Blue, Color::Yellow].iter();
  let mut players = vec![];
  for name in player_names {
    players.push(Player::new(
      "0".to_string(),
      name,
      *colors.next().unwrap(),
      false,
    ))
  }
  while players.len() < 4 {
    players.push(Player::new(
      "0".to_string(),
      create_bot_name(),
      *colors.next().unwrap(),
      true,
    ))
  }
  players
}


// called upon receiving either PromotePiece or MovePiece(position, Option<Color>)
// TODO: use struct Position { position: usize, is_home: bool }
pub async fn play_round(db: &Arc<Mutex<Database>>, game_id: String, position: usize, is_home: bool) -> MoveResult {

  let mut game = match find_game(db, game_id.as_str()).await {
    Ok(game) => match game {
      Some(game) => game,
      None => return MoveResult::Error("Couldn't find game.".into())
    },
    Err(_) => return MoveResult::Error("Couldn't find game.".into())
  };

  let player = game.get_current_player();

  let mut move_result = match player.is_bot {
    true => make_a_move_bot(&mut game),
    false => make_a_move_player(&mut game, position, is_home)
  };

  if let MoveResult::Success(_) = move_result {
    game.update_current_player();
    game.dice_throws.clear();
  }

  if let Some(winner) = game.check_winner() {
    move_result = MoveResult::Winner(winner);
  }

  match move_result {
    MoveResult::Winner(winner) => {
      // TODO: handle errors
      finish_game(db, game_id.as_str()).await.ok();
      MoveResult::Winner(winner)
    },
    MoveResult::Success(msg) => {
      update_board(db, game_id.as_str(), game.fields.clone()).await.ok();
      update_player(db, game_id.as_str(), game.get_current_player().clone()).await.ok();
      MoveResult::Success(msg)
    },
    MoveResult::Error(msg) => MoveResult::Error(msg)
  }
}
