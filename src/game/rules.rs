use super::{Board, Move, PlayerColor};

const DIRECTIONS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Board {
    pub fn get_valid_moves(&self, player: PlayerColor) -> u64 {
        let (own, opp) = match player {
            PlayerColor::Black => (self.black, self.white),
            PlayerColor::White => (self.white, self.black),
        };

        let mut moves = 0u64;
        let empty = self.get_empty_squares();

        for &(dx, dy) in &DIRECTIONS {
            moves |= self.get_moves_in_direction(own, opp, empty, dx, dy);
        }

        moves
    }

    pub fn get_valid_moves_list(&self, player: PlayerColor) -> Vec<Move> {
        let moves_mask = self.get_valid_moves(player);
        let mut moves = Vec::new();

        for position in 0..64 {
            if moves_mask & (1u64 << position) != 0 {
                moves.push(Move { position });
            }
        }

        moves
    }

    pub fn is_valid_move(&self, position: u8, player: PlayerColor) -> bool {
        if position >= 64 || !self.is_empty(position) {
            return false;
        }

        let moves_mask = self.get_valid_moves(player);
        moves_mask & (1u64 << position) != 0
    }

    pub fn make_move(&mut self, position: u8, player: PlayerColor) -> bool {
        if !self.is_valid_move(position, player) {
            return false;
        }

        let mask = 1u64 << position;
        let flipped = self.get_flipped_discs(position, player);

        match player {
            PlayerColor::Black => {
                self.black |= mask | flipped;
                self.white &= !flipped;
            }
            PlayerColor::White => {
                self.white |= mask | flipped;
                self.black &= !flipped;
            }
        }

        true
    }

    fn get_moves_in_direction(&self, own: u64, opp: u64, empty: u64, dx: i8, dy: i8) -> u64 {
        let mut moves = 0u64;

        for pos in 0..64 {
            let row = pos / 8;
            let col = pos % 8;

            if empty & (1u64 << pos) == 0 {
                continue;
            }

            let mut r = row as i8 + dx;
            let mut c = col as i8 + dy;
            let mut found_opponent = false;

            while (0..8).contains(&r) && (0..8).contains(&c) {
                let check_pos = (r * 8 + c) as u8;
                let check_mask = 1u64 << check_pos;

                if opp & check_mask != 0 {
                    found_opponent = true;
                } else if own & check_mask != 0 && found_opponent {
                    moves |= 1u64 << pos;
                    break;
                } else {
                    break;
                }

                r += dx;
                c += dy;
            }
        }

        moves
    }

    fn get_flipped_discs(&self, position: u8, player: PlayerColor) -> u64 {
        let (own, opp) = match player {
            PlayerColor::Black => (self.black, self.white),
            PlayerColor::White => (self.white, self.black),
        };

        let mut flipped = 0u64;
        let row = (position / 8) as i8;
        let col = (position % 8) as i8;

        for &(dx, dy) in &DIRECTIONS {
            let mut r = row + dx;
            let mut c = col + dy;
            let mut candidate_flips = 0u64;

            while (0..8).contains(&r) && (0..8).contains(&c) {
                let check_pos = (r * 8 + c) as u8;
                let check_mask = 1u64 << check_pos;

                if opp & check_mask != 0 {
                    candidate_flips |= check_mask;
                } else if own & check_mask != 0 {
                    flipped |= candidate_flips;
                    break;
                } else {
                    break;
                }

                r += dx;
                c += dy;
            }
        }

        flipped
    }

    pub fn has_valid_moves(&self, player: PlayerColor) -> bool {
        self.get_valid_moves(player) != 0
    }
}
