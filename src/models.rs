use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]

pub enum Font {
    Lato,
}

impl Font {
    pub fn get_bytes(self) -> &'static [u8] {
        match self {
            Font::Lato => {
                include_bytes!("../fonts/Lato-SemiBold.ttf")
            }
        }
    }
}
