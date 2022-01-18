use actix_web::{middleware, App, HttpServer};
use env_logger::Env;

mod components;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  env_logger::init_from_env(Env::default().default_filter_or("info"));

  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .configure(components::info::routes::attach_routes)
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await?;

  Ok(())
}
