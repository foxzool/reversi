mod ai;
mod audio;
mod fonts;
mod game;
mod localization;
mod ui;

use ai::{AiDifficulty, AiPlayer};
use audio::{
    load_audio_assets, play_sound_system, toggle_audio_system, AudioSettings, PlaySoundEvent,
    SoundType,
};
use bevy::prelude::*;
use fonts::{get_font_for_language, load_font_assets, update_chinese_text_fonts, FontAssets, LocalizedText};
use game::{Board, Move, PlayerColor};
use localization::{ChangeLanguageEvent, Language, LanguageSettings};
use reversi::systems::GameSystems;
use ui::{
    cleanup_marked_entities, handle_restart_button, handle_rules_button, manage_rules_panel,
    setup_board_ui, setup_game_ui, update_current_player_text, update_difficulty_text,
    update_game_status_text, update_pieces, update_score_text, update_turn_indicator,
    update_valid_moves, BackToDifficultyButton, BoardColors, BoardUI, ButtonColors, CurrentPlayer, GameUI, Piece, RestartGameEvent,
    RulesPanel, ToDelete, ToggleRulesEvent, UiState, ValidMoveIndicator, SQUARE_SIZE,
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    LoadingScreen,
    LanguageSelection,
    DifficultySelection,
    Playing,
    GameOver,
    Restarting,
}

#[derive(Event)]
pub struct PlayerMoveEvent {
    pub position: u8,
}

#[derive(Event)]
pub struct AiMoveEvent {
    pub ai_move: Move,
}

#[derive(Event)]
pub struct BackToDifficultyEvent;

#[derive(Resource)]
pub struct SelectedDifficulty(pub AiDifficulty);

impl Default for SelectedDifficulty {
    fn default() -> Self {
        Self(AiDifficulty::Intermediate)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Reversi".to_string(),
                resolution: (400.0, 600.0).into(), // 手机比例优化
                // 移动端适配设置
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_event::<PlayerMoveEvent>()
        .add_event::<AiMoveEvent>()
        .add_event::<PlaySoundEvent>()
        .add_event::<RestartGameEvent>()
        .add_event::<ToggleRulesEvent>()
        .add_event::<ChangeLanguageEvent>()
        .add_event::<BackToDifficultyEvent>()
        .init_resource::<BoardColors>()
        .init_resource::<SelectedDifficulty>()
        .init_resource::<AudioSettings>()
        .init_resource::<UiState>()
        .init_resource::<LanguageSettings>()
        .init_resource::<FontAssets>()
        .init_resource::<RestartTimer>()
        .insert_resource(CurrentPlayer(PlayerColor::Black))
        .insert_resource(ClearColor(Color::srgb(0.18, 0.58, 0.18)))
        .add_systems(Startup, (load_audio_assets, load_font_assets, setup_camera))
        // Loading Screen 状态系统
        .add_systems(OnEnter(GameState::LoadingScreen), setup_loading_screen)
        .add_systems(
            Update,
            (
                check_loading_complete,
                animate_loading_text,
            ).run_if(in_state(GameState::LoadingScreen)),
        )
        // 语言选择状态系统
        .add_systems(OnEnter(GameState::LanguageSelection), setup_language_selection)
        .add_systems(
            Update,
            (
                handle_language_selection,
                update_button_interactions,
                update_fade_in_effects,
            ).run_if(in_state(GameState::LanguageSelection)),
        )
        // 难度选择状态系统
        .add_systems(OnEnter(GameState::DifficultySelection), setup_difficulty_selection)
        .add_systems(
            Update,
            (
                handle_difficulty_selection,
                handle_rules_button,
                manage_rules_panel,
                update_button_interactions,
                update_fade_in_effects,
            ).run_if(in_state(GameState::DifficultySelection)),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            (setup_board_ui, setup_game_ui, setup_game, update_pieces),
        )
        // 游戏进行状态系统
        .add_systems(
            Update,
            (
                // 游戏核心逻辑
                (
                    handle_input,
                    handle_player_move,
                    handle_ai_move,
                    ai_system,
                    check_game_over,
                )
                    .chain() // 确保顺序执行
                    .in_set(GameSystems::Gameplay),
                // UI更新
                (
                    update_pieces,
                    update_valid_moves,
                    update_score_text,
                    update_current_player_text,
                    update_game_status_text,
                    update_turn_indicator,
                    update_difficulty_text,
                    handle_restart_button,
                    handle_back_to_difficulty_button,
                    update_button_interactions,
                )
                    .in_set(GameSystems::UI),
            )
                .run_if(in_state(GameState::Playing)),
        )
        // 游戏结束状态系统
        .add_systems(
            Update,
            handle_game_over_input.run_if(in_state(GameState::GameOver)),
        )
        // 重新开始状态处理
        .add_systems(OnEnter(GameState::Restarting), (setup_restart_timer,))
        .add_systems(
            Update,
            handle_restart_state.run_if(in_state(GameState::Restarting)),
        )
        // 通用系统 - 在所有状态下运行
        .add_systems(
            Update,
            (
                play_sound_system,
                toggle_audio_system,
                restart_game,
                handle_rules_toggle,
                handle_language_change,
                handle_back_to_difficulty_event,
                update_chinese_text_fonts,
            )
                .in_set(GameSystems::Common),
        )
        // 清理系统单独运行，确保在所有其他系统之后
        .add_systems(Update, cleanup_marked_entities.after(GameSystems::Common))
        // 配置系统依赖关系
        .configure_sets(
            Update,
            (GameSystems::Gameplay, GameSystems::UI, GameSystems::Common).chain(), // 按顺序执行
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    // 创建共享的2D相机
    commands.spawn(Camera2d);
}

fn setup_game(
    mut commands: Commands,
    selected_difficulty: Res<SelectedDifficulty>,
) {
    commands.spawn(Board::new());

    // 使用用户选择的难度创建AI
    commands.spawn(AiPlayer::new(
        selected_difficulty.0,
        PlayerColor::White,
    ));
    
    println!("Game started with difficulty: {:?}", selected_difficulty.0);
}

fn handle_input(
    mut move_events: EventWriter<PlayerMoveEvent>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    _keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_input: Res<Touches>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    current_player: Res<CurrentPlayer>,
    ai_query: Query<&AiPlayer>,
) {
    // 难度现在在游戏开始前选择，不再支持游戏中切换

    // 检查是否有输入事件（鼠标点击或触摸）
    let input_position = if mouse_input.just_pressed(MouseButton::Left) {
        // 鼠标输入
        let Ok(window) = windows.single() else {
            return;
        };
        window.cursor_position()
    } else if let Some(touch) = touch_input.first_pressed_position() {
        // 触摸输入 - 支持手机触摸
        Some(touch)
    } else {
        // 没有输入事件
        return;
    };

    // 检查是否轮到玩家
    if let Ok(ai_player) = ai_query.single() {
        if ai_player.color == current_player.0 {
            return;
        }
    }

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    if let Some(screen_position) = input_position {
        if let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, screen_position) {
            let col = ((world_position.x + SQUARE_SIZE * 4.0) / SQUARE_SIZE) as i32;
            let row = ((SQUARE_SIZE * 4.0 - world_position.y) / SQUARE_SIZE) as i32;

            if (0..8).contains(&col) && (0..8).contains(&row) {
                let position = (row * 8 + col) as u8;
                move_events.write(PlayerMoveEvent { position });
            }
        }
    }
}

fn handle_player_move(
    mut move_events: EventReader<PlayerMoveEvent>,
    mut board_query: Query<&mut Board>,
    mut current_player: ResMut<CurrentPlayer>,
    mut sound_events: EventWriter<PlaySoundEvent>,
) {
    for event in move_events.read() {
        if let Ok(mut board) = board_query.single_mut() {
            if board.is_valid_move(event.position, current_player.0) {
                board.make_move(event.position, current_player.0);

                // 播放落子音效
                sound_events.write(PlaySoundEvent {
                    sound_type: SoundType::PiecePlace,
                });

                // 播放翻转音效
                sound_events.write(PlaySoundEvent {
                    sound_type: SoundType::PieceFlip,
                });

                let next_player = current_player.0.opposite();
                if board.has_valid_moves(next_player) {
                    current_player.0 = next_player;
                } else if !board.has_valid_moves(current_player.0) {
                    // 游戏结束
                }
            } else {
                // 播放无效落子音效
                sound_events.write(PlaySoundEvent {
                    sound_type: SoundType::InvalidMove,
                });
            }
        }
    }
}

fn ai_system(
    mut ai_query: Query<&mut AiPlayer>,
    board_query: Query<&Board>,
    current_player: Res<CurrentPlayer>,
    mut ai_move_events: EventWriter<AiMoveEvent>,
    time: Res<Time>,
) {
    if let Ok(mut ai_player) = ai_query.single_mut() {
        if ai_player.color != current_player.0 {
            return;
        }

        ai_player.thinking_timer.tick(time.delta());

        if ai_player.thinking_timer.finished() {
            if let Ok(board) = board_query.single() {
                if let Some(ai_move) = ai_player.difficulty.get_ai_move(board, ai_player.color) {
                    ai_move_events.write(AiMoveEvent { ai_move });
                    ai_player.thinking_timer.reset();
                }
            }
        }
    }
}

fn handle_ai_move(
    mut ai_move_events: EventReader<AiMoveEvent>,
    mut board_query: Query<&mut Board>,
    mut current_player: ResMut<CurrentPlayer>,
    mut sound_events: EventWriter<PlaySoundEvent>,
) {
    for event in ai_move_events.read() {
        if let Ok(mut board) = board_query.single_mut() {
            if board.make_move(event.ai_move.position, current_player.0) {
                // 播放AI落子音效
                sound_events.write(PlaySoundEvent {
                    sound_type: SoundType::PiecePlace,
                });

                // 播放翻转音效
                sound_events.write(PlaySoundEvent {
                    sound_type: SoundType::PieceFlip,
                });

                let next_player = current_player.0.opposite();
                if board.has_valid_moves(next_player) {
                    current_player.0 = next_player;
                } else if !board.has_valid_moves(current_player.0) {
                    // 游戏结束
                }
            }
        }
    }
}

fn check_game_over(
    board_query: Query<&Board>,
    mut next_state: ResMut<NextState<GameState>>,
    mut sound_events: EventWriter<PlaySoundEvent>,
    ai_query: Query<&AiPlayer>,
    current_state: Res<State<GameState>>,
) {
    // 只在Playing状态下检查游戏结束
    if current_state.get() != &GameState::Playing {
        return;
    }

    if let Ok(board) = board_query.single() {
        if board.is_game_over() {
            println!("Game over detected!");

            // 播放游戏结束音效
            if let Some(winner) = board.get_winner() {
                // 如果有AI玩家，判断是玩家胜利还是AI胜利
                if let Ok(ai_player) = ai_query.single() {
                    if winner == ai_player.color {
                        // AI胜利，玩家失败
                        println!("Game over: AI wins, playing defeat sound");
                        sound_events.write(PlaySoundEvent {
                            sound_type: SoundType::Defeat,
                        });
                    } else {
                        // 玩家胜利
                        println!("Game over: Player wins, playing victory sound");
                        sound_events.write(PlaySoundEvent {
                            sound_type: SoundType::Victory,
                        });
                    }
                } else {
                    // 没有AI，根据黑棋结果判断（玩家是黑棋）
                    if winner == PlayerColor::Black {
                        println!("Game over: Black wins, playing victory sound");
                        sound_events.write(PlaySoundEvent {
                            sound_type: SoundType::Victory,
                        });
                    } else {
                        println!("Game over: White wins, playing defeat sound");
                        sound_events.write(PlaySoundEvent {
                            sound_type: SoundType::Defeat,
                        });
                    }
                }
            } else {
                // 平局，播放胜利音效（因为没有输）
                println!("Game over: Draw, playing victory sound");
                sound_events.write(PlaySoundEvent {
                    sound_type: SoundType::Victory,
                });
            }

            next_state.set(GameState::GameOver);
        }
    }
}

fn handle_game_over_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_input: Res<Touches>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut restart_events: EventWriter<RestartGameEvent>,
) {
    // 键盘输入（桌面端）
    let keyboard_restart =
        keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::Enter);

    // 触摸输入（移动端）
    let touch_restart = touch_input.any_just_pressed();

    // 鼠标输入（桌面端备用）
    let mouse_restart = mouse_input.just_pressed(MouseButton::Left);

    if keyboard_restart || touch_restart || mouse_restart {
        println!("Restarting game");
        restart_events.write(RestartGameEvent);
    }
}

fn restart_game(
    mut restart_events: EventReader<RestartGameEvent>,
    mut commands: Commands,
    board_entities: Query<Entity, With<Board>>,
    ai_entities: Query<Entity, With<AiPlayer>>,
    mut current_player: ResMut<CurrentPlayer>,
    mut next_state: ResMut<NextState<GameState>>,
    // 查询游戏UI实体
    game_ui_entities: Query<Entity, With<GameUI>>,
    board_ui_entities: Query<Entity, With<BoardUI>>,
    piece_entities: Query<Entity, With<Piece>>,
    valid_move_entities: Query<Entity, With<ValidMoveIndicator>>,
    // 添加资源用于重新创建UI
    _language_settings: Res<LanguageSettings>,
    _font_assets: Res<FontAssets>,
    _colors: Res<BoardColors>,
) {
    for _event in restart_events.read() {
        println!("Executing game restart");

        // 标记游戏UI实体为删除
        for entity in game_ui_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }

        // 标记棋盘UI实体为删除
        for entity in board_ui_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }

        // 标记棋子实体为删除
        for entity in piece_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }

        // 标记有效移动指示器为删除
        for entity in valid_move_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }

        // 标记Board实体为删除
        for entity in board_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }

        // 标记AI实体为删除
        for entity in ai_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }

        // 重置当前玩家为黑棋
        current_player.0 = PlayerColor::Black;

        // 通过状态切换来重新创建UI
        // 切换到Restarting状态，然后会自动切换回Playing
        next_state.set(GameState::Restarting);
    }
}

#[derive(Resource)]
struct RestartTimer {
    timer: Timer,
}

impl Default for RestartTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Once), // 100ms延迟
        }
    }
}

fn setup_restart_timer(mut restart_timer: ResMut<RestartTimer>) {
    restart_timer.timer.reset();
    println!("Reset restart timer");
}

fn handle_restart_state(
    mut restart_timer: ResMut<RestartTimer>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    restart_timer.timer.tick(time.delta());

    if restart_timer.timer.finished() {
        println!("Restart timer finished, switching to Playing state");
        next_state.set(GameState::Playing);
    }
}

fn handle_rules_toggle(
    mut rules_events: EventReader<ToggleRulesEvent>,
    mut ui_state: ResMut<UiState>,
) {
    for _event in rules_events.read() {
        ui_state.show_rules = !ui_state.show_rules;
        println!("Rules panel toggled: {}", ui_state.show_rules);
    }
}

// UI组件定义
#[derive(Component)]
struct LoadingScreenUI;

#[derive(Component)]
struct LoadingText {
    timer: Timer,
}

#[derive(Component)]
struct FadeIn {
    timer: Timer,
    duration: f32,
}

impl Default for LoadingText {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl FadeIn {
    fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            duration,
        }
    }
}

#[derive(Component)]
struct LanguageSelectionUI;

#[derive(Component)]
struct LanguageButton {
    language: Language,
}

#[derive(Component)]
struct DifficultySelectionUI;

#[derive(Component)]
struct DifficultyButton {
    difficulty: AiDifficulty,
}



// Loading Screen 相关函数
fn setup_loading_screen(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    language_settings: Res<LanguageSettings>,
) {
    let font = get_font_for_language(&language_settings, &font_assets);
    let texts = language_settings.get_texts();
    
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            LoadingScreenUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(texts.loading_text),
                TextFont {
                    font: font.clone(),
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                LocalizedText,
                LoadingText::default(),
            ));
        });
}

fn check_loading_complete(
    asset_server: Res<AssetServer>,
    font_assets: Res<FontAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    loading_ui_query: Query<Entity, With<LoadingScreenUI>>,
    mut commands: Commands,
) {
    // 检查字体是否加载完成
    match asset_server.load_state(&font_assets.chinese_font) {
        bevy::asset::LoadState::Loaded => {
            // 清理Loading UI
            for entity in loading_ui_query.iter() {
                commands.entity(entity).insert(ToDelete);
            }
            // 切换到语言选择
            next_state.set(GameState::LanguageSelection);
        }
        _ => {}
    }
}

// 语言选择状态
fn setup_language_selection(
    commands: Commands,
    language_settings: Res<LanguageSettings>,
    font_assets: Res<FontAssets>,
) {
    setup_language_selection_ui(commands, language_settings, font_assets);
}

fn setup_language_selection_ui(
    mut commands: Commands,
    _language_settings: Res<LanguageSettings>,
    font_assets: Res<FontAssets>,
) {
    // 总是使用中文字体以确保"中文"按钮能正确显示
    let font = font_assets.chinese_font.clone();

    // 语言选择界面
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            LanguageSelectionUI,
            FadeIn::new(0.5),
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new("Select Language / 选择语言"),
                TextFont {
                    font: font.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                LocalizedText,
            ));

            // 按钮容器
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|buttons| {
                    // English 按钮
                    let english_normal = Color::srgb(0.2, 0.2, 0.8);
                    buttons
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(english_normal),
                            BorderColor(Color::srgb(0.4, 0.4, 1.0)),
                            BorderRadius::all(Val::Px(10.0)),
                            LanguageButton {
                                language: Language::English,
                            },
                            ButtonColors {
                                normal: english_normal,
                                hovered: Color::srgb(0.3, 0.3, 0.9),
                                pressed: Color::srgb(0.1, 0.1, 0.7),
                            },
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("English"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                LocalizedText,
                            ));
                        });

                    // 中文 按钮
                    let chinese_normal = Color::srgb(0.8, 0.2, 0.2);
                    buttons
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(chinese_normal),
                            BorderColor(Color::srgb(1.0, 0.4, 0.4)),
                            BorderRadius::all(Val::Px(10.0)),
                            LanguageButton {
                                language: Language::Chinese,
                            },
                            ButtonColors {
                                normal: chinese_normal,
                                hovered: Color::srgb(0.9, 0.3, 0.3),
                                pressed: Color::srgb(0.7, 0.1, 0.1),
                            },
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("中文"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                                LocalizedText,
                            ));
                        });
                });
        });
}

fn handle_language_selection(
    interaction_query: Query<
        (&Interaction, &LanguageButton),
        (Changed<Interaction>, With<LanguageButton>),
    >,
    mut language_events: EventWriter<ChangeLanguageEvent>,
    mut language_settings: ResMut<LanguageSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    ui_query: Query<Entity, With<LanguageSelectionUI>>,
) {
    for (interaction, language_button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // 设置语言
            language_settings.set_language(language_button.language);

            // 发送语言切换事件
            language_events.write(ChangeLanguageEvent {
                language: language_button.language,
            });

            // 标记语言选择UI为删除
            for entity in ui_query.iter() {
                commands.entity(entity).insert(ToDelete);
            }

            // 切换到难度选择状态
            next_state.set(GameState::DifficultySelection);

            println!("Language selected: {:?}", language_button.language);
        }
    }
}

fn handle_language_change(
    mut language_events: EventReader<ChangeLanguageEvent>,
    mut language_settings: ResMut<LanguageSettings>,
) {
    for event in language_events.read() {
        language_settings.set_language(event.language);
        println!("Language changed to: {:?}", event.language);
    }
}

// Difficulty Selection 相关函数
fn setup_difficulty_selection(
    mut commands: Commands,
    language_settings: Res<LanguageSettings>,
    font_assets: Res<FontAssets>,
) {
    let font = get_font_for_language(&language_settings, &font_assets);
    let texts = language_settings.get_texts();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
            DifficultySelectionUI,
            FadeIn::new(0.5),
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new(texts.select_difficulty),
                TextFont {
                    font: font.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
                LocalizedText,
            ));

            // 难度按钮容器
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(15.0),
                    ..default()
                })
                .with_children(|buttons| {
                    // 创建四个难度按钮
                    let difficulties = [
                        (AiDifficulty::Beginner, texts.difficulty_easy, Color::srgb(0.2, 0.7, 0.2)),
                        (AiDifficulty::Intermediate, texts.difficulty_medium, Color::srgb(0.2, 0.2, 0.7)),
                        (AiDifficulty::Advanced, texts.difficulty_hard, Color::srgb(0.7, 0.5, 0.2)),
                        (AiDifficulty::Expert, texts.difficulty_expert, Color::srgb(0.7, 0.2, 0.2)),
                    ];

                    for (difficulty, text, color) in difficulties {
                        buttons
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(250.0),
                                    height: Val::Px(50.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                BackgroundColor(color),
                                BorderColor(Color::WHITE),
                                BorderRadius::all(Val::Px(10.0)),
                                DifficultyButton { difficulty },
                                ButtonColors {
                                    normal: color,
                                    hovered: Color::srgba(color.to_srgba().red + 0.1, color.to_srgba().green + 0.1, color.to_srgba().blue + 0.1, 1.0),
                                    pressed: Color::srgba(color.to_srgba().red - 0.1, color.to_srgba().green - 0.1, color.to_srgba().blue - 0.1, 1.0),
                                },
                            ))
                            .with_children(|button| {
                                button.spawn((
                                    Text::new(text),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 22.0,
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                    LocalizedText,
                                ));
                            });
                    }
                });

            // 帮助按钮
            let help_normal = Color::srgba(0.3, 0.3, 0.3, 0.8);
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(30.0)),
                        ..default()
                    },
                    BackgroundColor(help_normal),
                    BorderColor(Color::srgb(0.6, 0.6, 0.6)),
                    BorderRadius::all(Val::Px(8.0)),
                    ui::RulesButton,
                    ButtonColors {
                        normal: help_normal,
                        hovered: Color::srgba(0.4, 0.4, 0.4, 0.9),
                        pressed: Color::srgba(0.2, 0.2, 0.2, 0.9),
                    },
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new(texts.rules_title),
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

fn handle_difficulty_selection(
    interaction_query: Query<
        (&Interaction, &DifficultyButton),
        (Changed<Interaction>, With<DifficultyButton>),
    >,
    mut selected_difficulty: ResMut<SelectedDifficulty>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    ui_query: Query<Entity, With<DifficultySelectionUI>>,
) {
    for (interaction, difficulty_button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // 设置选中的难度
            selected_difficulty.0 = difficulty_button.difficulty;
            
            // 清理难度选择UI
            for entity in ui_query.iter() {
                commands.entity(entity).insert(ToDelete);
            }
            
            // 切换到游戏状态
            next_state.set(GameState::Playing);
            
            println!("Difficulty selected: {:?}", difficulty_button.difficulty);
        }
    }
}

// 处理返回难度选择按钮点击
fn handle_back_to_difficulty_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackToDifficultyButton>)>,
    mut back_events: EventWriter<BackToDifficultyEvent>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            back_events.write(BackToDifficultyEvent);
        }
    }
}

// 处理返回难度选择事件
fn handle_back_to_difficulty_event(
    mut back_events: EventReader<BackToDifficultyEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    // 清理游戏相关实体
    board_entities: Query<Entity, With<Board>>,
    ai_entities: Query<Entity, With<AiPlayer>>,
    game_ui_entities: Query<Entity, With<GameUI>>,
    board_ui_entities: Query<Entity, With<BoardUI>>,
    piece_entities: Query<Entity, With<Piece>>,
    valid_move_entities: Query<Entity, With<ValidMoveIndicator>>,
    rules_panel_entities: Query<Entity, With<RulesPanel>>,
    mut current_player: ResMut<CurrentPlayer>,
    mut ui_state: ResMut<UiState>,
) {
    for _event in back_events.read() {
        println!("Returning to difficulty selection");
        
        // 标记游戏相关实体为删除
        // 重要：按照依赖关系顺序删除，先删除子实体，再删除父实体
        
        // 首先删除规则面板（如果打开的话）
        for entity in rules_panel_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }
        
        // 删除棋子实体
        let piece_count = piece_entities.iter().count();
        for entity in piece_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }
        println!("清理了 {} 个棋子实体", piece_count);
        
        // 删除有效移动指示器
        for entity in valid_move_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }
        
        // 删除UI实体（包含子文本实体）
        let game_ui_count = game_ui_entities.iter().count();
        for entity in game_ui_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }
        println!("清理了 {} 个游戏UI实体", game_ui_count);
        
        let board_ui_count = board_ui_entities.iter().count();
        for entity in board_ui_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }
        println!("清理了 {} 个棋盘UI实体", board_ui_count);
        
        // 最后删除游戏逻辑实体
        for entity in board_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }
        
        for entity in ai_entities.iter() {
            commands.entity(entity).insert(ToDelete);
        }
        
        // 重置游戏状态
        current_player.0 = PlayerColor::Black;
        ui_state.show_rules = false; // 重置规则面板状态
        
        // 切换到难度选择状态
        next_state.set(GameState::DifficultySelection);
    }
}

// 通用按钮交互效果
fn update_button_interactions(
    mut button_query: Query<(&Interaction, &mut BackgroundColor, &ButtonColors), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut background_color, button_colors) in button_query.iter_mut() {
        *background_color = match *interaction {
            Interaction::Pressed => button_colors.pressed.into(),
            Interaction::Hovered => button_colors.hovered.into(),
            Interaction::None => button_colors.normal.into(),
        };
    }
}

// 加载文本动画
fn animate_loading_text(
    mut loading_query: Query<(&mut LoadingText, &mut TextColor)>,
    time: Res<Time>,
) {
    for (mut loading_text, mut text_color) in loading_query.iter_mut() {
        loading_text.timer.tick(time.delta());
        
        // 计算脉冲效果的透明度
        let alpha = (loading_text.timer.elapsed_secs() * 3.14159).sin() * 0.3 + 0.7;
        text_color.0 = Color::srgba(1.0, 1.0, 1.0, alpha);
    }
}

// 淡入效果更新
fn update_fade_in_effects(
    mut fade_query: Query<(&mut FadeIn, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    for (mut fade_in, mut background_color) in fade_query.iter_mut() {
        fade_in.timer.tick(time.delta());
        
        let progress = fade_in.timer.elapsed_secs() / fade_in.duration;
        let alpha = progress.min(1.0);
        
        // 透明背景渐现效果
        background_color.0 = Color::srgba(0.0, 0.0, 0.0, alpha * 0.3);
    }
}
