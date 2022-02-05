use futures::SinkExt;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::icon::Icon;
use crate::components::pawn::Pawn;
use crate::context::game_context::context::GameContext;
use crate::models::color::Color;
use crate::models::messages::ClientMessage;
use crate::utils::{resolve_bg_color_class, resolve_text_color_class};

#[derive(PartialEq, Clone)]
pub enum FieldVariant {
  Home,
  Main,
}

#[derive(Properties, PartialEq, Clone)]
pub struct FieldProps {
  pub color: Color,
  #[prop_or(false)]
  pub color_background: bool,
  pub position: usize,
  pub raw_position: usize,
  pub variant: FieldVariant,
  pub arrow_class: String,
}

#[function_component(Field)]
pub fn field(props: &FieldProps) -> Html {
  let FieldProps {
    color,
    color_background,
    position,
    raw_position,
    variant,
    arrow_class,
  } = props.clone();
  let GameContext {
    game,
    sender,
    player_color,
    ..
  } = use_context::<GameContext>().expect("context not found");

  let bg_class = if color_background {
    resolve_bg_color_class(&color)
  } else {
    "".into()
  };

  let text_class = resolve_text_color_class(&color);

  let pawn_color = if variant == FieldVariant::Home {
    // TODO: add home pawns
    game
      .players
      .iter()
      .find(|player| player.color == color)
      .and_then(|player| player.home.get(position).unwrap_or(&None).clone())
  } else {
    game.fields.get(position)
  };

  let click_color = match variant {
    FieldVariant::Home => Some(color),
    FieldVariant::Main => None,
  };

  let onclick = {
    Callback::from(move |_| {
      let click_color = click_color.clone();
      let sender = sender.clone();
      spawn_local(async move {
        if let Some(mut sender) = sender.clone() {
          sender
            .0
            .send(ClientMessage::MoveFigure(position, click_color))
            .await
            .ok();
        }
      });
    })
  };

  let content = {
    if let Some(color) = pawn_color {
      html! { <Pawn color={color.clone()} onclick={(color == player_color).then(|| onclick)} /> }
    } else if raw_position == 6 {
      html! { <Icon class={classes!(arrow_class)} /> }
    } else {
      html! {}
    }
  };

  html! {
    <div class={classes!(String::from("relative border border-neutral-300 shadow-inner grid place-items-center"), bg_class, text_class)}>
      {content}
    </div>
  }
}
