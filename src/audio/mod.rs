use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioAssets {
    pub piece_place: Handle<AudioSource>,
    pub piece_flip: Handle<AudioSource>,
    pub game_over: Handle<AudioSource>,
    pub invalid_move: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct AudioSettings {
    pub enabled: bool,
    pub volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            volume: 0.5,
        }
    }
}

#[derive(Event)]
pub struct PlaySoundEvent {
    pub sound_type: SoundType,
}

#[derive(Clone)]
pub enum SoundType {
    PiecePlace,
    PieceFlip,
    GameOver,
    InvalidMove,
}

pub fn load_audio_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let audio_assets = AudioAssets {
        piece_place: asset_server.load("sounds/piece_place.ogg"),
        piece_flip: asset_server.load("sounds/piece_flip.ogg"),
        game_over: asset_server.load("sounds/game_over.ogg"),
        invalid_move: asset_server.load("sounds/invalid_move.ogg"),
    };
    
    commands.insert_resource(audio_assets);
}

pub fn play_sound_system(
    mut commands: Commands,
    mut sound_events: EventReader<PlaySoundEvent>,
    audio_assets: Res<AudioAssets>,
    audio_settings: Res<AudioSettings>,
) {
    if !audio_settings.enabled {
        return;
    }

    for event in sound_events.read() {
        let audio_source = match event.sound_type {
            SoundType::PiecePlace => &audio_assets.piece_place,
            SoundType::PieceFlip => &audio_assets.piece_flip,
            SoundType::GameOver => &audio_assets.game_over,
            SoundType::InvalidMove => &audio_assets.invalid_move,
        };

        commands.spawn(AudioPlayer::new(audio_source.clone()));
    }
}

pub fn toggle_audio_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut audio_settings: ResMut<AudioSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        audio_settings.enabled = !audio_settings.enabled;
        let status = if audio_settings.enabled { "开启" } else { "关闭" };
        println!("音效已{}", status);
    }
}