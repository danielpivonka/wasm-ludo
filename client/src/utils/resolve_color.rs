use crate::models::color::Color;

pub fn resolve_bg_color_class(color: &Color) -> String {
  match color {
    Color::Red => "bg-red-400".into(),
    Color::Green => "bg-green-400".into(),
    Color::Blue => "bg-blue-400".into(),
    Color::Yellow => "bg-yellow-400".into(),
  }
}

pub fn resolve_text_color_class(color: &Color) -> String {
  match color {
    Color::Red => "text-red-400".into(),
    Color::Green => "text-green-400".into(),
    Color::Blue => "text-blue-400".into(),
    Color::Yellow => "text-yellow-400".into(),
  }
}
