use serde::{Deserialize, Serialize};

use super::{color::Color, game::Game};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "command", content = "payload")]
pub enum ServerMessage {
  DiceValue(usize), // response to ThrowDice
  SkipPlayer,       // - if there are no moves, we have to move on to the next player?
  // - is handled in FE? (we can use 'GetAvailableMoves' util)
  //   since player can't really choose a piece/position to move (i.e. we ?
  MoveSuccessful(String), // we successfully moved a piece; msg = "Piece is now secure in your home"
  // or MoveSuccess
  MoveFailed(String), // or MoveFail / MoveError - if we can't make a certain move
  // - would overjump home; field is blocked by our piece ...
  PiecePromoted, // response to PromotePiece - is also MoveSuccessful, but
  Information(String),
  GameUpdate(Game),
  // NewPlayer(String),
  PlayerConnected(Color),
  PlayerDisconnected(Color),
  // GameOver(Color),  // winner of a game
  PlayerCountChange(usize),
  GameStarted,
}
