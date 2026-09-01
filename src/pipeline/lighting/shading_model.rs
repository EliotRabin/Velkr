use crate::pipeline::lighting::light::Light;
use crate::pipeline::lighting::surface::Surface;
use crate::pipeline::screen::color::Color;

pub trait ShadingModel: Send + Sync {
    fn shade(&self, surface: &Surface, light: &Light) -> Color;
}
