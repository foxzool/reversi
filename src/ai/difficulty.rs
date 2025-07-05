// AI难度系统 - 实现多级别AI对手
//
// 通过调整搜索深度、时间限制和错误概率来模拟不同水平的AI对手
// 让玩家可以根据自己的水平选择合适的挑战难度

use super::minimax::find_best_move_with_time_limit;
use crate::game::{Board, Move, PlayerColor};
use bevy::prelude::*;
use rand::{random, Rng};
// 时间相关功能：根据平台支持情况选择合适的Duration类型
#[cfg(any(target_arch = "wasm32", target_family = "wasm"))]
use core::time::Duration;
#[cfg(not(any(target_arch = "wasm32", target_family = "wasm")))]
use std::time::Duration;

/// AI难度级别枚举
///
/// 定义了四个不同的AI难度级别，每个级别都有对应的搜索参数配置
#[derive(Component, Debug, Clone, Copy)]
pub enum AiDifficulty {
    /// 初级难度 - 适合新手玩家
    /// 搜索深度较浅，会偶尔犯错
    Beginner,

    /// 中级难度 - 适合有一定经验的玩家
    /// 搜索深度适中，偶尔会有失误
    Intermediate,

    /// 高级难度 - 适合熟练玩家
    /// 搜索深度较深，很少出错
    Advanced,

    /// 专家难度 - 最高难度
    /// 搜索深度最深，完美发挥
    Expert,
}

/// AI搜索参数配置
///
/// 定义了AI搜索算法的关键参数，用于控制AI的行为和性能
#[derive(Debug, Clone)]
pub struct SearchParams {
    /// 最大搜索深度 - 控制AI思考的层数
    /// 深度越大，AI越聪明但计算时间越长
    pub max_depth: u8,

    /// 搜索时间限制 - 防止AI思考时间过长
    /// 确保游戏流畅性
    pub time_limit: Duration,

    /// 失误概率 - 模拟AI犯错的可能性
    /// 0.0表示完美发挥，1.0表示完全随机
    pub mistake_probability: f32,

    /// 是否使用开局库 - 预设的开局走法
    /// 未来可能用于优化开局表现
    #[allow(dead_code)]
    pub use_opening_book: bool,
}

impl AiDifficulty {
    /// 获取对应难度级别的搜索参数
    ///
    /// 根据AI难度返回相应的搜索配置，包括搜索深度、时间限制和错误率
    pub fn get_search_params(&self) -> SearchParams {
        match self {
            // 初级：搜索2层，100ms时限，30%错误率
            Self::Beginner => SearchParams {
                max_depth: 2,
                time_limit: Duration::from_millis(100),
                mistake_probability: 0.3, // 30%概率犯错，模拟新手
                use_opening_book: false,
            },
            // 中级：搜索4层，500ms时限，15%错误率
            Self::Intermediate => SearchParams {
                max_depth: 4,
                time_limit: Duration::from_millis(500),
                mistake_probability: 0.15, // 15%概率犯错，偶尔失误
                use_opening_book: false,
            },
            // 高级：搜索6层，2秒时限，5%错误率
            Self::Advanced => SearchParams {
                max_depth: 6,
                time_limit: Duration::from_secs(2),
                mistake_probability: 0.05, // 5%概率犯错，很少出错
                use_opening_book: true,
            },
            // 专家：搜索12层，5秒时限，0%错误率
            Self::Expert => SearchParams {
                max_depth: 12,
                time_limit: Duration::from_secs(5),
                mistake_probability: 0.0, // 完美发挥，不犯错
                use_opening_book: true,
            },
        }
    }

    /// 获取AI的下一步棋
    ///
    /// 根据当前棋盘状态和AI难度，计算出最佳走法
    /// 可能会根据错误概率故意选择非最优解，模拟真实对手
    pub fn get_ai_move(&self, board: &Board, player: PlayerColor) -> Option<Move> {
        let params = self.get_search_params();

        // 使用Minimax算法搜索最佳走法
        let result =
            find_best_move_with_time_limit(board, params.time_limit, params.max_depth, player);

        // 根据失误概率决定是否故意犯错
        if params.mistake_probability > 0.0 && random::<f32>() < params.mistake_probability {
            // 故意选择随机走法，模拟人类失误
            self.make_random_mistake(board, player)
        } else {
            // 返回最佳走法
            result.best_move
        }
    }

    /// 模拟AI犯错 - 随机选择一个合法走法
    ///
    /// 当AI需要故意犯错时调用，从所有合法走法中随机选择一个
    /// 这样可以让低难度AI更像真实的初学者
    fn make_random_mistake(&self, board: &Board, player: PlayerColor) -> Option<Move> {
        let valid_moves = board.get_valid_moves_list(player);
        if valid_moves.is_empty() {
            return None;
        }

        // 随机选择一个合法走法
        let random_index = rand::thread_rng().gen_range(0..valid_moves.len());
        Some(valid_moves[random_index])
    }
}

/// AI玩家组件
///
/// 在Bevy ECS系统中表示AI玩家实体的组件
/// 包含AI的配置信息和状态
#[derive(Component)]
pub struct AiPlayer {
    /// AI难度级别
    pub difficulty: AiDifficulty,

    /// AI控制的棋子颜色
    pub color: PlayerColor,

    /// AI思考计时器 - 用于模拟思考时间
    /// 避免AI瞬间出招，提供更好的游戏体验
    pub thinking_timer: Timer,
}

impl AiPlayer {
    /// 创建新的AI玩家
    ///
    /// # 参数
    /// * `difficulty` - AI难度级别
    /// * `color` - AI控制的棋子颜色
    ///
    /// # 返回
    /// 配置好的AI玩家实例，包含1秒的基础思考时间
    pub fn new(difficulty: AiDifficulty, color: PlayerColor) -> Self {
        Self {
            difficulty,
            color,
            // 设置1秒的基础思考时间，让AI看起来在思考
            thinking_timer: Timer::new(Duration::from_millis(1000), TimerMode::Once),
        }
    }
}
