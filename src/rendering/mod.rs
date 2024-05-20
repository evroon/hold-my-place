pub(crate) mod png;
pub(crate) mod svg;
mod text;
mod text_builder;

use crate::models::Font;

pub(crate) type RenderFunc = for<'a, 'b, 'c> fn(
    u32,
    u32,
    &'a str,
    &'b str,
    &'c str,
    Font,
) -> Result<Vec<u8>, std::string::String>;
