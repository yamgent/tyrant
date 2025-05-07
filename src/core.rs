use vello::{
    Scene,
    kurbo::{Affine, RoundedRect, Stroke},
    peniko::{Brush, Color, Fill, color::palette},
};

use crate::{
    font::DefaultFonts,
    ui_text::{UiBasicText, UiFont},
};

pub struct Core {
    pub default_fonts: DefaultFonts,

    // TODO: remove this dummy test code
    pub dummy: f64,
}

impl Core {
    pub fn render(&self, scene: &mut Scene) {
        let stroke = Stroke::new(self.dummy);
        let rect = RoundedRect::new(10.0, 10.0, 240.0, 240.0, 20.0);
        let rect_stroke_color = Color::new([0.9804, 0.702, 0.5294, 1.]);
        scene.stroke(&stroke, Affine::IDENTITY, rect_stroke_color, None, &rect);

        if let Some(ui_font) = UiFont::new(self.default_fonts.monospace(), 32.0, &[]) {
            if let Some(mut ui_text) = UiBasicText::new(ui_font) {
                ui_text.push_str("Hello world\nand bye!");
                ui_text.draw(
                    scene,
                    Affine::translate((40.0, 40.0)),
                    None,
                    &Brush::Solid(palette::css::WHITE),
                    Fill::NonZero,
                );
            }
        }
    }
}
