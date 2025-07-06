pub mod board_ui;
pub mod game_ui;

pub use board_ui::*;
pub use game_ui::*;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct UiState {
    pub show_rules: bool,
}

#[derive(Event)]
pub struct ToggleRulesEvent;

#[derive(Event)]
pub struct RestartGameEvent;

#[derive(Component)]
pub struct BackToDifficultyButton;

#[derive(Component)]
pub struct ButtonColors {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}
