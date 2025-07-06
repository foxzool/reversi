/// 本地化文本结构
#[derive(Debug)]
#[allow(dead_code)]
pub struct LocalizedTexts {
    // 语言选择界面
    pub language_selection_title: &'static str,
    pub language_english: &'static str,
    pub language_chinese: &'static str,

    // UI 文本
    pub score_format: &'static str,
    pub ai_difficulty_format: &'static str,
    pub game_in_progress: &'static str,
    pub click_to_restart: &'static str,
    pub your_turn: &'static str,
    pub ai_turn: &'static str,

    // 难度级别
    pub difficulty_easy: &'static str,
    pub difficulty_medium: &'static str,
    pub difficulty_hard: &'static str,
    pub difficulty_expert: &'static str,

    // 游戏状态
    pub black_wins: &'static str,
    pub white_wins: &'static str,
    pub draw: &'static str,
    pub pass_turn: &'static str,

    // 规则文本
    pub rules_title: &'static str,
    pub rules_close: &'static str,
    pub rules_content: &'static str,

    // 调试信息
    pub ai_difficulty_changed: &'static str,
    pub game_over_detected: &'static str,
    pub restarting_game: &'static str,
    pub executing_game_restart: &'static str,
    
    // 新增界面文本
    pub loading_text: &'static str,
    pub select_difficulty: &'static str,
    pub back_to_difficulty: &'static str,
}

/// 英文文本
pub const ENGLISH_TEXTS: LocalizedTexts = LocalizedTexts {
    // 语言选择界面
    language_selection_title: "Select Language / 选择语言",
    language_english: "English",
    language_chinese: "中文",

    // UI 文本
    score_format: "B:{} W:{}",
    ai_difficulty_format: "AI: {}",
    game_in_progress: "Game in progress",
    click_to_restart: "Click to restart",
    your_turn: "Your turn.",
    ai_turn: "Bill's turn.",

    // 难度级别
    difficulty_easy: "Easy",
    difficulty_medium: "Medium",
    difficulty_hard: "Hard",
    difficulty_expert: "Expert",

    // 游戏状态
    black_wins: "Black wins!",
    white_wins: "White wins!",
    draw: "Draw!",
    pass_turn: "has no valid moves. Pass turn.",

    // 规则文本
    rules_title: "Reversi Rules",
    rules_close: "Close",
    rules_content: "OBJECTIVE:\nCapture the most pieces by the end of the game.\n\nHOW TO PLAY:\n• Players alternate placing pieces\n• Black always goes first\n• Place pieces to trap opponent's pieces\n• Trapped pieces flip to your color\n• Must make a valid move if possible\n• Game ends when board is full or no moves available\n\nVALID MOVES:\n• Must trap at least one opponent piece\n• Pieces are trapped in straight lines (horizontal, vertical, diagonal)\n• All trapped pieces between your new piece and existing piece flip\n\nCONTROLS:\n• Click/tap to place pieces\n• M: Toggle sound",

    // 调试信息
    ai_difficulty_changed: "AI difficulty changed to:",
    game_over_detected: "Game over detected!",
    restarting_game: "Restarting game",
    executing_game_restart: "Executing game restart",
    
    // 新增界面文本
    loading_text: "Loading...",
    select_difficulty: "Select Difficulty",
    back_to_difficulty: "← Back",
};

/// 中文文本
pub const CHINESE_TEXTS: LocalizedTexts = LocalizedTexts {
    // 语言选择界面
    language_selection_title: "Select Language / 选择语言",
    language_english: "English",
    language_chinese: "中文",

    // UI 文本
    score_format: "黑:{} 白:{}",
    ai_difficulty_format: "AI: {}",
    game_in_progress: "游戏进行中",
    click_to_restart: "点击重新开始",
    your_turn: "轮到你了。",
    ai_turn: "AI回合。",

    // 难度级别
    difficulty_easy: "简单",
    difficulty_medium: "中等",
    difficulty_hard: "困难",
    difficulty_expert: "专家",

    // 游戏状态
    black_wins: "黑棋获胜！",
    white_wins: "白棋获胜！",
    draw: "平局！",
    pass_turn: "无可用走法，跳过回合。",

    // 规则文本
    rules_title: "黑白棋规则",
    rules_close: "关闭",
    rules_content: "游戏目标：\n在游戏结束时获得最多的棋子。\n\n游戏玩法：\n• 玩家轮流放置棋子\n• 黑棋先手\n• 放置棋子以夹住对手棋子\n• 被夹住的棋子翻转为己方颜色\n• 有合法走法时必须走棋\n• 棋盘填满或无合法走法时游戏结束\n\n合法走法：\n• 必须至少夹住一个对手棋子\n• 棋子在直线上被夹住（水平、垂直、对角线）\n• 新棋子与已有棋子之间的所有对手棋子都会翻转\n\n操作控制：\n• 点击/触摸放置棋子\n• M：切换音效",

    // 调试信息
    ai_difficulty_changed: "AI难度已改为：",
    game_over_detected: "检测到游戏结束！",
    restarting_game: "重新开始游戏",
    executing_game_restart: "执行游戏重新开始",
    
    // 新增界面文本
    loading_text: "加载中...",
    select_difficulty: "选择难度",
    back_to_difficulty: "← 返回",
};
