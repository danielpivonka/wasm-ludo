use actix_web::{web, HttpResponse};

use crate::{
  models::{game::Game, player::Player},
  types::WebAppData,
};

pub async fn create_game(data: web::Data<WebAppData>, creating_player: Player) -> HttpResponse {
  let db = &data.lock().await.db;
  let game_collection = db.collection::<Game>("games");
  let mock_game = Game::new(vec![creating_player.id]);

  let res = game_collection.insert_one(mock_game, None).await;
  match res {
    Ok(result) => HttpResponse::Ok().body(result.inserted_id.to_string()),
    Err(_) => HttpResponse::InternalServerError().body("Whoops"),
  }
}
