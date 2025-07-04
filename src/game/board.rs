use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Board {
    pub black: u64,
    pub white: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerColor {
    Black,
    White,
}

impl PlayerColor {
    pub fn opposite(&self) -> PlayerColor {
        match self {
            PlayerColor::Black => PlayerColor::White,
            PlayerColor::White => PlayerColor::Black,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub position: u8,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            black: 0x0000001008000000,
            white: 0x0000000810000000,
        }
    }

    pub fn get_piece(&self, position: u8) -> Option<PlayerColor> {
        let mask = 1u64 << position;
        if self.black & mask != 0 {
            Some(PlayerColor::Black)
        } else if self.white & mask != 0 {
            Some(PlayerColor::White)
        } else {
            None
        }
    }

    pub fn is_empty(&self, position: u8) -> bool {
        let mask = 1u64 << position;
        (self.black | self.white) & mask == 0
    }

    pub fn count_pieces(&self, color: PlayerColor) -> u32 {
        match color {
            PlayerColor::Black => self.black.count_ones(),
            PlayerColor::White => self.white.count_ones(),
        }
    }

    pub fn get_empty_squares(&self) -> u64 {
        !(self.black | self.white)
    }

    pub fn is_game_over(&self) -> bool {
        self.get_valid_moves(PlayerColor::Black) == 0 && self.get_valid_moves(PlayerColor::White) == 0
    }

    pub fn get_winner(&self) -> Option<PlayerColor> {
        if !self.is_game_over() {
            return None;
        }

        let black_count = self.count_pieces(PlayerColor::Black);
        let white_count = self.count_pieces(PlayerColor::White);

        if black_count > white_count {
            Some(PlayerColor::Black)
        } else if white_count > black_count {
            Some(PlayerColor::White)
        } else {
            None
        }
    }

    pub fn position_to_coords(position: u8) -> (usize, usize) {
        let row = (position / 8) as usize;
        let col = (position % 8) as usize;
        (row, col)
    }

    pub fn coords_to_position(row: usize, col: usize) -> u8 {
        (row * 8 + col) as u8
    }
}