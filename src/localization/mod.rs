pub mod texts;

pub use texts::*;
use bevy::prelude::*;

/// 支持的语言枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

impl Default for Language {
    fn default() -> Self {
        Self::English
    }
}

/// 语言设置资源
#[derive(Resource, Debug, Clone)]
pub struct LanguageSettings {
    pub current_language: Language,
}

impl Default for LanguageSettings {
    fn default() -> Self {
        Self {
            current_language: Language::English,
        }
    }
}

/// 语言切换事件
#[derive(Event)]
pub struct ChangeLanguageEvent {
    pub language: Language,
}

impl LanguageSettings {
    /// 获取当前语言的本地化文本
    pub fn get_texts(&self) -> &'static LocalizedTexts {
        match self.current_language {
            Language::English => &ENGLISH_TEXTS,
            Language::Chinese => &CHINESE_TEXTS,
        }
    }

    /// 切换到指定语言
    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }
}