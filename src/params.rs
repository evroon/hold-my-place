use models::Font;
use serde::Deserialize;

use crate::models::{self, FileType};

#[derive(Deserialize, Clone)]
pub struct ImageQueryParams {
    pub text: Option<String>,
    #[serde(default = "default_color")]
    pub color: String,
    #[serde(default = "default_background_color")]
    pub background: String,
    #[serde(default = "default_font")]
    pub font: Font,
    #[serde(default = "default_filetype")]
    pub filetype: FileType,
}

pub fn default_color() -> String {
    String::from("#999999")
}

pub fn default_background_color() -> String {
    String::from("#dddddd")
}

pub fn default_font() -> Font {
    Font::Lato
}

pub fn default_filetype() -> FileType {
    FileType::Svg
}
