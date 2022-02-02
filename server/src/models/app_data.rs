use std::sync::{Arc, Mutex};

use actix::Addr;
use mongodb::Database;

use crate::components::game_server::actor::GameServer;

pub struct AppData {
  pub game_server_addr: Addr<GameServer>,
  pub db: Arc<Mutex<Database>>,
}
