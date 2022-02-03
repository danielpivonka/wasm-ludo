use anyhow::anyhow;
use mongodb::{
  bson::{self, doc, oid::ObjectId, Bson, Document},
  options::{FindOneAndUpdateOptions, ReturnDocument},
  Database,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};

use crate::{
  models::{game::Game, player::Player, color::Color},
  types::Field,
};

pub async fn create_game(db: &Arc<Mutex<Database>>) -> anyhow::Result<String> {
  let db_mutex = db.lock().unwrap();
  let game_collection = db_mutex.collection::<Game>("games");
  let mock_game = Game::new();
  let res = game_collection.insert_one(mock_game, None).await;
  match res {
    Ok(result) => {
      if let Bson::ObjectId(id) = result.inserted_id {
        return Ok(format!("{}", id));
      } else {
        return Err(anyhow!("game id couldn't be parsed"));
      };
    }
    Err(e) => Err(anyhow!(e)),
  }
}

pub async fn add_player(
  db: &Arc<Mutex<Database>>,
  game_id: &str,
  new_player: Player,
) -> anyhow::Result<Game> {
  let serialized_player = bson::to_bson(&new_player)?;
  let update = doc! { "$push": { "players": serialized_player } };
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(anyhow!(err)),
  };
  let filter = doc! { "_id" : oid };
  return update_game(db, filter, update).await;
}

pub async fn find_game(db: &Arc<Mutex<Database>>, game_id: &str) -> anyhow::Result<Option<Game>> {
  let db_mutex = db.lock().unwrap();
  let game_collection = db_mutex.collection::<Game>("games");
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(anyhow!(err)),
  };
  let filter = doc! { "_id" : oid };
  let found = game_collection.find_one(filter, None).await;
  match found {
    Ok(result) => Ok(result),
    Err(e) => Err(anyhow!(e)),
  }
}

// pub async fn update_board(
//   db: &Arc<Mutex<Database>>,
//   game_id: &str,
//   fields: Vec<Field>, // TODO: replaced Color with Field - does that fuck up anything?
// ) -> anyhow::Result<Game> {
//   let serialized_fields = bson::to_bson(&fields)?;
//   let oid = match ObjectId::parse_str(game_id) {
//     Ok(res) => res,
//     Err(err) => return Err(anyhow!(err)),
//   };
//   let filter = doc! { "_id" : oid };
//   let update = doc! { "fields": serialized_fields };

//   return update_game(db, filter, update).await;
// }

// pub async fn update_player(
//   db: &Arc<Mutex<Database>>,
//   game_id: &str,
//   player: Player,
// ) -> anyhow::Result<Game> {
//   let oid = match ObjectId::parse_str(game_id) {
//     Ok(res) => res,
//     Err(err) => return Err(anyhow!(err)),
//   };
//   let serialized_player = bson::to_bson(&player)?;
//   let serialized_color = bson::to_bson(&player.color)?;

//   let filter = doc! { "_id" : oid, "players.color": serialized_color };
//   let update = doc! { "$set": { "players.$" : serialized_player } };
//   return update_game(db, filter, update).await;
// }

pub async fn start_game(db: &Arc<Mutex<Database>>, game_id: &str) -> anyhow::Result<Game> {
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(anyhow!(err)),
  };
  let filter = doc! { "_id" : oid };
  let update = doc! { "$set": { "started_at" : mongodb::bson::DateTime::now() } };
  return update_game(db, filter, update).await;
}

// pub async fn finish_game(db: &Arc<Mutex<Database>>, game_id: &str) -> anyhow::Result<Game> {
//   let oid = match ObjectId::parse_str(game_id) {
//     Ok(res) => res,
//     Err(err) => return Err(anyhow!(err)),
//   };
//   let filter = doc! { "_id" : oid };
//   let update = doc! { "$set": { "finished_at" : mongodb::bson::DateTime::now() } };
//   return update_game(db, filter, update).await;
// }

pub async fn add_dice_roll(
  db: &Arc<Mutex<Database>>,
  game_id: &str,
  roll: usize,
) -> anyhow::Result<Game> {
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(anyhow!(err)),
  };
  let serialized_roll = bson::to_bson(&roll)?;
  let filter = doc! { "_id" : oid };
  let update = doc! { "$push": { "dice_throws": serialized_roll } };
  return update_game(db, filter, update).await;
}

// pub async fn update_current_player(
//   db: &Arc<Mutex<Database>>,
//   game_id: &str,
//   current_player: Color,
// ) -> anyhow::Result<Game> {
//   let oid = match ObjectId::parse_str(game_id) {
//     Ok(res) => res,
//     Err(err) => return Err(anyhow!(err)),
//   };
//   let current_player_bson = bson::to_bson(&current_player)?;
//   let filter = doc! { "_id" : oid };
//   let update = doc! { "$set": { "current_player": current_player_bson } };
//   return update_game(db, filter, update).await;
// }

async fn update_game(
  db: &Arc<Mutex<Database>>,
  filter: Document,
  update: Document,
) -> anyhow::Result<Game> {
  let db_mutex = db.lock().unwrap();
  let game_collection = db_mutex.collection::<Game>("games");
  let option = FindOneAndUpdateOptions::builder()
  .return_document(ReturnDocument::After)
  .build();
  let res = game_collection
  .find_one_and_update(filter, update, option)
  .await;
  match res {
    Ok(Some(game)) => Ok(game),
    Ok(None) => Err(anyhow!("Game doesnt exits")),
    Err(e) => Err(anyhow!(e)),
  }
}

pub async fn update(
  db: &Arc<Mutex<Database>>,
  game_id: &str,
  update: Document,
) -> anyhow::Result<Game> {
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(anyhow!(err)),
  };
  let filter = doc! { "_id" : oid };
  let res = update_game(db,filter, update)
    .await;
  match res {
    Ok(game) => Ok(game),
    Err(e) => Err(anyhow!(e)),
  }
}


pub async fn update_game_state(
  db: &Arc<Mutex<Database>>,
  game_id: &str,
  game: &Game,
) -> anyhow::Result<Game> {
    let update_doc = match make_doc(game){
      Ok(doc)=> doc,
      _ =>return Err(anyhow!("Failed to create document")),
    };
    update(db,game_id,update_doc).await
}

fn make_doc(game: &Game)-> anyhow::Result<Document>{
  let fields = bson::to_bson(&game.fields)?;
  let players = bson::to_bson(&game.players)?;
  let current_player = bson::to_bson(&game.current_player)?;
  let dice_throws: Vec<usize> = Vec::new();
  let bson_dice_throws = bson::to_bson(&dice_throws)?;
  let doc = doc! { "$set": { "fields": fields, "players": players, "current_player": current_player, "dice_throws": &bson_dice_throws } };
  Ok(doc)
}
// pub fn make_bson<T>(values: &[&T]) -> anyhow::Result<Box<[Bson]>>
//   where T: ?Sized + Serialize {
//     match values.into_iter().map(|value| {
//       bson::to_bson(value)
//     }).collect::<Result<Box<[_]>, _>>() {
//       Ok(values) => Ok(values),
//       Err(err) => Err(anyhow!(err)),
//     }
//   }
