use crate::game::{Board, PlayerColor};
use bevy::prelude::*;

#[derive(Component)]
pub struct BoardSquare {
    #[allow(dead_code)]
    pub position: u8,
}

#[derive(Component)]
pub struct Piece {
    #[allow(dead_code)]
    pub color: PlayerColor,
    #[allow(dead_code)]
    pub position: u8,
}

#[derive(Component)]
pub struct ValidMoveIndicator {
    #[allow(dead_code)]
    pub position: u8,
}

#[derive(Component)]
pub struct BoardUI;

#[derive(Component)]
pub struct ToDelete;

#[derive(Resource)]
pub struct BoardColors {
    pub board_color: bevy::prelude::Color,
    pub square_color: bevy::prelude::Color,
    pub line_color: bevy::prelude::Color,
    pub black_piece_color: bevy::prelude::Color,
    pub white_piece_color: bevy::prelude::Color,
    pub valid_move_color: bevy::prelude::Color,
    #[allow(dead_code)]
    pub hover_color: bevy::prelude::Color,
}

impl Default for BoardColors {
    fn default() -> Self {
        Self {
            board_color: bevy::prelude::Color::srgb(0.18, 0.58, 0.18),
            square_color: bevy::prelude::Color::srgb(0.16, 0.56, 0.16),
            line_color: bevy::prelude::Color::srgb(0.12, 0.45, 0.12),
            black_piece_color: bevy::prelude::Color::srgb(0.05, 0.05, 0.05),
            white_piece_color: bevy::prelude::Color::srgb(0.98, 0.98, 0.98),
            valid_move_color: bevy::prelude::Color::srgba(1.0, 1.0, 1.0, 0.4),
            hover_color: bevy::prelude::Color::srgba(1.0, 1.0, 1.0, 0.3),
        }
    }
}

pub const BOARD_SIZE: f32 = 320.0; // 减小棋盘尺寸为手机优化
pub const SQUARE_SIZE: f32 = BOARD_SIZE / 8.0;
pub const PIECE_RADIUS: f32 = SQUARE_SIZE * 0.35;

pub fn setup_board_ui(mut commands: Commands, colors: Res<BoardColors>) {
    let _board_transform = Transform::from_xyz(0.0, 0.0, 0.0);

    for row in 0..8 {
        for col in 0..8 {
            let position = (row * 8 + col) as u8;
            let x = (col as f32 - 3.5) * SQUARE_SIZE;
            let y = (3.5 - row as f32) * SQUARE_SIZE;

            let square_color = if (row + col) % 2 == 0 {
                colors.board_color
            } else {
                colors.square_color
            };

            commands.spawn((
                Sprite::from_color(square_color, Vec2::new(SQUARE_SIZE, SQUARE_SIZE)),
                Transform::from_xyz(x, y, 0.0),
                BoardSquare { position },
                BoardUI,
            ));
        }
    }

    for i in 0..9 {
        let offset = (i as f32 - 4.0) * SQUARE_SIZE;

        commands.spawn((
            Sprite::from_color(colors.line_color, Vec2::new(1.5, BOARD_SIZE)),
            Transform::from_xyz(offset, 0.0, 1.0),
            BoardUI,
        ));

        commands.spawn((
            Sprite::from_color(colors.line_color, Vec2::new(BOARD_SIZE, 1.5)),
            Transform::from_xyz(0.0, offset, 1.0),
            BoardUI,
        ));
    }
}

pub fn update_pieces(
    mut commands: Commands,
    board_query: Query<&Board>,
    piece_query: Query<Entity, With<Piece>>,
    colors: Res<BoardColors>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Ok(board) = board_query.single() {
        // 标记旧棋子为删除，而不是直接删除
        for entity in piece_query.iter() {
            commands.entity(entity).insert(ToDelete);
        }

        for position in 0..64 {
            if let Some(color) = board.get_piece(position) {
                let (row, col) = Board::position_to_coords(position);
                let x = (col as f32 - 3.5) * SQUARE_SIZE;
                let y = (3.5 - row as f32) * SQUARE_SIZE;

                let piece_color = match color {
                    PlayerColor::Black => colors.black_piece_color,
                    PlayerColor::White => colors.white_piece_color,
                };

                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(PIECE_RADIUS))),
                    MeshMaterial2d(materials.add(ColorMaterial::from(piece_color))),
                    Transform::from_xyz(x, y, 2.0),
                    Piece { color, position },
                    BoardUI,
                ));
            }
        }
    }
}

pub fn update_valid_moves(
    mut commands: Commands,
    board_query: Query<&Board>,
    current_player: Res<CurrentPlayer>,
    valid_move_query: Query<Entity, With<ValidMoveIndicator>>,
    colors: Res<BoardColors>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 标记旧的有效移动指示器为删除
    for entity in valid_move_query.iter() {
        commands.entity(entity).insert(ToDelete);
    }

    if let Ok(board) = board_query.single() {
        let valid_moves = board.get_valid_moves_list(current_player.0);

        for move_option in valid_moves {
            let (row, col) = Board::position_to_coords(move_option.position);
            let x = (col as f32 - 3.5) * SQUARE_SIZE;
            let y = (3.5 - row as f32) * SQUARE_SIZE;

            commands.spawn((
                Mesh2d(meshes.add(Circle::new(PIECE_RADIUS * 0.6))),
                MeshMaterial2d(materials.add(ColorMaterial::from(colors.valid_move_color))),
                Transform::from_xyz(x, y, 1.5),
                ValidMoveIndicator {
                    position: move_option.position,
                },
            ));
        }
    }
}

#[derive(Resource)]
pub struct CurrentPlayer(pub PlayerColor);

pub fn cleanup_marked_entities(
    mut commands: Commands,
    marked_entities: Query<Entity, With<ToDelete>>,
) {
    for entity in marked_entities.iter() {
        if let Ok(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn();
        }
    }
}
