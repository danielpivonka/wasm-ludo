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

    pub fn update_current_player(&mut self) {
        self.current_player = match self.current_player {
            Color::Yellow => Color::Blue,
            Color::Blue => Color::Red,
            Color::Red => Color::Green,
            Color::Green => Color::Yellow
        }
    }

    // how many steps we need to make to reach the first field of player's home
    // for yellow starting at offset = 0, we need to make 40 - position steps
    // position = index in the Vec<Field> fields [0 ; 39]
    //   - if yellow piece is at position 39, it is right in front of its home
    pub fn distance_from_home(&self, position: usize, color: Color) -> usize {
        self.fields.len() - position + get_offset(color)
    }

    // returns size of the home column (finish)
    pub fn get_home_size(&self) -> usize {
        match self.players.get(0) {
            Some(player) => player.home.len(),
            None => 4
        }
    }

    // we assume home_offset is valid
    pub fn get_home_field(&self, player_color: Color, home_offset: usize) -> &Field {
        let player = self.players.iter().filter(|&player| player.color == player_color).next().unwrap();
        &player.home[home_offset]
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

    pub fn is_a_valid_move(&self, position: usize, dice_value: usize) -> MoveResultType {

        if dice_value == 6 {
            ...
        }

        // if player threw 6 and decided to move his piece from the start, there are two options [*]:
        //   a) if the field at offset is empty:
        //      1. place our piece at offset
        //      2. decrease pieces_at_start by one for a specific player (color)
        //   b) if the field is occupied by:
        //      1) our own piece - we can't move there, invalid move
        //      2) opponent's piece - we can move there and remove his piece
        //         - the same actions as for a) + increase pieces_at_start for the player whose piece
        //           we have just removed (sent to start)

        // [*] another thing is:
        //    is player able to send a message to move a piece from his start, even if he has
        //    no pieces at the start anymore (i.e. we might have add a check for the situation,
        //    when pieces_at_start = 0)


        // do we always obtain Game (and fields ...) from DB for every turn ?
        // and do we update the DB after every move as well (when the game/board changed) ?
        // adding bonus throws after getting 6 is not solved yet


        let distance_from_home = distance_from_home(position, self.current_player);

        match dice_value < distance_from_home {
            // is within main 'board'
            true => {
                new_position = position + dice_value;
                match self.is_field_empty(position) {
                    true => MoveResultType::Success,
                    false => match self.is_players_piece(position, self.current_player) {
                        true => MoveResultType::Error(String::from("Our piece already occupies this position")),
                        false => MoveResultType::Success
                    }
                }

                // if piece at position:
                //   a) is empty, we can go there:
                //          1. clear field at 'position'
                //          2. update field at 'new_position'
                //   b) is occupied by:
                //      1) our own piece - we can't go there, not a valid move
                //      2) opponent's piece - we can move there (clear field at 'position', update field
                //
            },

            // reaches home - validity of move is based on home
            false => {
                // first we check a situation where we overshoot home
                if dice_value >= distance_from_home + self.get_home_size() {
                    return MoveResultType::Error(String::from("Would overshoot home."))
                }

                // offset in player's home column (if piece is right in front of home - distance = 1,
                //    and we throw a 1, we would reach the first home field
                let home_offset = dice_value - distance_from_home;
                match self.get_home_field(self.current_player, home_offset) {
                    Some(_) => MoveResultType::Error(String::from("Home field is already occupied.")),
                    None => MoveResultType::Success
                }

                // if field at home[home_offset] is occupied, invalid move
                // otherwise we move our piece to that position:
                //    place our piece at home[home_offset] and remove original piece from fields[position]
            }
        }
    }


    // // returns whether a field specified by <position> is is occupied by a piece with <color>
    // pub fn is_players_piece(&self, position: usize, player_color: Color) -> bool {
    //     match self.fields.get(position) {
    //         Some(field) => match field {
    //             Some(color) => color == player_color,
    //             None => false
    //         }
    //         None => false
    //     }
    // }

    // returns whether a field specified by <position> is is occupied by a piece with <color>
    pub fn is_players_piece(&self, position: usize) -> bool {
        match self.fields.get(position) {
            Some(field) => match field {
                Some(color) => color == self.current_player,
                None => false
            }
            None => false
        }
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

    pub fn get_player(&self) -> &Player {
        self.players.iter().filter(|&player| player.color == self.current_player).next().unwrap()
    }

    pub fn is_current_play_AI(&self) -> bool {
        self.AI_players.contains
    }
}


pub enum MoveResultType {
    Success,
    Error(String)
}
