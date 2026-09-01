use crate::pipeline::lighting::light::Light;
use crate::pipeline::lighting::shading_model::ShadingModel;
use crate::pipeline::lighting::surface::Surface;
use crate::pipeline::screen::color::Color;

pub struct Lambert;

impl Lambert {
    pub fn new() -> Self {
        Lambert
    }
}

impl ShadingModel for Lambert {
    fn shade(&self, surface: &Surface, light: &Light) -> Color {
        let point = surface.point();
        let cosine = surface.normal().dot(&light.direction_from(&point));

        if cosine <= 0.0 {
            return Color::from_f64(0.0, 0.0, 0.0);
        }

        surface.albedo() * *light.color() * (cosine * light.intensity_at(&point))
    }
}
