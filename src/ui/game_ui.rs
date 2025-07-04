use bevy::prelude::*;
use crate::game::{Board, PlayerColor};
use crate::ai::{AiPlayer, AiDifficulty};
use super::CurrentPlayer;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct CurrentPlayerText;

#[derive(Component)]
pub struct GameStatusText;

#[derive(Component)]
pub struct PlayerAvatar {
    pub player_color: PlayerColor,
}

#[derive(Component)]
pub struct PlayerNameText {
    pub player_color: PlayerColor,
}

#[derive(Component)]
pub struct TurnIndicator;

#[derive(Component)]
pub struct DifficultyText;

pub fn setup_game_ui(
    mut commands: Commands,
) {
    // 创建根UI容器
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
    )).with_children(|parent| {
        // 顶部区域 - Bill
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        )).with_children(|top_parent| {
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
                PlayerAvatar { player_color: PlayerColor::White },
            ));
            
            // Bill名称
            top_parent.spawn((
                Text::new("Bill"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                PlayerNameText { player_color: PlayerColor::White },
            ));
        });

        // 中间区域保留给棋盘
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                flex_grow: 1.0,
                ..default()
            },
        ));

        // 底部区域 - You
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        )).with_children(|bottom_parent| {
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
                PlayerAvatar { player_color: PlayerColor::Black },
            ));
        });
    });

    // 侧边信息面板
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
    )).with_children(|parent| {
        // 分数显示
        parent.spawn((
            Text::new("Black: 2  White: 2"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            ScoreText,
        ));

        // AI难度显示
        parent.spawn((
            Text::new("AI Difficulty: Intermediate (Press 1-4 to change)"),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            DifficultyText,
        ));
    });

    // 底部状态信息
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            bottom: Val::Px(10.0),
            ..default()
        },
        Text::new("Game in progress"),
        TextFont {
            font_size: 14.0,
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
        **text = format!("Black: {black_count}  White: {white_count}");
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
                **text = format!("{winner:?} wins!");
            } else {
                **text = "Draw!".to_string();
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
                AiDifficulty::Beginner => "Beginner",
                AiDifficulty::Intermediate => "Intermediate", 
                AiDifficulty::Advanced => "Advanced",
                AiDifficulty::Expert => "Expert",
            };
            **text = format!("AI Difficulty: {difficulty_name} (Press 1-4 to change)");
        }
    }
}