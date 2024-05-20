use crate::{models::Font, rendering::text::build_text};
use rusttype::{Font as RustTypeFont, Point};
use svg::{node::element::Rectangle, Document};

pub fn render_svg(
    width: u32,
    height: u32,
    text: &str,
    color: &str,
    background_color: &str,
    font: Font,
) -> Result<Vec<u8>, String> {
    let font = RustTypeFont::try_from_bytes(font.get_bytes()).unwrap();
    let lines_count = text.chars().filter(|c| *c == '\n').count() + 1;
    let render_size = 100.0;

    let (path, bounding_box) =
        build_text(&font, text, color, render_size, Point { x: 0., y: 0. }, 3.0);

    let font_size = (width as f32 / bounding_box.width())
        .min(height as f32 / lines_count as f32 / bounding_box.height())
        * 0.7;

    let (trans_x, trans_y) = (
        (width as f32 / 2.0 / font_size - bounding_box.width() * 0.5),
        ((bounding_box.height() * 0.5 - render_size) + height as f32 / 2.0 / font_size),
    );

    let document = Document::new()
        .set("width", width)
        .set("height", height)
        .add(
            Rectangle::new()
                .set("fill", background_color)
                .set("x", 0.)
                .set("y", 0.)
                .set("width", width)
                .set("height", height),
        )
        .add(path.set(
            "transform",
            format!("scale({font_size}, {font_size}), translate({trans_x}, {trans_y})",),
        ));

    let mut output = Vec::new();
    svg::write(&mut output, &document).unwrap();
    Ok(output)
}
