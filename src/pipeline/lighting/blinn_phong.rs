use crate::pipeline::lighting::lambert::Lambert;
use crate::pipeline::lighting::light::Light;
use crate::pipeline::lighting::shading_model::ShadingModel;
use crate::pipeline::lighting::surface::Surface;
use crate::pipeline::screen::color::Color;

pub struct BlinnPhong {
    diffuse: Lambert,
}

impl BlinnPhong {
    pub fn new() -> Self {
        BlinnPhong { diffuse: Lambert::new() }
    }
}

impl ShadingModel for BlinnPhong {
    fn shade(&self, surface: &Surface, light: &Light) -> Color {
        let diffuse = self.diffuse.shade(surface, light);

        let shininess = surface.shininess();
        if shininess <= 0.0 {
            return diffuse;
        }

        let point = surface.point();
        let direction = light.direction_from(&point);

        if surface.normal().dot(&direction) <= 0.0 {
            return diffuse;
        }

        let halfway = (direction + surface.view()).normalize();
        let specular = surface.normal().dot(&halfway).max(0.0).powf(shininess);

        diffuse + *light.color() * (specular * light.intensity_at(&point))
    }
}
