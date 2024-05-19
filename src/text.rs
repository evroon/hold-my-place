use ab_glyph::{point, Font, FontRef, GlyphId, PxScale, ScaleFont};
use rusttype::{Font, Point, Rect, Scale};
use svg::node::element::Path;

pub fn _build_text(
    font: &FontRef,
    text: &str,
    fill: &str,
    size: f32,
    start: Point<f32>,
    letter_spacing: f32,
) -> (Path, Rect<f32>) {
    let mut d = String::new();
    let mut x = start.x;
    let (mut lowest, mut highest) = (0.0f32, 0.0f32);
    let (mut w, mut h) = (0f32, 0f32);
    let mut last: Option<GlyphId> = None;

    let font_scaled = font.as_scaled(size);

    let scale = PxScale::from(size);
    let glyphs_height = font_scaled.ascent() - font_scaled.descent();

    for c in text.to_string().chars() {
        let glyph_id = font.glyph_id(c);
        let glyph = glyph_id.with_scale_and_position(scale, point(w, font_scaled.ascent()));

        w += font_scaled.h_advance(glyph_id);

        if let Some(g) = font.outline_glyph(glyph) {
            if let Some(last) = last {
                w += font_scaled.kern(glyph_id, last);
            }
            last = Some(glyph_id);
            let bb = g.px_bounds();
            h = h.max(bb.height());
            g.glyph()

            g.outline
            .curves
            .iter().map(|x| {});

            glyph.build_outline(&mut crate::text_builder::Builder {
                x,
                y: glyphs_height + bb.min.y,
                d: &mut d,
            });
        }
        // if let Some(bounding_box) = glyph.unpositioned().exact_bounding_box() {
        //     x += bounding_box.min.x;

        //     highest = highest.max(bounding_box.max.y);
        //     lowest = lowest.min(bounding_box.min.y);

        //     x += bounding_box.width() + letter_spacing;
        // } else {
        //     x += letter_spacing + size * 0.25;
        // }
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
