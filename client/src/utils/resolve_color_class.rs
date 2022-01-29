use crate::models::color::Color;

pub fn resolve_color_class(color: &Color) -> String {
  match color {
    Color::Red => "bg-red-400".into(),
    Color::Green => "bg-green-400".into(),
    Color::Blue => "bg-blue-400".into(),
    Color::Yellow => "bg-yellow-400".into(),
  }
}