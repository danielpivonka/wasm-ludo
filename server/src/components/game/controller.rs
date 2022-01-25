use actix_web::{get, post, web, HttpResponse};
use futures::stream::TryStreamExt;

use crate::{
  models::{color::Color, game::Game, player::Player},
  types::WebAppData,
};

#[get("")]
pub async fn get_games(data: web::Data<WebAppData>) -> HttpResponse {
  let db = &data.lock().await.db;
  let game_collection = db.collection::<Game>("games");

  let mut cursor = game_collection.find(None, None).await.unwrap();

  let mut games: Vec<Game> = Vec::new();
  while let Some(game) = cursor.try_next().await.unwrap() {
    games.push(game);
  }

  HttpResponse::Ok().json(games)
}

#[post("")]
pub async fn post_game(data: web::Data<WebAppData>) -> HttpResponse {
  let db = &data.lock().await.db;
  let game_collection = db.collection::<Game>("games");

  // just a mocked game, TODO: create new method for struct Game
  let mock_game = Game {
    id: "mock_game_id".to_string(),
    started_at: mongodb::bson::DateTime::now(),
    finished_at: None,
    fields: vec![Some(Color::Red), None],
    players: vec![Player {
      color: Color::Red,
      pawns_at_start: 4,
      player_id: "mock_player_id".to_string(),
      home: vec![None, None, None, None],
    }],
  };

  let res = game_collection.insert_one(mock_game, None).await;

  match res {
    Ok(_) => HttpResponse::Ok().body("Ok"),
    Err(_) => HttpResponse::InternalServerError().body("Whoops"),
  }
}
