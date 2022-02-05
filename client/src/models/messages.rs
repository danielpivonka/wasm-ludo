use serde::{Deserialize, Serialize};

use super::{color::Color, game::Game};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "command", content = "payload")]
pub enum ServerMessage {
  DiceValue(usize, bool), // response to ThrowDice - bool: whether player should throw again
  AvailablePositions(Vec<usize>, Vec<usize>, bool), // position of pieces that can make a valid move (based on dice value)
  SkipPlayer,                                       // followed by GameUpdate ?
  PiecePromoted, // response to PromotePiece - maybe use MoveSuccessful("Piece promoted") instead ?
  GameUpdate(Game),
  PlayerCountChange(usize),
  GameStarted(Game),
  Error(String),
  ConnectResponse(Game, Color),
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum ClientMessage {
  ThrowDice,
  MoveFigure(usize, Option<Color>),
  PromotePiece, // shouldn't need to pass color, since server should has attr current_player
  StartGame,
}
