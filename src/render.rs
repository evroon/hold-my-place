use ab_glyph::{FontRef, PxScale};
use csscolorparser::Color;
use image::codecs::png::PngEncoder;
use image::{ImageEncoder, Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};

use crate::models::Font;
use crate::params::{default_background_color, default_color};

fn rgba_to_rgb(color: Color) -> Rgb<u8> {
    Rgb(color.to_rgba8()[0..3].try_into().unwrap())
}

pub fn render(
    width: u32,
    height: u32,
    text: &str,
    color: &str,
    background_color: &str,
    font: Font,
) -> Result<Vec<u8>, String> {
    // Use default colors when we can't parse the colors.
    let color_rgb = rgba_to_rgb(match csscolorparser::parse(color) {
        Ok(x) => x,
        _ => csscolorparser::parse(&default_color()).unwrap(),
    });
    let background_color_rgb = rgba_to_rgb(match csscolorparser::parse(background_color) {
        Ok(x) => x,
        _ => csscolorparser::parse(&default_background_color()).unwrap(),
    });

    let mut image = RgbImage::from_pixel(width, height, background_color_rgb); // [237u8, 237u8, 237u8]

    let font = FontRef::try_from_slice(font.get_bytes()).unwrap();
    let (w_unscaled, h_unscaled) = text_size(PxScale { x: 100.0, y: 100.0 }, &font, text);

    let lines_count = text.chars().filter(|c| *c == '\n').count() + 1;

    let font_height = (width as f32 / w_unscaled as f32)
        .min(height as f32 / h_unscaled as f32 / lines_count as f32 * 0.75)
        * 65.0;

    let (w, h) = (
        (w_unscaled as f32 * font_height / 100.0) as u32,
        (h_unscaled as f32 * font_height / 100.0) as u32,
    );

    draw_text_mut(
        &mut image,
        color_rgb,
        ((width - w) / 2) as i32,
        height as i32 / 2 - h as i32,
        PxScale {
            x: font_height,
            y: font_height,
        },
        &font,
        text,
    );

    let mut cursor = std::io::Cursor::new(Vec::new());
    let encoder = PngEncoder::new(&mut cursor);
    encoder
        .write_image(&image, width, height, image::ColorType::Rgb8.into())
        .unwrap();

    Ok(cursor.into_inner())
}
