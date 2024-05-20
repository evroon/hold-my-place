use serde::{Deserialize, Serialize};

use crate::rendering::png::render_png;
use crate::rendering::svg::render_svg;
use crate::rendering::RenderFunc;

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Png,
    Svg,
}

impl FileType {
    pub fn get_content_type(self) -> String {
        String::from(match self {
            FileType::Png => "image/png",
            FileType::Svg => "image/svg+xml",
        })
    }

    pub fn get_render_func(self) -> RenderFunc {
        match self {
            FileType::Png => render_png,
            FileType::Svg => render_svg,
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Font {
    Lato,
    Lora,
    Montserrat,
    Roboto,
    NotoEmoji,
}

impl Font {
    pub fn get_bytes(self) -> &'static [u8] {
        match self {
            Font::Lato => {
                include_bytes!("../assets/fonts/lato-semibold.ttf")
            }
            Font::Lora => {
                include_bytes!("../assets/fonts/lora-semibold.ttf")
            }
            Font::Montserrat => {
                include_bytes!("../assets/fonts/montserrat-semibold.ttf")
            }
            Font::Roboto => {
                include_bytes!("../assets/fonts/roboto-regular.ttf")
            }
            Font::NotoEmoji => {
                include_bytes!("../assets/fonts/notocoloremoji-regular.ttf")
            }
        }
    }
}
