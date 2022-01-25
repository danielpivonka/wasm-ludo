use actix_web::web;

use super::controller;

pub fn attach_routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/games")
      .service(controller::get_games)
      .service(controller::post_game),
  );
}
