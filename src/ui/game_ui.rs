use super::CurrentPlayer;
use crate::{
    ai::{AiDifficulty, AiPlayer},
    game::{Board, PlayerColor},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct CurrentPlayerText;

#[derive(Component)]
pub struct GameStatusText;

#[derive(Component)]
pub struct PlayerAvatar {
    #[allow(dead_code)]
    pub player_color: PlayerColor,
}

#[derive(Component)]
pub struct PlayerNameText {
    #[allow(dead_code)]
    pub player_color: PlayerColor,
}

#[derive(Component)]
pub struct TurnIndicator;

#[derive(Component)]
pub struct DifficultyText;

pub fn setup_game_ui(mut commands: Commands) {
    // 创建根UI容器
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },))
        .with_children(|parent| {
            // 顶部区域 - Bill
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|top_parent| {
                    // Bill头像
                    top_parent.spawn((
                        Node {
                            width: Val::Px(50.0),
                            height: Val::Px(50.0),
                            border: UiRect::all(Val::Px(2.0)),
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(25.0)),
                        BackgroundColor(Color::srgb(0.7, 0.7, 0.7)),
                        BorderColor(Color::WHITE),
                        PlayerAvatar {
                            player_color: PlayerColor::White,
                        },
                    ));

                    // Bill名称
                    top_parent.spawn((
                        Text::new("Bill"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        PlayerNameText {
                            player_color: PlayerColor::White,
                        },
                    ));
                });

            // 中间区域保留给棋盘
            parent.spawn((Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                ..default()
            },));

            // 底部区域 - You
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|bottom_parent| {
                    // Your turn文本
                    bottom_parent.spawn((
                        Text::new("Your turn."),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        },
                        TurnIndicator,
                    ));

                    // You头像
                    bottom_parent.spawn((
                        Node {
                            width: Val::Px(50.0),
                            height: Val::Px(50.0),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BorderRadius::all(Val::Px(25.0)),
                        BackgroundColor(Color::srgb(0.9, 0.7, 0.5)),
                        BorderColor(Color::WHITE),
                        PlayerAvatar {
                            player_color: PlayerColor::Black,
                        },
                    ));
                });
        });

    // 移动端适配的信息面板 - 放在顶部中央，更紧凑
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Px(5.0),
            // 中心对齐
            translate: (-50.0, 0.0).into(),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(15.0),
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(8.0)),
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            border_radius: BorderRadius::all(Val::Px(8.0)),
            ..default()
        },))
        .with_children(|parent| {
            // 分数显示 - 更紧凑
            parent.spawn((
                Text::new("●2 ○2"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                ScoreText,
            ));

            // AI难度显示 - 移动端简化显示
            parent.spawn((
                Text::new("AI: 中级"),
                TextFont {
                    font_size: 12.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                DifficultyText,
            ));
        });

    // 状态信息 - 移动端放在底部中央
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            bottom: Val::Px(5.0),
            // 中心对齐
            translate: (-50.0, 0.0).into(),
            padding: UiRect::all(Val::Px(6.0)),
            background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        Text::new("Game in progress"),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::WHITE),
        GameStatusText,
    ));
}

pub fn update_score_text(
    mut score_query: Query<&mut Text, With<ScoreText>>,
    board_query: Query<&Board>,
) {
    if let (Ok(mut text), Ok(board)) = (score_query.single_mut(), board_query.single()) {
        let black_count = board.count_pieces(PlayerColor::Black);
        let white_count = board.count_pieces(PlayerColor::White);
        **text = format!("●{black_count} ○{white_count}");
    }
}

pub fn update_current_player_text(
    mut player_query: Query<&mut Text, With<CurrentPlayerText>>,
    current_player: Res<CurrentPlayer>,
) {
    if current_player.is_changed() {
        if let Ok(mut text) = player_query.single_mut() {
            **text = format!("Current Player: {:?}", current_player.0);
        }
    }
}

pub fn update_game_status_text(
    mut status_query: Query<&mut Text, With<GameStatusText>>,
    board_query: Query<&Board>,
    current_player: Res<CurrentPlayer>,
) {
    if let (Ok(mut text), Ok(board)) = (status_query.single_mut(), board_query.single()) {
        if board.is_game_over() {
            if let Some(winner) = board.get_winner() {
                **text = format!("{winner:?} wins! Tap to restart");
            } else {
                **text = "Draw! Tap to restart".to_string();
            }
        } else if !board.has_valid_moves(current_player.0) {
            **text = format!("{:?} has no valid moves. Pass turn.", current_player.0);
        } else {
            **text = "Game in progress".to_string();
        }
    }
}

pub fn update_turn_indicator(
    mut turn_query: Query<&mut Text, With<TurnIndicator>>,
    current_player: Res<CurrentPlayer>,
) {
    if current_player.is_changed() {
        if let Ok(mut text) = turn_query.single_mut() {
            match current_player.0 {
                PlayerColor::Black => **text = "Your turn.".to_string(),
                PlayerColor::White => **text = "Bill's turn.".to_string(),
            }
        }
    }
}

pub fn update_difficulty_text(
    mut difficulty_query: Query<&mut Text, With<DifficultyText>>,
    ai_query: Query<&AiPlayer, Changed<AiPlayer>>,
) {
    if let Ok(ai_player) = ai_query.single() {
        if let Ok(mut text) = difficulty_query.single_mut() {
            let difficulty_name = match ai_player.difficulty {
                AiDifficulty::Beginner => "新手",
                AiDifficulty::Intermediate => "中级",
                AiDifficulty::Advanced => "高级",
                AiDifficulty::Expert => "专家",
            };
            **text = format!("AI: {difficulty_name}");
        }
    }
}
