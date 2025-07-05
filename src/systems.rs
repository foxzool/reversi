use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameSystems {
    // 游戏核心逻辑（输入、移动、AI、状态）
    Gameplay,
    // UI更新和渲染
    UI,
    // 通用系统（音频、语言、清理等）
    Common,
}
