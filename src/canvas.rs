use vello::{
    Glyph, RenderParams, Renderer, Scene,
    kurbo::Affine,
    peniko::{BrushRef, StyleRef},
    wgpu::{Device, Queue, SurfaceTexture},
};

use crate::ui_text::UiFont;

pub struct Canvas {
    scene: Scene,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
        }
    }

    pub fn reset(&mut self) {
        // instead of re-creating scene every frame, we just
        // reset the same scene to save memory allocation
        self.scene.reset();
    }

    pub fn draw_text<'a>(
        &'a mut self,
        transform: Affine,
        glyph_transform: Option<Affine>,
        brush: impl Into<BrushRef<'a>>,
        style: impl Into<StyleRef<'a>>,
        ui_font: UiFont,
        glyphs: Vec<Glyph>,
    ) {
        self.scene
            .draw_glyphs(ui_font.font())
            .font_size(ui_font.font_size_ppem())
            .transform(transform)
            .glyph_transform(glyph_transform)
            .normalized_coords(bytemuck::cast_slice(ui_font.var_loc().coords()))
            .brush(brush)
            .hint(false)
            .draw(style, glyphs.into_iter());
    }
}

pub trait RenderToCanvas {
    fn render_to_canvas(
        &mut self,
        device: &Device,
        queue: &Queue,
        canvas: &Canvas,
        surface: &SurfaceTexture,
        params: &RenderParams,
    ) -> Result<(), vello::Error>;
}

impl RenderToCanvas for Renderer {
    fn render_to_canvas(
        &mut self,
        device: &Device,
        queue: &Queue,
        canvas: &Canvas,
        surface: &SurfaceTexture,
        params: &RenderParams,
    ) -> Result<(), vello::Error> {
        self.render_to_surface(device, queue, &canvas.scene, surface, params)
    }
}
