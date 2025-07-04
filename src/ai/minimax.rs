use super::evaluation::evaluate_board;
use crate::game::{Board, Move, PlayerColor};
use rayon::prelude::*;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Default)]
pub struct SearchResult {
    pub best_move: Option<Move>,
    pub evaluation: i32,
    pub depth_reached: u8,
    pub nodes_evaluated: u64,
    pub completed: bool,
}

pub fn minimax(
    board: &Board,
    depth: u8,
    alpha: i32,
    beta: i32,
    maximizing: bool,
    player: PlayerColor,
) -> i32 {
    if depth == 0 || board.is_game_over() {
        return evaluate_board(board, player);
    }

    let current_player = if maximizing {
        player
    } else {
        player.opposite()
    };
    let moves = board.get_valid_moves_list(current_player);

    if moves.is_empty() {
        return minimax(board, depth - 1, alpha, beta, !maximizing, player);
    }

    if maximizing {
        let mut max_eval = i32::MIN;
        let mut alpha = alpha;

        for chess_move in moves {
            let mut new_board = *board;
            new_board.make_move(chess_move.position, current_player);
            let eval = minimax(&new_board, depth - 1, alpha, beta, false, player);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;
        let mut beta = beta;

        for chess_move in moves {
            let mut new_board = *board;
            new_board.make_move(chess_move.position, current_player);
            let eval = minimax(&new_board, depth - 1, alpha, beta, true, player);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

pub fn find_best_move(board: &Board, depth: u8, player: PlayerColor) -> SearchResult {
    let moves = board.get_valid_moves_list(player);

    if moves.is_empty() {
        return SearchResult::default();
    }

    let move_evaluations: Vec<(Move, i32)> = moves
        .par_iter()
        .map(|&chess_move| {
            let mut new_board = *board;
            new_board.make_move(chess_move.position, player);
            let evaluation = minimax(&new_board, depth - 1, i32::MIN, i32::MAX, false, player);
            (chess_move, evaluation)
        })
        .collect();

    let (best_move, best_eval) = move_evaluations
        .into_iter()
        .max_by_key(|(_, eval)| *eval)
        .unwrap();

    SearchResult {
        best_move: Some(best_move),
        evaluation: best_eval,
        depth_reached: depth,
        nodes_evaluated: 0,
        completed: true,
    }
}

pub fn find_best_move_with_time_limit(
    board: &Board,
    time_limit: Duration,
    max_depth: u8,
    player: PlayerColor,
) -> SearchResult {
    let start_time = Instant::now();
    let mut best_result = SearchResult::default();

    for depth in 1..=max_depth {
        let elapsed = start_time.elapsed();
        if elapsed >= time_limit.mul_f32(0.9) {
            break;
        }

        let result = find_best_move(board, depth, player);

        if start_time.elapsed() < time_limit {
            best_result = result;
        } else {
            break;
        }
    }

    best_result
}
