use crate::components::game::database::{find_game, update};
use crate::models::color::Color;
use crate::models::game::Game;
use crate::models::player::Player;
use crate::utils::bot::{create_bot_name, make_a_move_bot};
use crate::utils::enums::MoveResult;
use crate::utils::player::make_a_move_player;
use mongodb::bson::Document;
use mongodb::{Database, bson::{self, doc}};
use std::sync::{Arc, Mutex};

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
pub async fn play_round(
  db: &Arc<Mutex<Database>>,
  mut game: Game,
  position: usize,
) -> MoveResult {

  let player = game.get_current_player();

  // let mut move_result = match player.is_bot {
  //   true => make_a_move_bot(&mut game),
  //   false => make_a_move_player(&mut game, position, is_home),
  // };

  if let MoveResult::Success(_) = move_result {
    game.update_current_player();
    game.dice_throws.clear();
  }

  if let Some(winner) = game.check_winner() {
    move_result = MoveResult::Winner(winner);
  }

  // match move_result {
  //   MoveResult::Winner(winner) => {
  //     // TODO: handle errors
  //     let make_doc = || -> anyhow::Result<Document> {
  //       let fields = bson::to_bson(&game.fields)?;
  //       let players = bson::to_bson(&game.players)?;
  //       let current_player = bson::to_bson(&game.current_player)?;
  //       let finished_at = bson::to_bson(&mongodb::bson::DateTime::now())?;
  //       let doc = doc! { "$set": { "fields": fields, "players": players, "current_player": current_player, "finished_at": finished_at } };
  //       Ok(doc)
  //     };
  //     let doc = match make_doc() {
  //       Ok(doc) => doc,
  //       Err(_) => return MoveResult::Error("failed to create document".into()),
  //     };
  //     if let Err(err) = update(db, game_id.as_str(), doc).await {
  //       return MoveResult::Error(err.to_string());
  //    };
  //     MoveResult::Winner(winner)
  //   }
  //   MoveResult::Success(msg) => {
  //     let make_doc = || -> anyhow::Result<Document> {
  //       let fields = bson::to_bson(&game.fields)?;
  //       let players = bson::to_bson(&game.players)?;
  //       let current_player = bson::to_bson(&game.current_player)?;
  //       let dice_throws: Vec<usize> = Vec::new();
  //       let bson_dice_throws = bson::to_bson(&dice_throws)?;
  //       let doc = doc! { "$set": { "fields": fields, "players": players, "current_player": current_player, "dice_throws": &bson_dice_throws } };
  //       Ok(doc)
  //     };
  //     let doc = match make_doc() {
  //       Ok(doc) => doc,
  //       Err(_) => return MoveResult::Error("failed to create document".into()),
  //     };
  //     if let Err(err) = update(db, game_id.as_str(), doc).await {
  //       return MoveResult::Error(err.to_string());
  //     };
  //     MoveResult::Success(msg)
  //   }
  //   MoveResult::Error(msg) => MoveResult::Error(msg),
  // }
}
