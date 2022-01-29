use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use mongodb::{options::ClientOptions, Client};
use std::env;
use tokio::sync::Mutex;

mod components;
mod models;
mod types;
mod utils;

use models::app_data::AppData;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  env_logger::init_from_env(Env::default().default_filter_or("info"));
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable is not set");

  let mut client_options = ClientOptions::parse(database_url).await?;
  client_options.app_name = Some("Ludo".to_string());

  let client = Client::with_options(client_options)?;

  let app_data = web::Data::new(Mutex::new(AppData {
    db: client.database("main"),
  }));

  HttpServer::new(move || {
    App::new()
      .app_data(app_data.clone())
      .wrap(middleware::Logger::default())
      .configure(components::info::routes::attach_routes)
      .configure(components::game::routes::attach_routes)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await?;

  Ok(())
}
