use tokio::sync::Mutex;

use crate::models::{app_data::AppData, color::Color};

pub type Field = Option<Color>;
pub type WebAppData = Mutex<AppData>;
