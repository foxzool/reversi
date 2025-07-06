use crate::localization::{Language, LanguageSettings};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct FontAssets {
    pub default_font: Handle<Font>,
    pub chinese_font: Handle<Font>,
}

pub fn load_font_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_assets = FontAssets {
        default_font: Handle::default(), // Bevy默认字体
        chinese_font: asset_server.load("fonts/NotoSerifCJKsc-Regular.otf"),
    };

    commands.insert_resource(font_assets);
}

#[derive(Component)]
pub struct LocalizedText;

pub fn update_chinese_text_fonts(
    mut text_query: Query<&mut TextFont, With<LocalizedText>>,
    font_assets: Res<FontAssets>,
    language_settings: Res<LanguageSettings>,
) {
    // 只在语言设置改变时更新字体
    if language_settings.is_changed() {
        for mut text_font in text_query.iter_mut() {
            match language_settings.current_language {
                Language::Chinese => {
                    text_font.font = font_assets.chinese_font.clone();
                }
                Language::English => {
                    text_font.font = font_assets.default_font.clone();
                }
            }
        }
    }
}

// 辅助函数：根据当前语言获取正确的字体
pub fn get_font_for_language(
    language_settings: &LanguageSettings,
    font_assets: &FontAssets,
) -> Handle<Font> {
    match language_settings.current_language {
        Language::Chinese => font_assets.chinese_font.clone(),
        Language::English => font_assets.default_font.clone(),
    }
}
