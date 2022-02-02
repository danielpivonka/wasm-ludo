use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use components::game_server::actor::GameServer;
use dotenv::dotenv;
use env_logger::Env;
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::sync::{Arc, Mutex};

mod components;
mod models;
mod types;
mod utils;

use models::app_data::AppData;
use crate::utils::enums::MoveResult;
use crate::models::color::Color;
//
// #[actix_web::main]
// async fn main() -> anyhow::Result<()> {
//   env_logger::init_from_env(Env::default().default_filter_or("info"));
//   dotenv().ok();
//
//   let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable is not set");
//
//   let mut client_options = ClientOptions::parse(database_url).await?;
//   client_options.app_name = Some("Ludo".to_string());
//
//   let client = Client::with_options(client_options)?;
//
//   let app_data = web::Data::new(Mutex::new(AppData {
//     db: client.database("main"),
//   }));
//
//   HttpServer::new(move || {
//     App::new()
//       .app_data(app_data.clone())
//       .wrap(middleware::Logger::default())
//       .configure(components::info::routes::attach_routes)
//       .configure(components::game::routes::attach_routes)
//   })
//   .bind("127.0.0.1:8080")?
//   .run()
//   .await?;
//
//   Ok(())
// }

fn main() {
  // let mut game = get_empty_game();
  // game.current_player = Color::Yellow;
  //
  // let dice_value = 6 + 8;
  // let opponent_color = Color::Green;
  // let fields = game.fields.clone();
  // let starting_pos = game.get_starting_position();
  // let field_size = fields.len();
  //
  // game.fields[starting_pos + dice_value - 6] = Some(opponent_color);
  //
  // // set_field(&mut game.fields, starting_pos + dice_value - 6, Some(opponent_color));
  // let mut opponent = game.get_player_mut(opponent_color);
  // opponent.pawns_at_start = 3;
  //
  // let mut game = game.clone();
  // match game.execute_move(PROMOTE_PIECE, dice_value, false) {
  //   MoveResult::Error(_) => assert!(false),
  //   MoveResult::Success(_) => assert!(true)
  // }
  //
  // assert_eq!(game.get_current_player().pawns_at_start, 3);
  // assert_eq!(game.get_player(opponent_color).pawns_at_start, 4);
  // assert!(is_empty_field(&fields, 8));
  // assert!(is_occupied_field_by(&fields, 8 + 8, Color::Yellow));
  // assert_eq!(empty_fields_count(&fields), field_size - 1);

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable is not set");

  let mut client_options = ClientOptions::parse(database_url).await?;
  client_options.app_name = Some("Ludo".to_string());

  let client = Client::with_options(client_options)?;
  let db = Arc::new(Mutex::new(client.database("main")));

  let game_server_addr = GameServer::new(db.clone()).start();

  let app_data = web::Data::new(AppData {
    game_server_addr,
    db: db.clone(),
  });

  HttpServer::new(move || {
    App::new()
      .wrap(
        Cors::default()
          .allow_any_header()
          .allow_any_origin()
          .allow_any_method(),
      )
      .app_data(app_data.clone())
      .wrap(middleware::Logger::default())
      .configure(components::game::routes::attach_routes)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await?;

  Ok(())
}
