// 棋盘评估系统 - 实现黑白棋局面评估的核心算法
//
// 通过综合考虑多个战略因素来评估棋盘局面的优劣：
// - 角位控制：占据角位获得稳定优势
// - 稳定性：不可翻转棋子的价值
// - 行动力：可用合法着法数量
// - 位置价值：基于棋盘位置的静态评估
// - 奇偶性：残局中的先手优势

use crate::game::{Board, PlayerColor};

/// 棋盘位置权重表
/// 
/// 定义了64个位置的静态评估值，反映了不同位置的战略价值：
/// - 角位(100): 最高价值，一旦占据就无法被翻转
/// - 边位(10): 相对稳定，不易被翻转
/// - 次角位(-20): 负值位置，容易让对手占据角位
/// - 内部位置: 根据与边角的距离分配不同权重
const POSITION_WEIGHTS: [i32; 64] = [
    // 第1行: 左上角(100) 到右上角(100)
    100, -20, 10, 5, 5, 10, -20, 100,
    // 第2行: 次角位为负值(-20, -50)
    -20, -50, -2, -2, -2, -2, -50, -20,
    // 第3-6行: 内部位置，渐进式权重
    10, -2, -1, -1, -1, -1, -2, 10,
    5, -2, -1, -1, -1, -1, -2, 5,
    5, -2, -1, -1, -1, -1, -2, 5,
    10, -2, -1, -1, -1, -1, -2, 10,
    // 第7行: 对称于第2行
    -20, -50, -2, -2, -2, -2, -50, -20,
    // 第8行: 对称于第1行，左下角(100) 到右下角(100)
    100, -20, 10, 5, 5, 10, -20, 100,
];

/// 评估权重配置
/// 
/// 根据游戏阶段动态调整各项评估因子的权重
/// 不同阶段的战略重点不同，需要相应调整评估标准
pub struct EvaluationWeights {
    /// 角位控制权重 - 角位的重要性
    pub corner: f32,
    /// 稳定性权重 - 不可翻转棋子的价值
    pub stability: f32,
    /// 行动力权重 - 可选择走法数量的重要性
    pub mobility: f32,
    /// 位置权重 - 基于位置表的静态评估
    pub positional: f32,
    /// 奇偶性权重 - 先手优势的重要性
    pub parity: f32,
}

impl EvaluationWeights {
    /// 根据游戏阶段返回相应的权重配置
    /// 
    /// 将游戏分为三个阶段，每个阶段有不同的战略重点：
    /// - 开局(0-20步): 重视行动力和位置控制
    /// - 中局(21-45步): 平衡各项因素
    /// - 残局(46-60步): 重视角位、稳定性和奇偶性
    pub fn for_stage(move_number: u32) -> Self {
        match move_number {
            // 开局阶段：重视行动力和位置控制
            // 此阶段棋子较少，要占据有利位置并保持选择性
            0..=20 => Self {
                corner: 0.8,      // 角位重要但不是最优先
                stability: 0.6,   // 稳定性次要
                mobility: 1.0,    // 行动力最重要，保持选择余地
                positional: 0.8,  // 位置控制重要
                parity: 0.2,      // 奇偶性不重要
            },
            // 中局阶段：各因素平衡发展
            // 棋子增多，开始争夺关键位置
            21..=45 => Self {
                corner: 1.0,      // 角位变得更重要
                stability: 0.8,   // 稳定性增加
                mobility: 0.6,    // 行动力权重下降
                positional: 0.6,  // 位置权重下降
                parity: 0.4,      // 奇偶性开始重要
            },
            // 残局阶段：重视稳定性和先手优势
            // 棋盘接近填满，稳定棋子和先手权最重要
            _ => Self {
                corner: 1.0,      // 角位依然重要
                stability: 1.0,   // 稳定性最重要
                mobility: 0.2,    // 行动力不重要了
                positional: 0.4,  // 位置权重较低
                parity: 0.8,      // 奇偶性很重要，决定最后几步的主动权
            },
        }
    }
}

/// 棋盘评估主函数
/// 
/// 综合所有评估因子，计算当前局面对指定玩家的价值
/// 返回正值表示对该玩家有利，负值表示不利
/// 
/// # 参数
/// * `board` - 当前棋盘状态
/// * `player` - 要评估的玩家颜色
/// 
/// # 返回
/// 局面评估分数，范围通常在-10000到+10000之间
pub fn evaluate_board(board: &Board, player: PlayerColor) -> i32 {
    // 计算当前步数，用于确定游戏阶段
    let move_count =
        board.count_pieces(PlayerColor::Black) + board.count_pieces(PlayerColor::White);
    
    // 获取当前阶段的权重配置
    let weights = EvaluationWeights::for_stage(move_count);

    // 计算各项评估分数
    let corner_score = evaluate_corners(board, player) as f32;
    let stability_score = evaluate_stability(board, player) as f32;
    let mobility_score = evaluate_mobility(board, player) as f32;
    let positional_score = evaluate_positional(board, player) as f32;
    let parity_score = evaluate_parity(board, player) as f32;

    // 加权求和得到最终评估分数
    (corner_score * weights.corner
        + stability_score * weights.stability
        + mobility_score * weights.mobility
        + positional_score * weights.positional
        + parity_score * weights.parity) as i32
}

/// 角位控制评估
/// 
/// 角位是黑白棋中最重要的位置，一旦占据就永远不会被翻转
/// 控制更多角位的玩家拥有显著优势
/// 
/// # 参数
/// * `board` - 当前棋盘状态
/// * `player` - 要评估的玩家颜色
/// 
/// # 返回
/// 角位控制分数，每占据一个角位+100分，失去一个角位-100分
pub fn evaluate_corners(board: &Board, player: PlayerColor) -> i32 {
    // 四个角位的位置索引：左上(0), 右上(7), 左下(56), 右下(63)
    let corners = [0, 7, 56, 63];
    let mut score = 0;

    for &corner in &corners {
        match board.get_piece(corner) {
            // 己方占据角位，获得100分奖励
            Some(color) if color == player => score += 100,
            // 对手占据角位，扣除100分
            Some(_) => score -= 100,
            // 角位空置，不影响分数
            None => {}
        }
    }

    score
}

/// 稳定性评估
/// 
/// 稳定棋子是指无法被对手翻转的棋子，包括角位和边位棋子
/// 稳定棋子越多，局面越有利
/// 
/// # 参数
/// * `board` - 当前棋盘状态
/// * `player` - 要评估的玩家颜色
/// 
/// # 返回
/// 稳定性分数，每个稳定棋子贡献50分
pub fn evaluate_stability(board: &Board, player: PlayerColor) -> i32 {
    // 获取该玩家的棋子位置位图
    let player_pieces = match player {
        PlayerColor::Black => board.black,
        PlayerColor::White => board.white,
    };

    let _opponent_pieces = match player {
        PlayerColor::Black => board.white,
        PlayerColor::White => board.black,
    };

    let mut stable_count = 0;

    // 遍历所有位置，统计稳定棋子数量
    for position in 0..64 {
        if player_pieces & (1u64 << position) != 0 && is_stable_piece(board, position) {
            stable_count += 1;
        }
    }

    // 每个稳定棋子价值50分
    stable_count * 50
}

/// 判断指定位置的棋子是否稳定
/// 
/// 简化实现：只考虑边位棋子为稳定棋子
/// 完整实现应该考虑连接到角位的稳定链
/// 
/// # 参数
/// * `_board` - 棋盘状态（当前简化实现未使用）
/// * `position` - 要检查的位置
/// 
/// # 返回
/// 如果该位置的棋子稳定则返回true
fn is_stable_piece(_board: &Board, position: u8) -> bool {
    let row = position / 8;
    let col = position % 8;

    // 简化判断：边位棋子视为稳定
    // TODO: 更精确的实现应该检查是否与角位形成稳定连接
    if row == 0 || row == 7 || col == 0 || col == 7 {
        return true;
    }

    false
}

/// 行动力评估
/// 
/// 行动力指可选择的合法走法数量，更多的选择意味着更大的灵活性
/// 同时限制对手的选择也是重要策略
/// 
/// # 参数
/// * `board` - 当前棋盘状态
/// * `player` - 要评估的玩家颜色
/// 
/// # 返回
/// 行动力分数，己方走法数与对手走法数的差值乘以30
pub fn evaluate_mobility(board: &Board, player: PlayerColor) -> i32 {
    // 计算己方可用走法数量
    let player_moves = board.get_valid_moves(player).count_ones() as i32;
    // 计算对手可用走法数量
    let opponent_moves = board.get_valid_moves(player.opposite()).count_ones() as i32;

    // 行动力优势 = (己方走法数 - 对手走法数) × 权重
    // 正值表示己方有更多选择，负值表示对手选择更多
    (player_moves - opponent_moves) * 30
}

/// 位置价值评估
/// 
/// 基于预定义的位置权重表评估棋子分布
/// 角位价值最高，次角位价值为负，其他位置根据重要性分配权重
/// 
/// # 参数
/// * `board` - 当前棋盘状态
/// * `player` - 要评估的玩家颜色
/// 
/// # 返回
/// 位置价值分数，基于POSITION_WEIGHTS表计算
pub fn evaluate_positional(board: &Board, player: PlayerColor) -> i32 {
    let mut score = 0;

    // 遍历棋盘上的每个位置
    for position in 0..64 {
        match board.get_piece(position) {
            // 己方棋子：加上该位置的权重值
            Some(color) if color == player => score += POSITION_WEIGHTS[position as usize],
            // 对手棋子：减去该位置的权重值
            Some(_) => score -= POSITION_WEIGHTS[position as usize],
            // 空位：不影响分数
            None => {}
        }
    }

    score
}

/// 奇偶性评估
/// 
/// 在黑白棋中，剩余空位数的奇偶性决定了谁将走最后一步
/// 走最后一步的玩家通常能获得先手优势，特别是在残局阶段
/// 
/// # 参数
/// * `board` - 当前棋盘状态
/// * `_player` - 要评估的玩家颜色（此函数中未使用，因为奇偶性是中性的）
/// 
/// # 返回
/// 奇偶性分数：奇数空位+10分，偶数空位-10分
/// 
/// # 注意
/// 这是一个简化实现，实际上应该考虑当前是哪个玩家的回合
pub fn evaluate_parity(board: &Board, _player: PlayerColor) -> i32 {
    // 计算棋盘上的空位数量
    let empty_squares = board.get_empty_squares().count_ones();

    // 奇偶性判断：
    // - 奇数空位: 意味着后续还有奇数步要走，当前玩家可能获得最后一步的优势
    // - 偶数空位: 意味着后续还有偶数步要走，对手可能获得最后一步的优势
    if empty_squares % 2 == 1 {
        10  // 奇数空位，对当前局面评估有小幅加分
    } else {
        -10 // 偶数空位，对当前局面评估有小幅减分
    }
}
