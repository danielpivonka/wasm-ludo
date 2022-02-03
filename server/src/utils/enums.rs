use serde::{Deserialize, Serialize};

use crate::models::{color::Color, game::Game, position::Position};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ClientMessage {
  ThrowDice,
  MoveFigure(usize, Option<Color>),
  PromotePiece, // shouldn't need to pass color, since server should has attr current_player
  StartGame,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "command", content = "payload")]
pub enum ServerMessage {
  DiceValue(usize, bool), // response to ThrowDice - bool: whether player should throw again
  AvailablePositions(Vec<usize>, Vec<usize>, bool), // position of pieces that can make a valid move (based on dice value)
  // AvailablePositions(available_positions_in_main_board, available_positions_in_home, can_promote)
  SkipPlayer,             // followed by GameUpdate ?
  MoveSuccessful(String), // followed by GameUpdate ?
  MoveFailed(String), // shouldn't happen, but player chooses a different position / throws again ??
  PiecePromoted, // response to PromotePiece - maybe use MoveSuccessful("Piece promoted") instead ?
  Information(String),
  GameUpdate(Game),
  PlayerConnected(Color),
  PlayerDisconnected(Color),
  // GameOver(Color),  // winner of a game
  PlayerCountChange(usize),
  GameOver(Color), // winner of a game
  GameStarted(Game),
  Error(String),
}

// TODO: make MoveResult more detailed - instead of just Success/Error:
// PiecePromoted, PieceFinished, ...
// (MoveBlocked / MoveSkipped / ... depends on whether this is checked in FE -
//   server provides precomputed positions of player's pieces which can be moved)
//
pub enum MoveResult {
  Winner(Color),
  Success(String),
  Error(String),
}
pub enum MoveType {
  Promote,
  Move(Position),
}
