// Minimax搜索算法 - 黑白棋AI的核心决策引擎
//
// 实现了经典的Minimax算法，配合Alpha-Beta剪枝优化
// 支持迭代加深搜索和时间控制，确保AI在限定时间内做出最佳决策
//
// 算法特点：
// - Alpha-Beta剪枝：大幅减少搜索节点数
// - 迭代加深：逐步增加搜索深度，支持时间控制
// - 并行搜索：桌面版支持多线程加速
// - 跨平台：Web版使用单线程，保持兼容性

use super::evaluation::evaluate_board;
use crate::game::{Board, Move, PlayerColor};
// 只在非WebAssembly平台导入并行计算库
#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;
// 时间相关功能：在支持的平台上使用，不支持的平台跳过
#[cfg(not(any(target_arch = "wasm32", target_family = "wasm")))]
use std::time::{Duration, Instant};

/// 搜索结果结构体
///
/// 包含搜索过程的完整信息，用于调试和性能分析
#[derive(Debug, Clone, Default)]
pub struct SearchResult {
    /// 找到的最佳走法
    pub best_move: Option<Move>,

    /// 该走法的评估分数
    #[allow(dead_code)]
    pub evaluation: i32,

    /// 实际达到的搜索深度
    #[allow(dead_code)]
    pub depth_reached: u8,

    /// 评估的节点总数
    #[allow(dead_code)]
    pub nodes_evaluated: u64,

    /// 搜索是否完整完成（未被时间限制中断）
    #[allow(dead_code)]
    pub completed: bool,
}

/// Minimax算法核心实现（带Alpha-Beta剪枝）
///
/// 这是一个递归搜索算法，通过最大化己方收益、最小化对手收益来找出最佳走法
/// Alpha-Beta剪枝可以大幅减少需要搜索的节点数量
///
/// # 参数
/// * `board` - 当前棋盘状态
/// * `depth` - 剩余搜索深度
/// * `alpha` - Alpha值（最大化玩家的最好选择下界）
/// * `beta` - Beta值（最小化玩家的最好选择上界）
/// * `maximizing` - 当前层是否为最大化层（AI回合）
/// * `player` - 要优化的目标玩家
///
/// # 返回
/// 当前局面的评估分数
pub fn minimax(
    board: &Board,
    depth: u8,
    alpha: i32,
    beta: i32,
    maximizing: bool,
    player: PlayerColor,
) -> i32 {
    // 递归终止条件：达到搜索深度或游戏结束
    if depth == 0 || board.is_game_over() {
        return evaluate_board(board, player);
    }

    // 确定当前层的玩家
    let current_player = if maximizing {
        player // 最大化层：AI玩家
    } else {
        player.opposite() // 最小化层：对手玩家
    };

    let moves = board.get_valid_moves_list(current_player);

    // 如果当前玩家无法走棋，跳过该层继续搜索
    if moves.is_empty() {
        return minimax(board, depth - 1, alpha, beta, !maximizing, player);
    }

    if maximizing {
        // 最大化层：寻找对AI最有利的走法
        let mut max_eval = i32::MIN;
        let mut alpha = alpha;

        for chess_move in moves {
            // 尝试每一个可能的走法
            let mut new_board = *board;
            new_board.make_move(chess_move.position, current_player);

            // 递归搜索下一层（切换到最小化层）
            let eval = minimax(&new_board, depth - 1, alpha, beta, false, player);

            // 更新最大值
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);

            // Alpha-Beta剪枝：如果beta <= alpha，后续分支不可能更好
            if beta <= alpha {
                break; // 剪枝
            }
        }
        max_eval
    } else {
        // 最小化层：寻找对AI最不利的走法（对手的最佳应对）
        let mut min_eval = i32::MAX;
        let mut beta = beta;

        for chess_move in moves {
            // 尝试每一个可能的走法
            let mut new_board = *board;
            new_board.make_move(chess_move.position, current_player);

            // 递归搜索下一层（切换到最大化层）
            let eval = minimax(&new_board, depth - 1, alpha, beta, true, player);

            // 更新最小值
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);

            // Alpha-Beta剪枝：如果beta <= alpha，后续分支不可能更好
            if beta <= alpha {
                break; // 剪枝
            }
        }
        min_eval
    }
}

/// 寻找最佳走法
///
/// 对当前玩家的所有可能走法进行评估，返回评分最高的走法
/// 支持桌面版并行计算和Web版单线程计算
///
/// # 参数
/// * `board` - 当前棋盘状态
/// * `depth` - 搜索深度
/// * `player` - 要寻找最佳走法的玩家
///
/// # 返回
/// 包含最佳走法和相关信息的SearchResult
pub fn find_best_move(board: &Board, depth: u8, player: PlayerColor) -> SearchResult {
    let moves = board.get_valid_moves_list(player);

    // 如果没有可用走法，返回默认结果
    if moves.is_empty() {
        return SearchResult::default();
    }

    // 评估所有可能的走法
    // 根据编译目标选择并行或串行处理
    let move_evaluations: Vec<(Move, i32)> = {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // 桌面版：使用Rayon并行计算，加速搜索
            moves
                .par_iter() // 并行迭代器
                .map(|&chess_move| {
                    let mut new_board = *board;
                    new_board.make_move(chess_move.position, player);
                    // 搜索对手的最佳应对（最小化层）
                    let evaluation =
                        minimax(&new_board, depth - 1, i32::MIN, i32::MAX, false, player);
                    (chess_move, evaluation)
                })
                .collect()
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Web版：使用单线程计算，保持兼容性
            moves
                .iter() // 普通迭代器
                .map(|&chess_move| {
                    let mut new_board = *board;
                    new_board.make_move(chess_move.position, player);
                    // 搜索对手的最佳应对（最小化层）
                    let evaluation =
                        minimax(&new_board, depth - 1, i32::MIN, i32::MAX, false, player);
                    (chess_move, evaluation)
                })
                .collect()
        }
    };

    // 选择评分最高的走法
    let (best_move, best_eval) = move_evaluations
        .into_iter()
        .max_by_key(|(_, eval)| *eval) // 按评估分数排序
        .unwrap();

    SearchResult {
        best_move: Some(best_move),
        evaluation: best_eval,
        depth_reached: depth,
        nodes_evaluated: 0, // TODO: 实际实现中应该统计节点数
        completed: true,
    }
}

/// 带时间限制的迭代加深搜索
///
/// 从深度1开始逐步增加搜索深度，直到时间用完或达到最大深度
/// 这种方法确保AI在限定时间内总能返回一个结果，并尽可能搜索得更深
///
/// # 参数
/// * `board` - 当前棋盘状态
/// * `time_limit` - 搜索时间限制（在不支持时间的平台上被忽略）
/// * `max_depth` - 最大搜索深度
/// * `player` - 要寻找最佳走法的玩家
///
/// # 返回
/// 在时间限制内找到的最佳搜索结果
///
/// # 算法优势
/// - 时间控制：保证在限定时间内返回结果（支持的平台）
/// - 渐进优化：更深的搜索通常产生更好的结果
/// - 提前终止：在时间不足时使用已有的较浅结果
/// - 跨平台兼容：在不支持时间的平台上回退到固定深度搜索
#[cfg(not(any(target_arch = "wasm32", target_family = "wasm")))]
pub fn find_best_move_with_time_limit(
    board: &Board,
    time_limit: Duration,
    max_depth: u8,
    player: PlayerColor,
) -> SearchResult {
    let start_time = Instant::now();
    let mut best_result = SearchResult::default();

    // 迭代加深：从深度1开始逐步增加搜索深度
    for depth in 1..=max_depth {
        let elapsed = start_time.elapsed();

        // 如果已经用了90%的时间，停止搜索以确保有足够时间返回结果
        if elapsed >= time_limit.mul_f32(0.9) {
            break;
        }

        // 在当前深度进行搜索
        let result = find_best_move(board, depth, player);

        // 检查搜索是否在时间限制内完成
        if start_time.elapsed() < time_limit {
            // 搜索完成，更新最佳结果
            best_result = result;
        } else {
            // 时间超限，使用之前深度的结果
            break;
        }
    }

    best_result
}

/// 带时间限制的迭代加深搜索（不支持时间的平台版本）
///
/// 在不支持时间功能的平台上，直接使用最大深度进行搜索
/// 这确保了跨平台兼容性，特别是在WebAssembly等环境中
#[cfg(any(target_arch = "wasm32", target_family = "wasm"))]
pub fn find_best_move_with_time_limit(
    board: &Board,
    _time_limit: core::time::Duration, // 参数保持兼容但不使用
    max_depth: u8,
    player: PlayerColor,
) -> SearchResult {
    // 在不支持时间的平台上，直接使用最大深度搜索
    // 这样既保证了API兼容性，又避免了时间相关的错误
    find_best_move(board, max_depth, player)
}
