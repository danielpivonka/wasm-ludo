use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::models::color::Color;
use crate::types::Field;
use crate::utils::enums::MoveResult;

use super::player::Player;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
  // #[serde(serialize_with = "serialize_hex_string_as_object_id")]
  // pub oid: String,
  pub started_at: DateTime,
  pub finished_at: Option<DateTime>,
  pub fields: Vec<Field>,
  pub players: Vec<Player>,
  pub current_player: Color,
  pub dice_throws: Vec<usize>,
}

const PROMOTE_PIECE: usize = 100;

impl Game {
  pub fn new() -> Self {
    Game {
      started_at: mongodb::bson::DateTime::now(),
      finished_at: None,
      fields: vec![None; 52],
      players: vec![],
      current_player: Color::ordered().first().unwrap().clone(),
      dice_throws: vec![],
    }
  }

  // there should be at most one winner at a time, therefore we take the first
  //   player that meets the winning condition
  pub fn check_winner(&self) -> Option<Color> {
    for player in &self.players {
      if player.check_winner() {
        return Some(player.color);
      }
    }
    None
  }

  pub fn field_size(&self) -> usize {
    self.fields.len()
  }

  pub fn update_current_player(&mut self) {
    self.current_player = match self.current_player {
      Color::Yellow => Color::Blue,
      Color::Blue => Color::Red,
      Color::Red => Color::Green,
      Color::Green => Color::Yellow,
    }
  }

  // how many steps we need to make to reach the first field of player's home
  // e.g. curr_pos = 0, end_pos = 39 => distance = 40 (need to throw 40 to get to home)
  // max(end_pos + field_size) = 39, max(curr_pos) = 39
  pub fn distance_from_home(&self, current_position: usize) -> usize {
    // position of the field right in front of home
    let end_position = self.get_end_position();
    // +1 to get to the first home field
    (end_position + self.field_size() - current_position) % self.field_size() + 1
  }

  // how far away is the starting position (where we place pieces after throwing 6)
  //   from the ending position (= the last field before home)
  fn start_end_position_difference(&self) -> usize {
    // 1
    2
  }

  // we can use this 'modulo trick' to deal with different offsets and looping (pos 39 -> 0)
  // e.g. start_pos = 0 => end_pos = 39
  pub fn get_end_position(&self) -> usize {
    (self.get_starting_position() + self.fields.len() - self.start_end_position_difference())
      % self.fields.len()
  }

  // returns size of the home column (finish)
  pub fn get_home_size(&self) -> usize {
    match self.players.get(0) {
      Some(player) => player.home.len(),
      None => 4,
    }
  }

  pub fn is_occupied_by(&self, field: &Field, color: &Color) -> bool {
    match field {
      None => false,
      Some(_color) => _color == color,
    }
  }

  pub fn get_players_pieces_positions(&self, color: &Color) -> Vec<usize> {
    self
      .fields
      .iter()
      .enumerate()
      .filter(|&(_position, field)| self.is_occupied_by(field, color))
      .map(|(position, _field)| position)
      .collect()
  }

  // we assume home_offset is valid
  pub fn get_home_field(&self, home_offset: usize) -> &Field {
    let player = self.get_current_player();
    // let player = self.players.iter().filter(|&player| player.color == player_color).next().unwrap();
    &player.home[home_offset]
  }

  pub fn is_in_bounds(&self, position: usize) -> bool {
    position < self.fields.len()
  }

  // there is a clock-wise ordering: Yellow, Blue, Red, Green
  // TODO: move to a utility, pass attr 'color' to replace 'self.current_player'
  pub fn get_offset(&self) -> usize {
    let offset = (self.fields.len() / 4) as usize;
    match self.current_player {
      Color::Yellow => 0,
      Color::Blue => offset,
      Color::Red => offset * 2,
      Color::Green => offset * 3,
    }
  }

  // position of the field where we put pieces after throwing 6
  pub fn get_starting_position(&self) -> usize {
    self.get_offset() + 8
  }

  // if we land on opponent at 'position', we remove his piece (we can't jump on our own piece)
  pub fn clear_field(&mut self, position: usize) {
    if let Some(color) = self.fields[position] {
      self.remove_players_piece(color);
    }
  }

  pub fn remove_players_piece(&mut self, color: Color) {
    let player = self.get_player_mut(color);
    player.increase_pieces_at_start();
  }

  // add check for player.pawns_at_start > 0 ?
  /// check if position where promoted piece would land is not occupied by our piece
  pub fn can_promote_piece(&self, dice_value: usize) -> bool {
    dice_value >= 6 && self.is_available_field(self.get_starting_position() + dice_value - 6)
  }

  // TODO: should we remove opponent on starting position when promoting a piece?
  //  probably just skip as with other 'multi-hops' ? currently it's not being removed
  pub fn promote_piece(&mut self, dice_value: usize) {
    let mut position = self.get_starting_position();
    // self.clear_field(position);
    position += dice_value - 6;
    self.clear_field(position);
    let player = self.get_player_mut(self.current_player);
    player.decrease_pieces_at_start();
    self.fields[position] = Some(self.current_player)
  }

  // we can jump to a field, if it's either empty or occupied by opponent,
  // i.e. it's not occupied by us
  pub fn is_available_field(&self, position: usize) -> bool {
    self.is_in_bounds(position) && !self.is_current_players_piece(position)
  }

  pub fn opponent_at_field(&self, position: usize) -> bool {
    self.is_in_bounds(position)
      && !self.is_current_players_piece(position)
      && self.is_opponents_piece(position)
  }

  pub fn get_new_position(&self, position: usize, dice_value: usize) -> usize {
    (position + dice_value) % self.fields.len()
  }

  /// if we can make a move/jump within main board/field (not reaching home)
  pub fn can_jump(&self, position: usize, dice_value: usize) -> bool {
    dice_value < self.distance_from_home(position)
      && self.is_available_field(self.get_new_position(position, dice_value))
  }

  pub fn will_remove_enemy(&self, position: usize, dice_value: usize) -> bool {
    dice_value < self.distance_from_home(position)
      && self.is_opponents_piece(self.get_new_position(position, dice_value))
  }

  pub fn is_in_bounds_home(&self, home_offset: usize) -> bool {
    home_offset < self.get_home_size()
  }

  pub fn is_available_home_field(&self, home_offset: usize) -> bool {
    self.is_in_bounds_home(home_offset) && !self.is_home_field_occupied(home_offset)
  }

  /// check if piece can reach `safe zone`
  pub fn can_jump_to_home(&self, position: usize, dice_value: usize) -> bool {
    // let distance_from_home = self.distance_from_home(position);
    // let will_reach_home = dice_value >= distance_from_home;
    // let will_not_overjump_home = dice_value < distance_from_home + self.get_home_size();
    // match will_reach_home && will_not_overjump_home {
    //     true => {
    //         let home_offset = dice_value - distance_from_home;
    //         self.is_available_home_field(home_offset)
    //     },
    //     false => false
    // }

    match self.can_reach_home(position, dice_value)
      && !self.would_overjump_home(position, dice_value)
    {
      true => self.is_available_home_field(self.get_home_offset(position, dice_value)),
      false => false,
    }
  }

  pub fn jump(&mut self, old_position: usize, new_position: usize) {
    self.fields[old_position] = None;
    self.clear_field(new_position);
    self.fields[new_position] = Some(self.current_player)
  }

  // we assume we jump from 'main fields' to player's home
  // this currently doesn't allow moving pieces within home itself - we would just have to
  //    distinguish between old_position in self.fields and in home, so that we can clear
  //    the correct field
  pub fn jump_home(&mut self, old_position: usize, home_offset: usize) {
    self.fields[old_position] = None;
    let color = self.current_player;
    let home = self.get_home_mut();
    home[home_offset] = Some(color);
  }

  // if we move 'dice_value' fields, we will reach beyond the main board/field
  pub fn can_reach_home(&self, position: usize, dice_value: usize) -> bool {
    dice_value >= self.distance_from_home(position)
  }

  // if we move 'dice_value' fields, we will reach beyond the main board/field
  pub fn can_reach_finish(&self, position: usize, dice_value: usize) -> bool {
    dice_value == self.distance_from_home(position) + self.get_home_size()
  }

  // distance_from_home gets you already to the first home field, that's why '>=' and not only '>'
  pub fn would_overjump_home(&self, position: usize, dice_value: usize) -> bool {
    dice_value >= self.distance_from_home(position) + self.get_home_size()
  }

  // returns position/index of field in player's home column where we will jump,
  // i.e. offset in player's home column
  // e.g. if piece is right in front of home => distance = 1, and if we throw a 1,
  //      we would reach the first home field (home_offset = 0)
  pub fn get_home_offset(&self, position: usize, dice_value: usize) -> usize {
    dice_value - self.distance_from_home(position)
  }

  pub fn get_home(&self) -> &Vec<Field> {
    let player = self.get_current_player();
    &player.home
  }

  pub fn get_home_mut(&mut self) -> &mut Vec<Field> {
    let player = self.get_current_player_mut();
    &mut player.home
  }

  pub fn is_home_field_occupied(&self, home_offset: usize) -> bool {
    self.get_home_field(home_offset).is_some()
  }

  // jump from home column (1 of 5 home fields) to finish
  pub fn jump_from_home_to_finish(&mut self, home_offset: usize) {
    let mut player = self.get_current_player_mut();
    player.home[home_offset] = None;
    player.pawns_at_finish += 1;
  }

  // jump from main field to finish
  pub fn jump_to_finish(&mut self, position: usize) {
    self.fields[position] = None;
    let mut player = self.get_current_player_mut();
    player.pawns_at_finish += 1;
  }

  pub fn can_jump_from_home(&self, position: usize, dice_value: usize) -> bool {
    let home = self.get_home();
    position + dice_value == 5 || (position + dice_value < 5 && home[position + dice_value] == None)
  }

  pub fn jump_from_home(&mut self, old_home_offset: usize, new_home_offset: usize) {
    let color = self.current_player;
    let home = self.get_home_mut();
    home[old_home_offset] = None;
    home[new_home_offset] = Some(color)
  }

  // when we are trying to move piece in home column (1 out of 5 home fields)
  fn execute_move_from_home(&mut self, home_offset: usize, dice_value: usize) -> MoveResult {
    let distance_from_home = self.get_home_size() - home_offset;
    match dice_value == distance_from_home {
      true => {
        self.jump_from_home_to_finish(home_offset);
        MoveResult::Success(String::from("Move successful."))
      }
      false => match dice_value > distance_from_home {
        true => MoveResult::Error(String::from("Would overjump home.")),
        false => {
          let new_home_offset = home_offset + dice_value;
          match self.is_available_home_field(new_home_offset) {
            true => {
              self.jump_from_home(home_offset, new_home_offset);
              MoveResult::Success(String::from("Move successful."))
            }
            false => MoveResult::Error(String::from("Home field is occupied.")),
          }
        }
      },
    }
  }

  // as of now, we assume we can only move pieces from 'main fields', not home
  pub fn execute_move(
    &mut self,
    position: usize,
    dice_value: usize,
    home_column: bool,
  ) -> MoveResult {
    // dice_value = 0 means player threw 3x6, therefore he gets skipped (should we create a message ?)
    // MoveResult::SkipPlayer
    if dice_value == 0 {
      return MoveResult::Success(String::from("Throwing 3x6 means you have to wait a round."));
    }

    //TODO we need to get message if player wants to promote his piece (how?)
    if position == PROMOTE_PIECE {
      match self.can_promote_piece(dice_value) {
        false => {
          return MoveResult::Error(String::from(
            "We can't promote - starting field is occupied by our piece.",
          ))
        }
        true => {
          self.promote_piece(dice_value);
          return MoveResult::Success(String::from("Your piece has been promoted!"));
        }
      }
    }

    // we are trying to moving piece from
    if home_column {
      return self.execute_move_from_home(position, dice_value);
    }

    if self.can_reach_finish(position, dice_value) {
      self.jump_to_finish(position);
      return MoveResult::Success(String::from("Jumped to finish!"));
    }

    match self.can_reach_home(position, dice_value) {
      true => {
        // first we check a situation where we overjump home
        if self.would_overjump_home(position, dice_value) {
          return MoveResult::Error(String::from("Can't move - would overjump home."));
        }

        // offset/position in player's home column
        let home_offset = self.get_home_offset(position, dice_value);
        match self.is_available_home_field(home_offset) {
          false => MoveResult::Error(String::from("Can't move - home field is already occupied.")),
          true => {
            self.jump_home(position, home_offset);
            MoveResult::Success(String::from("Successfully moved a piece to home!"))
          }
        }
      }
      false => {
        let new_position = self.get_new_position(position, dice_value);
        match self.is_available_field(new_position) {
          false => MoveResult::Error(String::from("Can't move - field is occupied by our piece.")),
          true => {
            self.jump(position, new_position);
            MoveResult::Success(String::from("Moved to a new position."))
          }
        }
      }
    }
  }

  // returns whether a field specified by <position> is is occupied by a piece with <color>
  pub fn is_players_piece(&self, position: usize, player_color: &Color) -> bool {
    match self.fields.get(position) {
      Some(Some(color)) => color == player_color,
      _ => false,
    }
  }

  pub fn is_opponents_piece(&self, position: usize) -> bool {
    match self.fields.get(position) {
      Some(Some(color)) => color != &self.current_player,
      _ => false,
    }
  }

  pub fn is_current_players_piece(&self, position: usize) -> bool {
    self.is_players_piece(position, &self.current_player)
  }

  // returns whether a field is empty
  pub fn is_field_empty(&self, position: usize) -> bool {
    match self.fields.get(position) {
      Some(field) => field.is_none(),
      None => false,
    }
  }

  pub fn get_player(&self, player_color: Color) -> &Player {
    self
      .players
      .iter()
      .find(|&player| player.color == player_color)
      .unwrap()
  }

  pub fn get_player_mut(&mut self, player_color: Color) -> &mut Player {
    self
      .players
      .iter_mut()
      .find(|player| player.color == player_color)
      .unwrap()
  }

  pub fn get_current_player(&self) -> &Player {
    self.get_player(self.current_player)
  }

  pub fn get_current_player_mut(&mut self) -> &mut Player {
    self.get_player_mut(self.current_player)
  }

  pub fn is_player_ai(&self, player_color: Color) -> bool {
    self
      .players
      .iter()
      .any(|player| player.is_bot && player.color == player_color)
  }

  pub fn is_current_player_ai(&self) -> bool {
    self.is_player_ai(self.current_player)
  }
}

// ----------------[ tests ]-----------------

#[cfg(test)]
mod tests {
  use super::*;
  use crate::utils::player::get_available_positions;
  use std::borrow::Borrow;

  fn get_empty_game() -> Game {
    let mut game = Game::new();
    let mut players = Vec::new();
    for color in get_colors() {
      players.push(Player::new("".into(), "".into(), color, false));
    }
    game.players = players;
    game
  }

  fn get_colors() -> Vec<Color> {
    vec![Color::Yellow, Color::Blue, Color::Red, Color::Green]
  }

  fn get_all_players(game: &Game) -> Vec<&Player> {
    get_colors()
      .iter()
      .map(|color| game.get_player(*color))
      .collect::<Vec<&Player>>()
  }

  fn is_empty_fields(fields: &Vec<Field>) -> bool {
    fields.iter().all(|field| field.is_none())
  }

  fn empty_fields_count(fields: &Vec<Field>) -> usize {
    fields
      .iter()
      .filter(|&field| field.is_none())
      .collect::<Vec<&Field>>()
      .len()
  }

  fn is_empty_field(fields: &Vec<Field>, position: usize) -> bool {
    fields[position].is_none()
  }

  fn is_occupied_field_by(fields: &Vec<Field>, position: usize, color: Color) -> bool {
    match fields[position] {
      None => false,
      Some(_color) => _color == color,
    }
  }

  fn set_field(fields: &mut Vec<Field>, position: usize, field: Field) {
    fields[position] = field
  }

  fn print_game(game: &Game) {
    println!();
    for (i, field) in game.fields.iter().enumerate() {
      println!("{}: {:?}", i, field);
    }
    println!();
  }

  fn print_home(player: &Player) {
    println!();
    for (i, field) in player.home.iter().enumerate() {
      println!("{}: {:?}", i, field);
    }
    println!();
  }

  #[test]
  fn initial_promote() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    for player in get_all_players(&game) {
      assert_eq!(player.pawns_at_start, 4);
      assert_eq!(player.pawns_at_finish, 0);
      assert!(is_empty_fields(&player.home));
    }

    // the starting player is Yellow
    assert_eq!(game.current_player, Color::Yellow);
    assert!(is_empty_fields(&game.fields));
    assert_eq!(game.get_starting_position(), 8); // Yellow player starts at 8

    let dice_value = 9;
    let position = PROMOTE_PIECE;
    let home_column = false;

    match game.execute_move(position, dice_value, home_column) {
      MoveResult::Winner(_) => assert!(false),
      MoveResult::Error(_) => assert!(false),
      MoveResult::Success(_) => assert!(true),
    }

    assert!(is_empty_field(&game.fields, game.get_starting_position()));
    assert!(is_occupied_field_by(
      &game.fields,
      game.get_starting_position() + 3,
      Color::Yellow
    ));
    assert!(is_occupied_field_by(&game.fields, 11, Color::Yellow));
    assert!(!(is_empty_fields(&game.fields)));
    assert_eq!(game.get_current_player().pawns_at_start, 3);

    game.update_current_player();

    assert_eq!(game.current_player, Color::Blue)
  }

  #[test]
  fn blocked_promotion() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    assert_eq!(game.get_starting_position(), 8);
    set_field(&mut game.fields, 8 + 3, Some(Color::Yellow));
    assert_eq!(empty_fields_count(&game.fields), game.field_size() - 1);

    match game.execute_move(PROMOTE_PIECE, 6 + 3, false) {
      MoveResult::Error(_) => assert!(true),
      MoveResult::Winner(_) => assert!(false),
      MoveResult::Success(_) => assert!(false),
    }

    assert_eq!(game.get_current_player().pawns_at_start, 4);
    assert!(is_empty_field(&game.fields, 8));
    assert!(is_occupied_field_by(&game.fields, 8 + 3, Color::Yellow));
    assert_eq!(empty_fields_count(&game.fields), game.field_size() - 1);
  }

  #[test]
  fn promotion_remove_opponent() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    let dice_value = 6 + 8;
    let opponent_color = Color::Green;
    let starting_pos = game.get_starting_position();
    let field_size = game.fields.len();

    game.fields[starting_pos + dice_value - 6] = Some(opponent_color);

    // set_field(&mut game.fields, starting_pos + dice_value - 6, Some(opponent_color));
    let mut opponent = game.get_player_mut(opponent_color);
    opponent.pawns_at_start = 3;

    print_game(&game);

    let mut game = game.clone();
    match game.execute_move(PROMOTE_PIECE, dice_value, false) {
      MoveResult::Error(_) => assert!(false),
      MoveResult::Winner(_) => assert!(false),
      MoveResult::Success(_) => assert!(true),
    }

    print_game(&game);

    assert_eq!(game.get_current_player().pawns_at_start, 3);
    assert_eq!(game.get_player(opponent_color).pawns_at_start, 4);
    assert!(is_empty_field(&game.fields, 8));
    assert!(is_occupied_field_by(&game.fields, 8 + 8, Color::Yellow));
    assert_eq!(empty_fields_count(&game.fields), field_size - 1);
  }

  // #[test]
  // fn blocked_move() {
  //
  // }

  #[test]
  fn remove_opponent() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    let dice_value = 5;
    let opponent_color = Color::Green;
    let starting_pos = 20;
    let field_size = game.fields.len();

    game.fields[starting_pos] = Some(game.current_player);
    game.fields[starting_pos + dice_value] = Some(opponent_color);
    game.fields[starting_pos + dice_value + 1] = Some(opponent_color);
    game.fields[starting_pos + dice_value - 1] = Some(opponent_color);

    let mut opponent = game.get_player_mut(opponent_color);
    opponent.pawns_at_start = 1;

    let mut opponent = game.get_current_player_mut();
    opponent.pawns_at_start = 3;

    print_game(&game);

    let mut game = game.clone();
    match game.execute_move(starting_pos, dice_value, false) {
      MoveResult::Error(_) => assert!(false),
      MoveResult::Winner(_) => assert!(false),
      MoveResult::Success(_) => assert!(true),
    }

    print_game(&game);

    assert_eq!(game.get_current_player().pawns_at_start, 3);
    assert_eq!(game.get_player(opponent_color).pawns_at_start, 2);
    assert!(is_empty_field(&game.fields, starting_pos));
    assert!(is_occupied_field_by(
      &game.fields,
      starting_pos + dice_value,
      Color::Yellow
    ));
    assert!(is_occupied_field_by(
      &game.fields,
      starting_pos + dice_value + 1,
      Color::Green
    ));
    assert!(is_occupied_field_by(
      &game.fields,
      starting_pos + dice_value - 1,
      Color::Green
    ));

    // normally, Blue would follow
    game.current_player = Color::Green;

    let dice_value = 1;
    let starting_pos = 24;

    let mut game = game.clone();
    match game.execute_move(starting_pos, dice_value, false) {
      MoveResult::Error(_) => assert!(false),
      MoveResult::Winner(_) => assert!(false),
      MoveResult::Success(_) => assert!(true),
    }

    print_game(&game);

    assert_eq!(game.get_player(Color::Green).pawns_at_start, 2);
    // assert_eq!(game.get_player(Color::Yellow).pawns_at_start, 4);
    assert!(is_empty_field(&game.fields, starting_pos));
    assert!(is_occupied_field_by(
      &game.fields,
      starting_pos + dice_value,
      Color::Green
    ));
    assert!(is_occupied_field_by(
      &game.fields,
      starting_pos + dice_value + 1,
      Color::Green
    ));
  }

  #[test]
  fn move_board_to_home_success() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    let dice_value = 1;
    let starting_pos = 6; // right in front of home
    game.fields[starting_pos] = Some(game.current_player);

    let mut game = game.clone();
    match game.execute_move(starting_pos, dice_value, false) {
      MoveResult::Error(_) => assert!(false),
      MoveResult::Winner(_) => assert!(false),
      MoveResult::Success(_) => assert!(true),
    }

    let player = game.get_current_player();
    assert!(is_empty_field(&game.fields, starting_pos));
    assert!(is_occupied_field_by(&player.home, 0, Color::Yellow));
  }

  #[test]
  fn move_board_to_home_overjump() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    let dice_value = 9;
    let starting_pos = 6; // right in front of home
    game.fields[starting_pos] = Some(game.current_player);

    let mut game = game.clone();
    match game.execute_move(starting_pos, dice_value, false) {
      MoveResult::Error(_) => assert!(true),
      MoveResult::Winner(_) => assert!(false),
      MoveResult::Success(_) => assert!(false),
    }

    let player = game.get_current_player();
    assert!(is_occupied_field_by(
      &game.fields,
      starting_pos,
      Color::Yellow
    ));
    assert!(is_empty_fields(&player.home));
  }

  #[test]
  fn move_board_to_home_blocked() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    let dice_value = 1;
    let starting_pos = 6; // right in front of home
    game.fields[starting_pos] = Some(game.current_player);
    let mut player = game.get_current_player_mut();
    player.home[0] = Some(Color::Yellow);

    let mut game = game.clone();
    match game.execute_move(starting_pos, dice_value, false) {
      MoveResult::Error(_) => assert!(true),
      MoveResult::Winner(_) => assert!(false),
      MoveResult::Success(_) => assert!(false),
    }

    let player = game.get_current_player();
    assert!(is_occupied_field_by(
      &game.fields,
      starting_pos,
      Color::Yellow
    ));
    assert!(is_occupied_field_by(&player.home, 0, Color::Yellow));
  }

  #[test]
  fn move_home_to_home() {}

  #[test]
  fn move_home_to_home_blocked() {}

  #[test]
  fn move_board_to_finish() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    let dice_value = 6;
    let starting_pos = 6; // right in front of home
    game.fields[starting_pos] = Some(game.current_player);

    let mut game = game.clone();
    match game.execute_move(starting_pos, dice_value, false) {
      MoveResult::Error(_) => assert!(false),
      MoveResult::Winner(_) => assert!(false),
      MoveResult::Success(_) => assert!(true),
    }

    let player = game.get_current_player();
    assert!(is_empty_field(&game.fields, starting_pos));
    assert_eq!(player.pawns_at_finish, 1);
  }

  #[test]
  fn move_home_to_finish() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    let dice_value = 3;
    let starting_pos = 2;
    let player = game.get_current_player_mut();
    player.home[starting_pos] = Some(Color::Yellow);

    let mut game = game.clone();
    match game.execute_move(starting_pos, dice_value, true) {
      MoveResult::Error(_) => assert!(false),
      MoveResult::Winner(_) => assert!(false),
      MoveResult::Success(_) => assert!(true),
    }

    let player = game.get_current_player();
    assert!(is_empty_field(&player.home, starting_pos));
    assert_eq!(player.pawns_at_finish, 1);
  }

  #[test]
  fn move_to_finish_check_winner() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    let dice_value = 3;
    let starting_pos = 2;
    let player = game.get_current_player_mut();
    player.home[starting_pos] = Some(Color::Yellow);
    player.pawns_at_finish = 3;

    let mut game = game.clone();
    match game.execute_move(starting_pos, dice_value, true) {
      MoveResult::Error(_) => assert!(false),
      MoveResult::Winner(_) => assert!(true),
      MoveResult::Success(_) => assert!(true),
    }

    let winner = game.check_winner();
    assert!(winner.is_some());
    assert_eq!(winner.unwrap(), Color::Yellow);
  }

  #[test]
  fn invalid_moves() {}

  fn compare_vectors(a: &Vec<usize>, b: &Vec<usize>) {
    assert_eq!(a.len(), b.len());
    for n in a {
      assert!(b.contains(n));
    }
  }

  #[test]
  fn available_positions() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    // Yellow starts at position 8, the field in front of home is at position 6
    game.fields[9] = Some(Color::Yellow);
    game.fields[12] = Some(Color::Yellow);
    game.fields[6] = Some(Color::Yellow);

    let mut yellow_player = game.get_player_mut(Color::Yellow);
    yellow_player.home[2] = Some(Color::Yellow);
    yellow_player.pawns_at_start = 0;

    let dice_value = 1;
    let (actual_board_pos, actual_home_pos, actual_can_promote) =
      get_available_positions(&game, dice_value);
    let (expected_board_pos, expected_home_pos, expected_can_promote): (
      Vec<usize>,
      Vec<usize>,
      bool,
    ) = (vec![6, 9, 12], vec![2], false);
    compare_vectors(&actual_board_pos, &expected_board_pos);
    compare_vectors(&actual_home_pos, &expected_home_pos);
    assert_eq!(actual_can_promote, expected_can_promote);

    let dice_value = 3;
    let (actual_board_pos, actual_home_pos, actual_can_promote) =
      get_available_positions(&game, dice_value);
    let (expected_board_pos, expected_home_pos, expected_can_promote): (
      Vec<usize>,
      Vec<usize>,
      bool,
    ) = (vec![12], vec![2], false);
    compare_vectors(&actual_board_pos, &expected_board_pos);
    compare_vectors(&actual_home_pos, &expected_home_pos);
    assert_eq!(actual_can_promote, expected_can_promote);

    let dice_value = 4;
    let (actual_board_pos, actual_home_pos, actual_can_promote) =
      get_available_positions(&game, dice_value);
    let (expected_board_pos, expected_home_pos, expected_can_promote): (
      Vec<usize>,
      Vec<usize>,
      bool,
    ) = (vec![6, 9, 12], vec![], false);
    compare_vectors(&actual_board_pos, &expected_board_pos);
    compare_vectors(&actual_home_pos, &expected_home_pos);
    assert_eq!(actual_can_promote, expected_can_promote);

    let dice_value = 6;
    let (actual_board_pos, actual_home_pos, actual_can_promote) =
      get_available_positions(&game, dice_value);
    let (expected_board_pos, expected_home_pos, expected_can_promote): (
      Vec<usize>,
      Vec<usize>,
      bool,
    ) = (vec![6, 9, 12], vec![], false);
    compare_vectors(&actual_board_pos, &expected_board_pos);
    compare_vectors(&actual_home_pos, &expected_home_pos);
    assert_eq!(actual_can_promote, expected_can_promote);
  }

  #[test]
  fn available_positions_promote() {
    let mut game = get_empty_game();
    game.current_player = Color::Yellow;

    // Yellow starts at position 8
    game.fields[9] = Some(Color::Yellow);

    let mut yellow_player = game.get_player_mut(Color::Yellow);
    yellow_player.pawns_at_start = 3;

    let dice_value = 11;
    let (actual_board_pos, actual_home_pos, actual_can_promote) =
      get_available_positions(&game, dice_value);
    let (expected_board_pos, expected_home_pos, expected_can_promote): (
      Vec<usize>,
      Vec<usize>,
      bool,
    ) = (vec![9], vec![], true);
    compare_vectors(&actual_board_pos, &expected_board_pos);
    compare_vectors(&actual_home_pos, &expected_home_pos);
    assert_eq!(actual_can_promote, expected_can_promote);

    // blocked by our piece
    let dice_value = 7;
    let (actual_board_pos, actual_home_pos, actual_can_promote) =
      get_available_positions(&game, dice_value);
    let (expected_board_pos, expected_home_pos, expected_can_promote): (
      Vec<usize>,
      Vec<usize>,
      bool,
    ) = (vec![9], vec![], false);
    compare_vectors(&actual_board_pos, &expected_board_pos);
    compare_vectors(&actual_home_pos, &expected_home_pos);
    assert_eq!(actual_can_promote, expected_can_promote);

    // // if there is an opponent piece, we don't get blocked (promotion)
    game.fields[9] = Some(Color::Green);
    let dice_value = 7;
    let (actual_board_pos, actual_home_pos, actual_can_promote) =
      get_available_positions(&game, dice_value);
    let (expected_board_pos, expected_home_pos, expected_can_promote): (
      Vec<usize>,
      Vec<usize>,
      bool,
    ) = (vec![], vec![], true);
    compare_vectors(&actual_board_pos, &expected_board_pos);
    compare_vectors(&actual_home_pos, &expected_home_pos);
    assert_eq!(actual_can_promote, expected_can_promote);

    // if there is an opponent piece, we don't get blocked (on board)
    game.fields[9] = Some(Color::Yellow);
    game.fields[12] = Some(Color::Green);
    let dice_value = 3;
    let (actual_board_pos, actual_home_pos, actual_can_promote) =
      get_available_positions(&game, dice_value);
    let (expected_board_pos, expected_home_pos, expected_can_promote): (
      Vec<usize>,
      Vec<usize>,
      bool,
    ) = (vec![9], vec![], false);
    compare_vectors(&actual_board_pos, &expected_board_pos);
    compare_vectors(&actual_home_pos, &expected_home_pos);
    assert_eq!(actual_can_promote, expected_can_promote);
  }
}
