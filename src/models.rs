use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Font {
    Lato,
    Lora,
    Montserrat,
    Roboto,
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
        }
    }
}
