use actix_web::web;

use super::controller;

pub fn attach_routes(config: &mut web::ServiceConfig) {
  config.service(web::scope("/info").service(controller::get_info));
}
