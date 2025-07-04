// AI模块 - 包含黑白棋游戏的人工智能实现
//
// 本模块实现了基于Minimax算法的AI对手，包括：
// - 多级难度设置
// - 棋盘评估函数
// - 搜索算法优化

/// AI难度级别定义模块
pub mod difficulty;

/// 棋盘评估函数模块
/// 实现了综合的位置评估策略
pub mod evaluation;

/// Minimax搜索算法模块
/// 包含Alpha-Beta剪枝和时间控制
pub mod minimax;

// 重新导出常用类型，方便外部模块使用
pub use difficulty::*;
