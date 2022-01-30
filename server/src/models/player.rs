use serde::{Deserialize, Serialize};

use crate::types::Field;

use super::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub player_id: String,
    pub color: Color,
    pub pawns_at_start: u32,
    pub home: Vec<Field>,
    pub is_bot: bool
}

impl Player {

    // number of player's figures
    pub fn pieces_count(&self) -> usize {
        4
    }

    // returns whether all player's pieces are in home (occupy fields of home)
    // we assume there are 4 pieces for each player
    pub fn check_winner(&self) -> bool {
        let occupied_fields: Vec<Field> = self.home.iter().filter(|&field| field.is_some()).collect();
        occupied_fields.len() >= self.pieces_count()
    }

    pub fn return_piece_to_start(&mut self) {
        self.pawns_at_start += 1
    }

    pub fn promote_piece(&mut self) {
        self.pawns_at_start -= 1
    }
}
