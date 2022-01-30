use crate::types::Field;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use super::player::Player;
use crate::models::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub started_at: DateTime,
    pub finished_at: Option<DateTime>,
    pub fields: Vec<Field>,
    pub players: Vec<Player>,
    pub current_player: Color
}


impl Game {

    // there should be at most one winner at a time, therefore we take the first
    //   player that meets the winning condition
    pub fn check_winner(&self) -> Option<Color> {
        for player in &self.players {
            if player.check_winner() {
                return Some(player.color)
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
            Color::Green => Color::Yellow
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
        1
    }

    // we can use this 'modulo trick' to deal with different offsets and looping (pos 39 -> 0)
    // e.g. start_pos = 0 => end_pos = 39
    pub fn get_end_position(&self) -> usize {
        (self.get_starting_position() + self.fields.len() - self.start_end_position_difference()) % self.fields.len()
    }

    // returns size of the home column (finish)
    pub fn get_home_size(&self) -> usize {
        match self.players.get(0) {
            Some(player) => player.home.len(),
            None => 4
        }
    }

    pub fn is_occupied_by(&self, field: &Field, color: Color) -> bool {
        match field {
            None => false,
            Some(_color) => _color == color
        }
    }

    pub fn get_players_pieces_positions(&self, color: Color) -> Vec<usize> {
        self.fields
            .iter()
            .enumerate()
            .filter(|&(position, field)| self.is_occupied_by(field, color))
            .map(|&(position, field)| position)
            .collect()
    }

    // we assume home_offset is valid
    pub fn get_home_field(&self, player_color: Color, home_offset: usize) -> &Field {
        let player = self.players.iter().filter(|&player| player.color == player_color).next().unwrap();
        &player.home[home_offset]
    }

    pub fn is_in_bounds(&self, position: usize) -> bool {
        position >= 0 && position < self.fields.len()
    }

    pub fn get_field(&self, position: usize) -> &Field {
        // match self.fields.get(position) {
        //     None => None,
        //     Some(field) => field
        // }
        match self.is_in_bounds(position) {
            true => &self.fields[position],
            false => None
        }
    }

    // we can use the following (simplest) board as an example:
    // https://www.vectorstock.com/royalty-free-vector/ludo-board-game-vector-8703408
    // there is a clock-wise ordering: Yellow, Blue, Red, Green
    // there are 40 fields in the main 'board' (if we change the board - 56 fields e.g.),
    //   we have to adjust the constants
    pub fn get_offset(&self, color: Color) -> usize {
        let offset = (self.fields.len() / 4) as usize;
        match color {
            Color::Yellow => 0,
            Color::Blue => offset,
            Color::Red => offset * 2,
            Color::Green => offset * 3
        }
    }

    // position of the field where we put pieces after throwing 6
    pub fn get_starting_position(&self) -> usize {
        // self.get_offset() + 1
        self.get_offset(self.current_player)
    }

    // if we land on opponent at 'position', we remove his piece (we can't jump on our own piece)
    pub fn clear_field(&mut self, position: usize) {
        match self.get_field(position) {
            None => (),
            Some(color) => self.remove_players_piece(*color)
        }
    }

    pub fn remove_players_piece(&mut self, color: Color) {
        let mut player = self.get_player_mut(color);
        player.return_piece_to_start();
    }

    pub fn can_promote_piece(&self) -> bool {
        self.is_available_field(self.get_starting_position())
    }

    pub fn promote_piece(&mut self) {
        let position = self.get_starting_position();
        self.clear_field(position);
        let mut player = self.get_player_mut(self.current_player);
        player.promote_piece();
        self.fields[position] = Some(self.current_player)
    }


    // we can jump to a field, if it's either empty or occupied by opponent,
    // i.e. it's not occupied by us
    pub fn is_available_field(&self, position: usize) -> bool {
        self.is_in_bounds(position) && !self.is_current_players_piece(position)
    }

    pub fn get_new_position(&self, position: usize, dice_value: usize) -> usize {
        (position + dice_value) % self.fields.len()
    }

    // if we can make a move/jump within main board/field (not reaching home)
    pub fn can_jump(&self, position: usize, dice_value: usize) -> bool {
        dice_value < self.distance_from_home(position) &&
            self.is_available_field(self.get_new_position(position, dice_value))
    }

    pub fn is_in_bounds_home(&self, home_offset: usize) -> bool {
        home_offset >= 0 && home_offset < self.get_home_size()
    }

    pub fn is_available_home_field(&self, home_offset: usize) -> bool {
        self.is_in_bounds_home(home_offset) && !self.is_home_field_occupied(home_offset)
    }

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

        match self.can_reach_home(position, dice_value) && !self.would_overjump_home(position, dice_value) {
            true => self.is_available_home_field(self.get_home_offset(position, dice_value)),
            false => false
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
        let mut home = self.get_home_mut();
        home[home_offset] = Some(self.current_player)
    }

    // if we move 'dice_value' fields, we will reach beyond the main board/field
    pub fn can_reach_home(&self, position: usize, dice_value: usize) -> bool {
        dice_value >= self.distance_from_home(position)
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
        let mut player = self.get_current_player_mut();
        &mut player.home
    }

    pub fn is_home_field_occupied(&self, home_offset: usize) -> bool {
        match self.get_home_field(self.current_player, home_offset) {
            None => false,
            Some(_) => true
        }
    }


    // as of now, we assume we can only move pieces from 'main fields', not home
    pub fn execute_move(&mut self, position: usize, dice_value: usize) -> MoveResult {

        // dice_value = 0 means player threw 3x6, therefore he gets skipped (should we create a message ?)
        if dice_value == 0 {
            MoveResult::Success(String::from("Throwing 3x6 means you have to wait a round."))
        }

        // since dice_value includes bonus throws, dice_value = 6 can only occur when
        //   a player decides to promote his piece (without bonus throws)
        // might add a check for 'pawns_at_start > 0', if that is not handled in FE
        if dice_value == 6 {
            match self.can_promote_piece() {
                false => MoveResult::Error(String::from("We can't promote - starting field is occupied by our piece.")),
                true => {
                    self.promote_piece();
                    MoveResult::Success(String::from("Your piece has been promoted!"))
                }
            }
        }

        // if dice_value > 6 {
            // means player threw several time (with bonus throws) and wants to move his piece
            // do we need to check if any 'landing' positions are blocked by our pieces or do we simply
            //   check the end position?
            // e.g. we are at X=20, we throw 6 + 3 (bonus throw), end position = 29...
            //      do we also 'land' on position 20+6 ?
            //      if so, we have to check whether we should remove opponent's pieces / if we get blocked
            //        by our own pieces - IMO I would just ignore it, and simply check the end position,
            //        which is done correctly in the code below
        // }

        match self.can_reach_home(position, dice_value) {
            true => {
                // first we check a situation where we overjump home
                if self.would_overjump_home(position, dice_value) {
                    return MoveResult::Error(String::from("Can't move - would overjump home."))
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
            },
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
    pub fn is_players_piece(&self, position: usize, player_color: Color) -> bool {
        match self.fields.get(position) {
            Some(field) => match field {
                Some(color) => color == player_color,
                None => false
            }
            None => false
        }
    }

    pub fn is_current_players_piece(&self, position: usize) -> bool {
        self.is_players_piece(position, self.current_player)
    }

    // returns whether a field is empty
    pub fn is_field_empty(&self, position: usize) -> bool {
        match self.fields.get(position) {
            Some(field) => match field {
                Some(_) => false,
                None => true
            }
            None => false
        }
    }

    pub fn get_player(&self, player_color: Color) -> &Player {
        self.players.iter().filter(|&player| player.color == player_color).next().unwrap()
    }

    pub fn get_player_mut(&mut self, player_color: Color) -> &mut Player {
       self.players.iter_mut().filter(|&player| player.color == player_color).next().unwrap()
    }

    pub fn get_current_player(&self) -> &Player {
        self.get_player(self.current_player)
    }

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        self.get_player_mut(self.current_player)
    }

    pub fn is_player_AI(&self, player_color: Color) -> bool {
        let bots: Vec<Color> = self.players.iter().filter(|&player| player.is_bot).collect();
        bots.contains(&player_color)
    }

    pub fn is_current_player_AI(&self) -> bool {
        self.is_player_AI(self.current_player)
    }
}


pub enum MoveResult {
    Success(String),
    Error(String)
}
