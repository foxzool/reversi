use crate::game::{Board, PlayerColor};

const POSITION_WEIGHTS: [i32; 64] = [
    100, -20, 10, 5, 5, 10, -20, 100, -20, -50, -2, -2, -2, -2, -50, -20, 10, -2, -1, -1, -1, -1,
    -2, 10, 5, -2, -1, -1, -1, -1, -2, 5, 5, -2, -1, -1, -1, -1, -2, 5, 10, -2, -1, -1, -1, -1, -2,
    10, -20, -50, -2, -2, -2, -2, -50, -20, 100, -20, 10, 5, 5, 10, -20, 100,
];

pub struct EvaluationWeights {
    pub corner: f32,
    pub stability: f32,
    pub mobility: f32,
    pub positional: f32,
    pub parity: f32,
}

impl EvaluationWeights {
    pub fn for_stage(move_number: u32) -> Self {
        match move_number {
            0..=20 => Self {
                corner: 0.8,
                stability: 0.6,
                mobility: 1.0,
                positional: 0.8,
                parity: 0.2,
            },
            21..=45 => Self {
                corner: 1.0,
                stability: 0.8,
                mobility: 0.6,
                positional: 0.6,
                parity: 0.4,
            },
            _ => Self {
                corner: 1.0,
                stability: 1.0,
                mobility: 0.2,
                positional: 0.4,
                parity: 0.8,
            },
        }
    }
}

pub fn evaluate_board(board: &Board, player: PlayerColor) -> i32 {
    let move_count =
        board.count_pieces(PlayerColor::Black) + board.count_pieces(PlayerColor::White);
    let weights = EvaluationWeights::for_stage(move_count);

    let corner_score = evaluate_corners(board, player) as f32;
    let stability_score = evaluate_stability(board, player) as f32;
    let mobility_score = evaluate_mobility(board, player) as f32;
    let positional_score = evaluate_positional(board, player) as f32;
    let parity_score = evaluate_parity(board, player) as f32;

    (corner_score * weights.corner
        + stability_score * weights.stability
        + mobility_score * weights.mobility
        + positional_score * weights.positional
        + parity_score * weights.parity) as i32
}

pub fn evaluate_corners(board: &Board, player: PlayerColor) -> i32 {
    let corners = [0, 7, 56, 63];
    let mut score = 0;

    for &corner in &corners {
        match board.get_piece(corner) {
            Some(color) if color == player => score += 100,
            Some(_) => score -= 100,
            None => {}
        }
    }

    score
}

pub fn evaluate_stability(board: &Board, player: PlayerColor) -> i32 {
    let player_pieces = match player {
        PlayerColor::Black => board.black,
        PlayerColor::White => board.white,
    };

    let _opponent_pieces = match player {
        PlayerColor::Black => board.white,
        PlayerColor::White => board.black,
    };

    let mut stable_count = 0;

    for position in 0..64 {
        if player_pieces & (1u64 << position) != 0 && is_stable_piece(board, position) {
            stable_count += 1;
        }
    }

    stable_count * 50
}

fn is_stable_piece(_board: &Board, position: u8) -> bool {
    let row = position / 8;
    let col = position % 8;

    if row == 0 || row == 7 || col == 0 || col == 7 {
        return true;
    }

    false
}

pub fn evaluate_mobility(board: &Board, player: PlayerColor) -> i32 {
    let player_moves = board.get_valid_moves(player).count_ones() as i32;
    let opponent_moves = board.get_valid_moves(player.opposite()).count_ones() as i32;

    (player_moves - opponent_moves) * 30
}

pub fn evaluate_positional(board: &Board, player: PlayerColor) -> i32 {
    let mut score = 0;

    for position in 0..64 {
        match board.get_piece(position) {
            Some(color) if color == player => score += POSITION_WEIGHTS[position as usize],
            Some(_) => score -= POSITION_WEIGHTS[position as usize],
            None => {}
        }
    }

    score
}

pub fn evaluate_parity(board: &Board, _player: PlayerColor) -> i32 {
    let empty_squares = board.get_empty_squares().count_ones();

    if empty_squares % 2 == 1 {
        10
    } else {
        -10
    }
}
