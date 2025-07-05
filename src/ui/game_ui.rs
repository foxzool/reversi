use super::{CurrentPlayer, ToggleRulesEvent, UiState, RestartGameEvent};
use crate::{
    ai::{AiDifficulty, AiPlayer},
    fonts::{FontAssets, LocalizedText, get_font_for_language},
    game::{Board, PlayerColor},
    localization::LanguageSettings,
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

#[derive(Component)]
pub struct RulesButton;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct RulesPanel;

#[derive(Component)]
pub struct GameUI;

pub fn setup_game_ui(
    mut commands: Commands,
    language_settings: Res<LanguageSettings>,
    font_assets: Res<FontAssets>,
) {
    let font = get_font_for_language(&language_settings, &font_assets);
    let texts = language_settings.get_texts();
    // 创建根UI容器
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        }, GameUI))
        .with_children(|parent| {
            // 顶部区域 - Bill
            parent
                .spawn((Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(120.0),  // 增加高度为手机优化
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
                        Text::new("Bill"), // AI玩家名称保持英文
                        TextFont {
                            font: font.clone(),
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        PlayerNameText {
                            player_color: PlayerColor::White,
                        },
                        LocalizedText,
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
                    height: Val::Px(120.0),  // 增加高度为手机优化
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|bottom_parent| {
                    // Your turn文本
                    bottom_parent.spawn((
                        Text::new(texts.your_turn),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        Node {
                            margin: UiRect::bottom(Val::Px(8.0)),
                            ..default()
                        },
                        TurnIndicator,
                        LocalizedText,
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

    // 游戏信息面板 - 右上角
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(8.0),
                top: Val::Px(8.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                align_items: AlignItems::End,
                padding: UiRect::all(Val::Px(8.0)),
                max_width: Val::Px(120.0),  // 限制最大宽度适应手机屏幕
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            BorderRadius::all(Val::Px(6.0)),
            GameUI,
        ))
        .with_children(|parent| {
            // 分数显示
            parent.spawn((
                Text::new(texts.score_format.replacen("{}", "2", 1).replacen("{}", "2", 1)),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,  // 手机优化尺寸
                    ..default()
                },
                TextColor(Color::WHITE),
                ScoreText,
                LocalizedText,
            ));

            // AI难度显示
            parent.spawn((
                Text::new(texts.ai_difficulty_format.replace("{}", texts.difficulty_medium)),
                TextFont {
                    font: font.clone(),
                    font_size: 12.0,  // 手机优化尺寸
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                DifficultyText,
                LocalizedText,
            ));

            // 规则按钮
            parent.spawn((
                Button,
                Node {
                    padding: UiRect::all(Val::Px(4.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.8)),
                BorderColor(Color::srgb(0.6, 0.6, 0.6)),
                BorderRadius::all(Val::Px(4.0)),
                RulesButton,
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("?"), // 规则按钮符号保持通用
                    TextFont {
                        font: font.clone(),
                        font_size: 16.0,  // 手机优化尺寸
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    LocalizedText,
                ));
            });

            // 重新开始按钮
            parent.spawn((
                Button,
                Node {
                    padding: UiRect::all(Val::Px(4.0)),
                    align_self: AlignSelf::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.8, 0.2, 0.2, 0.8)),
                BorderColor(Color::srgb(1.0, 0.4, 0.4)),
                BorderRadius::all(Val::Px(4.0)),
                RestartButton,
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("↻"), // 重新开始按钮符号保持通用
                    TextFont {
                        font: font.clone(),
                        font_size: 16.0,  // 手机优化尺寸
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    LocalizedText,
                ));
            });
        });

    // 游戏状态信息 - 右下角
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(8.0),
            bottom: Val::Px(8.0),
            padding: UiRect::all(Val::Px(8.0)),
            max_width: Val::Px(180.0),  // 限制最大宽度适应手机屏幕
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        BorderRadius::all(Val::Px(6.0)),
        Text::new(texts.game_in_progress),
        TextFont {
            font: font.clone(),
            font_size: 12.0,  // 手机优化尺寸
            ..default()
        },
        TextColor(Color::WHITE),
        GameStatusText,
        LocalizedText,
        GameUI,
    ));
}

pub fn update_score_text(
    mut score_query: Query<&mut Text, With<ScoreText>>,
    board_query: Query<&Board>,
    language_settings: Res<LanguageSettings>,
) {
    if let (Ok(mut text), Ok(board)) = (score_query.single_mut(), board_query.single()) {
        let black_count = board.count_pieces(PlayerColor::Black);
        let white_count = board.count_pieces(PlayerColor::White);
        let texts = language_settings.get_texts();
        **text = texts.score_format
            .replacen("{}", &black_count.to_string(), 1)  // 只替换第一个{}
            .replacen("{}", &white_count.to_string(), 1); // 再替换下一个{}
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
    language_settings: Res<LanguageSettings>,
) {
    if let (Ok(mut text), Ok(board)) = (status_query.single_mut(), board_query.single()) {
        let texts = language_settings.get_texts();
        
        if board.is_game_over() {
            if let Some(winner) = board.get_winner() {
                **text = format!("{} {}", 
                    match winner {
                        PlayerColor::Black => texts.black_wins,
                        PlayerColor::White => texts.white_wins,
                    },
                    texts.click_to_restart
                );
            } else {
                **text = format!("{} {}", texts.draw, texts.click_to_restart);
            }
        } else if !board.has_valid_moves(current_player.0) {
            **text = format!("{:?} {}", current_player.0, texts.pass_turn);
        } else {
            **text = texts.game_in_progress.to_string();
        }
    }
}

pub fn update_turn_indicator(
    mut turn_query: Query<&mut Text, With<TurnIndicator>>,
    current_player: Res<CurrentPlayer>,
    language_settings: Res<LanguageSettings>,
) {
    if current_player.is_changed() {
        if let Ok(mut text) = turn_query.single_mut() {
            let texts = language_settings.get_texts();
            match current_player.0 {
                PlayerColor::Black => **text = texts.your_turn.to_string(),
                PlayerColor::White => **text = texts.ai_turn.to_string(),
            }
        }
    }
}

pub fn update_difficulty_text(
    mut difficulty_query: Query<&mut Text, With<DifficultyText>>,
    ai_query: Query<&AiPlayer, Changed<AiPlayer>>,
    language_settings: Res<LanguageSettings>,
) {
    if let Ok(ai_player) = ai_query.single() {
        if let Ok(mut text) = difficulty_query.single_mut() {
            let texts = language_settings.get_texts();
            let difficulty_name = match ai_player.difficulty {
                AiDifficulty::Beginner => texts.difficulty_easy,
                AiDifficulty::Intermediate => texts.difficulty_medium,
                AiDifficulty::Advanced => texts.difficulty_hard,
                AiDifficulty::Expert => texts.difficulty_expert,
            };
            **text = texts.ai_difficulty_format.replace("{}", difficulty_name);
        }
    }
}

pub fn handle_rules_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RulesButton>)>,
    mut rules_events: EventWriter<ToggleRulesEvent>,
) {
    for interaction in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            rules_events.write(ToggleRulesEvent);
        }
    }
}

pub fn manage_rules_panel(
    mut commands: Commands,
    ui_state: Res<UiState>,
    rules_panel_query: Query<Entity, With<RulesPanel>>,
    language_settings: Res<LanguageSettings>,
    font_assets: Res<FontAssets>,
) {
    if ui_state.is_changed() {
        // 标记现有的规则面板为删除
        for entity in rules_panel_query.iter() {
            commands.entity(entity).insert(super::ToDelete);
        }

        // 如果需要显示规则，创建新的面板
        if ui_state.show_rules {
            spawn_rules_panel(&mut commands, &language_settings, &font_assets);
        }
    }
}

fn spawn_rules_panel(
    commands: &mut Commands, 
    language_settings: &LanguageSettings,
    font_assets: &FontAssets,
) {
    let texts = language_settings.get_texts();
    let font = get_font_for_language(language_settings, font_assets);
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(50.0),
                width: Val::Px(400.0),
                max_height: Val::Px(500.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(-200.0, -250.0, 10.0)),
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.95)),
            BorderColor(Color::srgb(0.6, 0.6, 0.6)),
            BorderRadius::all(Val::Px(10.0)),
            RulesPanel,
        ))
        .with_children(|panel| {
            // 标题
            panel.spawn((
                Text::new(texts.rules_title),
                TextFont {
                    font: font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
                LocalizedText,
            ));

            // 规则内容
            panel.spawn((
                Text::new(texts.rules_content),
                TextFont {
                    font: font.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
                LocalizedText,
            ));

            // 关闭按钮
            panel.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                BorderColor(Color::srgb(0.6, 0.6, 0.6)),
                BorderRadius::all(Val::Px(5.0)),
                RulesButton, // 复用按钮组件来关闭
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new(texts.rules_close),
                    TextFont {
                        font: font.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    LocalizedText,
                ));
            });
        });
}

pub fn handle_restart_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut restart_events: EventWriter<RestartGameEvent>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            restart_events.write(RestartGameEvent);
        }
    }
}
