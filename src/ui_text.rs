use skrifa::{
    FontRef, MetadataProvider,
    charmap::Charmap,
    instance::Location,
    metrics::{GlyphMetrics, Metrics},
};
use vello::{
    Glyph,
    kurbo::Affine,
    peniko::{BrushRef, Font, StyleRef},
};

use crate::{canvas::Canvas, font};

pub struct UiBasicText<'a> {
    ui_font: UiFont<'a>,

    pen_x: f32,
    pen_y: f32,

    glyphs: Vec<Glyph>,
}

impl<'a> UiBasicText<'a> {
    pub fn new(ui_font: UiFont<'a>) -> Option<Self> {
        Some(Self {
            ui_font,
            pen_x: 0.0,
            pen_y: 0.0,
            glyphs: vec![],
        })
    }

    pub fn push_str<T: AsRef<str>>(&mut self, content: T) {
        let line_height = self.ui_font.line_height();
        let glyph_metrics = self.ui_font.glyph_metrics();
        let charmap = self.ui_font.charmap();

        // TODO: Use unicode grapheme clusters instead of chars
        content.as_ref().chars().for_each(|ch| {
            if ch == '\n' {
                self.pen_y += line_height;
                self.pen_x = 0.0;
            } else {
                let gid = charmap.map(ch).unwrap_or_default();
                let advance = glyph_metrics.advance_width(gid).unwrap_or_default();
                let x = self.pen_x;
                self.pen_x += advance;
                self.glyphs.push(Glyph {
                    id: gid.to_u32(),
                    x,
                    y: self.pen_y,
                });
            }
        });
    }

    pub fn draw(
        self,
        canvas: &'a mut Canvas,
        transform: Affine,
        glyph_transform: Option<Affine>,
        brush: impl Into<BrushRef<'a>>,
        style: impl Into<StyleRef<'a>>,
    ) {
        canvas.draw_text(
            transform,
            glyph_transform,
            brush,
            style,
            self.ui_font,
            self.glyphs,
        );
    }
}

pub struct UiFont<'a> {
    font: &'a Font,
    font_size: skrifa::instance::Size,

    font_ref: FontRef<'a>,
    var_loc: Location,
    metrics: Metrics,
    charmap: Charmap<'a>,
}

impl<'a> UiFont<'a> {
    pub fn new(font: &'a Font, size: f32, variations: &[(&'a str, f32)]) -> Option<Self> {
        let font_ref = font::to_font_ref(font)?;
        let font_size = skrifa::instance::Size::new(size);

        let axes = font_ref.axes();
        let var_loc = axes.location(variations.iter().copied());
        let metrics = font_ref.metrics(font_size, &var_loc);

        let charmap = font_ref.charmap();

        Some(Self {
            font,
            font_size,
            font_ref,
            var_loc,
            metrics,
            charmap,
        })
    }

    fn line_height(&self) -> f32 {
        self.metrics.ascent - self.metrics.descent + self.metrics.leading
    }

    fn charmap(&self) -> &Charmap<'a> {
        &self.charmap
    }

    fn glyph_metrics(&self) -> GlyphMetrics {
        self.font_ref.glyph_metrics(self.font_size, &self.var_loc)
    }

    pub fn font(&self) -> &Font {
        self.font
    }

    pub fn var_loc(&self) -> &Location {
        &self.var_loc
    }

    pub fn font_size_ppem(&self) -> f32 {
        self.font_size.ppem().expect("is ppem")
    }
}
