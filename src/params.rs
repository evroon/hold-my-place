use models::Font;
use serde::Deserialize;

use crate::models;

#[derive(Deserialize)]
pub struct ImageQueryParams {
    pub text: Option<String>,
    #[serde(default = "default_color")]
    pub color: String,
    #[serde(default = "default_background_color")]
    pub background: String,
    #[serde(default = "default_font")]
    pub font: Font,
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
