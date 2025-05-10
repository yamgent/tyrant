use vello::{
    Glyph, RenderParams, Renderer, Scene,
    kurbo::Affine,
    peniko::{BrushRef, StyleRef},
    wgpu::{Device, Queue, SurfaceTexture},
};

use crate::ui_text::UiFont;

/// A DPI-aware canvas, that draws to the scene
///
/// Use `RenderToCanvas` with a renderer to draw
/// the final scene to the window.
pub struct Canvas {
    scene: Scene,

    /// dpi
    scale_factor: f64,
}

impl Canvas {
    pub fn new(scale_factor: f64) -> Self {
        Self {
            scene: Scene::new(),
            scale_factor,
        }
    }

    pub fn reset(&mut self) {
        // instead of re-creating scene every frame, we just
        // reset the same scene to save memory allocation
        self.scene.reset();
    }

    pub fn update_scale_factor(&mut self, scale_factor: f64) {
        self.scale_factor = scale_factor;
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
            .font_size(ui_font.font_size_ppem() * self.scale_factor as f32)
            .transform(transform.then_scale(self.scale_factor))
            .glyph_transform(glyph_transform.map(|t| t.then_scale(self.scale_factor)))
            .normalized_coords(bytemuck::cast_slice(ui_font.var_loc().coords()))
            .brush(brush)
            .hint(false)
            .draw(
                style,
                glyphs.into_iter().map(|g| Glyph {
                    x: g.x * self.scale_factor as f32,
                    y: g.y * self.scale_factor as f32,
                    ..g
                }),
            );
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
