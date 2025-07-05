mod ai;
mod audio;
mod game;
mod ui;

use ai::{AiDifficulty, AiPlayer};
use audio::{
    load_audio_assets, play_sound_system, toggle_audio_system, AudioSettings, PlaySoundEvent,
    SoundType,
};
use bevy::prelude::*;
use game::{Board, Move, PlayerColor};
use ui::{
    setup_board_ui, setup_game_ui, update_current_player_text, update_difficulty_text,
    update_game_status_text, update_pieces, update_score_text, update_turn_indicator,
    update_valid_moves, BoardColors, CurrentPlayer, SQUARE_SIZE,
};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
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
pub struct RestartGameEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Reversi".to_string(),
                resolution: (800.0, 800.0).into(),
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
        .init_resource::<BoardColors>()
        .init_resource::<AudioSettings>()
        .insert_resource(CurrentPlayer(PlayerColor::Black))
        .insert_resource(ClearColor(Color::srgb(0.18, 0.58, 0.18)))
        .add_systems(
            Startup,
            (setup_board_ui, setup_game_ui, setup_game, load_audio_assets),
        )
        .add_systems(
            Update,
            (
                handle_input,
                handle_player_move,
                ai_system,
                handle_ai_move,
                update_pieces,
                update_valid_moves,
                update_score_text,
                update_current_player_text,
                update_game_status_text,
                update_turn_indicator,
                update_difficulty_text,
                check_game_over,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            (
                play_sound_system,
                toggle_audio_system,
                handle_game_over_input.run_if(in_state(GameState::GameOver)),
                restart_game,
            ),
        )
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn(Board::new());

    commands.spawn(AiPlayer::new(
        AiDifficulty::Intermediate,
        PlayerColor::White,
    ));
}

fn handle_input(
    mut move_events: EventWriter<PlayerMoveEvent>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_input: Res<Touches>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    current_player: Res<CurrentPlayer>,
    mut ai_query: Query<&mut AiPlayer>,
) {
    // 处理难度切换键盘输入
    if let Ok(mut ai_player) = ai_query.single_mut() {
        if keyboard_input.just_pressed(KeyCode::Digit1) {
            ai_player.difficulty = AiDifficulty::Beginner;
            println!("AI难度切换为：初级");
        } else if keyboard_input.just_pressed(KeyCode::Digit2) {
            ai_player.difficulty = AiDifficulty::Intermediate;
            println!("AI难度切换为：中级");
        } else if keyboard_input.just_pressed(KeyCode::Digit3) {
            ai_player.difficulty = AiDifficulty::Advanced;
            println!("AI难度切换为：高级");
        } else if keyboard_input.just_pressed(KeyCode::Digit4) {
            ai_player.difficulty = AiDifficulty::Expert;
            println!("AI难度切换为：专家");
        }
    }

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
            println!("检测到游戏结束！");

            // 播放游戏结束音效
            if let Some(winner) = board.get_winner() {
                // 如果有AI玩家，判断是玩家胜利还是AI胜利
                if let Ok(ai_player) = ai_query.single() {
                    if winner == ai_player.color {
                        // AI胜利，玩家失败
                        println!("游戏结束：AI胜利，播放失败音效");
                        sound_events.write(PlaySoundEvent {
                            sound_type: SoundType::Defeat,
                        });
                    } else {
                        // 玩家胜利
                        println!("游戏结束：玩家胜利，播放胜利音效");
                        sound_events.write(PlaySoundEvent {
                            sound_type: SoundType::Victory,
                        });
                    }
                } else {
                    // 没有AI，根据黑棋结果判断（玩家是黑棋）
                    if winner == PlayerColor::Black {
                        println!("游戏结束：黑棋胜利，播放胜利音效");
                        sound_events.write(PlaySoundEvent {
                            sound_type: SoundType::Victory,
                        });
                    } else {
                        println!("游戏结束：白棋胜利，播放失败音效");
                        sound_events.write(PlaySoundEvent {
                            sound_type: SoundType::Defeat,
                        });
                    }
                }
            } else {
                // 平局，播放胜利音效（因为没有输）
                println!("游戏结束：平局，播放胜利音效");
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
    let keyboard_restart = keyboard_input.just_pressed(KeyCode::Space) 
        || keyboard_input.just_pressed(KeyCode::Enter);
    
    // 触摸输入（移动端）
    let touch_restart = touch_input.any_just_pressed();
    
    // 鼠标输入（桌面端备用）
    let mouse_restart = mouse_input.just_pressed(MouseButton::Left);
    
    if keyboard_restart || touch_restart || mouse_restart {
        println!("重新开始游戏");
        restart_events.write(RestartGameEvent);
    }
}

fn restart_game(
    mut restart_events: EventReader<RestartGameEvent>,
    mut board_query: Query<&mut Board>,
    mut current_player: ResMut<CurrentPlayer>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ai_query: Query<&mut AiPlayer>,
) {
    for _event in restart_events.read() {
        println!("执行游戏重新开始");

        // 重置棋盘
        if let Ok(mut board) = board_query.single_mut() {
            *board = Board::new();
        }

        // 重置当前玩家为黑棋
        current_player.0 = PlayerColor::Black;

        // 重置AI思考计时器
        if let Ok(mut ai_player) = ai_query.single_mut() {
            ai_player.thinking_timer.reset();
        }

        // 切换回游戏状态
        next_state.set(GameState::Playing);
    }
}
