use rusttype::{Font, Point, Rect, Scale};
use svg::node::element::Path;

use super::text_builder::Builder;

pub fn build_text(
    font: &Font,
    text: &str,
    fill: &str,
    size: f32,
    start: Point<f32>,
    letter_spacing: f32,
) -> (Path, Rect<f32>) {
    let mut d = String::new();
    let mut x = start.x;
    let (mut lowest, mut highest) = (0.0f32, 0.0f32);

    let scale = Scale::uniform(size);
    let v_metrics = font.v_metrics(scale);
    let glyphs_height = v_metrics.ascent - v_metrics.descent;

    for glyph in font.layout(text, scale, Point { x, y: start.y }) {
        if let Some(bounding_box) = glyph.unpositioned().exact_bounding_box() {
            x += bounding_box.min.x;

            glyph.build_outline(&mut Builder {
                x,
                y: glyphs_height + bounding_box.min.y,
                d: &mut d,
            });

            highest = highest.max(bounding_box.max.y);
            lowest = lowest.min(bounding_box.min.y);

            x += bounding_box.width() + letter_spacing;
        } else {
            x += letter_spacing + size * 0.25;
        }
    }

    let bounding_box = Rect {
        min: Point {
            x: start.x,
            y: lowest,
        },
        max: Point { x, y: highest },
    };
    (Path::new().set("d", d).set("fill", fill), bounding_box)
}
