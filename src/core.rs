use vello::{
    kurbo::Affine,
    peniko::{Brush, Fill, color::palette},
};

use crate::{
    canvas::Canvas,
    command_bar::CommandBar,
    font::DefaultFonts,
    ui_text::{UiBasicText, UiFont},
};

pub struct Core {
    pub default_fonts: DefaultFonts,

    pub command_bar: CommandBar,
}

impl Core {
    pub fn new() -> Self {
        Self {
            default_fonts: DefaultFonts::new(),
            command_bar: CommandBar::new(),
        }
    }

    pub fn render(&self, canvas: &mut Canvas) {
        let mode = if self.command_bar.active() {
            "COMMAND"
        } else {
            "NORMAL"
        };

        if let Some(ui_font) = UiFont::new(self.default_fonts.monospace(), 32.0, &[]) {
            if let Some(mut ui_text) = UiBasicText::new(ui_font) {
                ui_text.push_str(mode);
                ui_text.draw(
                    canvas,
                    Affine::translate((40.0, 40.0)),
                    None,
                    &Brush::Solid(palette::css::WHITE),
                    Fill::NonZero,
                );
            }
        }
    }
}
