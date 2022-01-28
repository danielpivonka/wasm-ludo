use actix_web::{get, post, web, HttpResponse};
use mongodb::{bson::{oid::ObjectId, doc, bson, self, Document}, results::UpdateResult};

use crate::{
  models::{game::Game, player::Player},
  types::WebAppData,
};

pub async fn create_game(data: web::Data<WebAppData>,creating_player:Player)->Result<String,mongodb::error::Error>{
    let db = &data.lock().await.db;
  let game_collection = db.collection::<Game>("games");
  let mock_game = Game::new(vec![creating_player.id]);

  let res = game_collection.insert_one(mock_game, None).await;
  match res {
    Ok(result) => Ok(result.inserted_id.to_string()),
    Err(e) => Err(e),
  }
}

pub async fn add_player(data: web::Data<WebAppData>,game_id :&str ,new_player:Player)->Result<UpdateResult,Box<dyn std::error::Error>>{
  let serialized_player = bson::to_bson(&new_player)?;
  let update =  doc! { "$push": { "players": serialized_player } };
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(Box::new(err)),
  };
  let filter = doc! { "_id" : oid };
  return update_game(data,filter,update).await;
}
pub async fn find_game(data: web::Data<WebAppData>,game_id :&str) -> Result<Option<Game>,Box<dyn std::error::Error>>{
  let db = &data.lock().await.db;
  let game_collection = db.collection::<Game>("games");
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(Box::new(err)),
  };
  let filter = doc! { "_id" : oid };
  let found = game_collection.find_one(filter, None).await;
  match found {
    Ok(result) => Ok(result),
    Err(e) => Err(Box::new(e)),
  }
}
pub async fn update_board(data: web::Data<WebAppData>,game_id :&str ,fields:Vec<Color>)->Result<UpdateResult,Box<dyn std::error::Error>>{
  
  let serialized_fields = bson::to_bson(&fields)?;
  let update =  doc! { "fields": serialized_fields };
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(Box::new(err)),
  };
  let filter = doc! { "_id" : oid };
  return update_game(data,filter, update).await;
}
async fn update_game(data: web::Data<WebAppData>,filter:Document,update:Document)->Result<UpdateResult,Box<dyn std::error::Error>>{
  let db = &data.lock().await.db;
  let game_collection = db.collection::<Game>("games");
  let res = game_collection.update_one(filter,update,None).await;
     match res {
      Ok(result) => Ok(result),
      Err(e) => Err(Box::new(e)),
    }
}
