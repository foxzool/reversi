use super::minimax::find_best_move_with_time_limit;
use crate::game::{Board, Move, PlayerColor};
use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

#[derive(Component, Debug, Clone, Copy)]
pub enum AiDifficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub max_depth: u8,
    pub time_limit: Duration,
    pub mistake_probability: f32,
    #[allow(dead_code)]
    pub use_opening_book: bool,
}

impl AiDifficulty {
    pub fn get_search_params(&self) -> SearchParams {
        match self {
            Self::Beginner => SearchParams {
                max_depth: 2,
                time_limit: Duration::from_millis(100),
                mistake_probability: 0.3,
                use_opening_book: false,
            },
            Self::Intermediate => SearchParams {
                max_depth: 4,
                time_limit: Duration::from_millis(500),
                mistake_probability: 0.15,
                use_opening_book: false,
            },
            Self::Advanced => SearchParams {
                max_depth: 6,
                time_limit: Duration::from_secs(2),
                mistake_probability: 0.05,
                use_opening_book: true,
            },
            Self::Expert => SearchParams {
                max_depth: 12,
                time_limit: Duration::from_secs(5),
                mistake_probability: 0.0,
                use_opening_book: true,
            },
        }
    }

    pub fn get_ai_move(&self, board: &Board, player: PlayerColor) -> Option<Move> {
        let params = self.get_search_params();
        let result =
            find_best_move_with_time_limit(board, params.time_limit, params.max_depth, player);

        if params.mistake_probability > 0.0
            && rand::thread_rng().gen::<f32>() < params.mistake_probability
        {
            self.make_random_mistake(board, player)
        } else {
            result.best_move
        }
    }

    fn make_random_mistake(&self, board: &Board, player: PlayerColor) -> Option<Move> {
        let valid_moves = board.get_valid_moves_list(player);
        if valid_moves.is_empty() {
            return None;
        }

        let random_index = rand::thread_rng().gen_range(0..valid_moves.len());
        Some(valid_moves[random_index])
    }
}

#[derive(Component)]
pub struct AiPlayer {
    pub difficulty: AiDifficulty,
    pub color: PlayerColor,
    pub thinking_timer: Timer,
}

impl AiPlayer {
    pub fn new(difficulty: AiDifficulty, color: PlayerColor) -> Self {
        Self {
            difficulty,
            color,
            thinking_timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
        }
    }
}
