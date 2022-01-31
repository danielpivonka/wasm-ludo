use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ClientMessage {
  ThrowDice,
  MoveFigure(usize),
  PromotePiece, // shouldn't need to pass color, since server should has attr current_player
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
  // NewPlayer(String),
  // PlayerReconnected(usize),
  // PlayerDisconnected(usize),
  // GameOver(Color),  // winner of a game
}

pub enum MoveResult {
  Success(String),
  Error(String),
}
