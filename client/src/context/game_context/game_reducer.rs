use std::{collections::HashMap, rc::Rc};
use yew::Reducible;

use crate::models::{color::Color, die_info::DieInfo, game::Game, messages::ServerMessage};

#[derive(Clone, Debug, PartialEq)]
pub struct GameState {
  pub game: Game,
  pub player_color: Color,
  pub player_count: u32,
  pub dice_info: HashMap<Color, DieInfo>,
}

impl Default for GameState {
  fn default() -> Self {
    Self {
      game: Game::new(),
      player_color: Color::Green,
      player_count: 0,
      dice_info: [
        (Color::Green, DieInfo::new()),
        (Color::Yellow, DieInfo::new()),
        (Color::Blue, DieInfo::new()),
        (Color::Red, DieInfo::new()),
      ]
      .iter()
      .cloned()
      .collect::<HashMap<_, _>>(),
    }
  }
}

impl Reducible for GameState {
  type Action = ServerMessage;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    match action {
      ServerMessage::DiceValue(number, can_roll) => {
        let mut new_dice_info = self.dice_info.clone();
        new_dice_info.insert(
          self.game.current_player.clone(),
          DieInfo { number, can_roll },
        );

        Self {
          dice_info: new_dice_info,
          ..(*self).clone()
        }
        .into()
      }
      ServerMessage::GameUpdate(game) | ServerMessage::GameStarted(game) => {
        let current_player = game.current_player.clone();
        let dice_info = self.dice_info.iter().map(|(color, die_info)| {
          let can_roll = current_player == *color;
          let die_info = DieInfo {
            can_roll,
            ..die_info.clone()
          };
          (color.clone(), die_info)
        });

        Self {
          game,
          dice_info: dice_info.collect(),
          ..(*self).clone()
        }
        .into()
      }
      ServerMessage::ConnectResponse(game, player_color) => {
        let current_player = game.current_player.clone();
        let dice_info = self.dice_info.iter().map(|(color, die_info)| {
          let can_roll = current_player == *color;
          let die_info = DieInfo {
            can_roll,
            ..die_info.clone()
          };
          (color.clone(), die_info)
        });

        Self {
          game,
          player_color,
          dice_info: dice_info.collect(),
          ..(*self).clone()
        }
        .into()
      }
      _ => self,
    }
  }
}
