use vello::{
    Scene,
    kurbo::{Affine, RoundedRect, Stroke},
    peniko::Color,
};

pub struct Core {
    // TODO: remove this dummy test code
    pub dummy: f64,
}

impl Core {
    pub fn render(&self, scene: &mut Scene) {
        let stroke = Stroke::new(self.dummy);
        let rect = RoundedRect::new(10.0, 10.0, 240.0, 240.0, 20.0);
        let rect_stroke_color = Color::new([0.9804, 0.702, 0.5294, 1.]);
        scene.stroke(&stroke, Affine::IDENTITY, rect_stroke_color, None, &rect);
    }
}
