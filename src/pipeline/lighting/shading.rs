use crate::pipeline::geometry::light::Light;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::screen::color::Color;

pub struct Shading;

impl Shading {
    pub fn lambert(point: &Vec3, normal: &Vec3, surface: Color, lights: &[&Light]) -> Color {
        let mut color = Color::from_f64(0.0, 0.0, 0.0);

        for light in lights {
            let lambert = normal.dot(&light.direction_from(point));

            if lambert <= 0.0 {
                continue;
            }

            color = color + surface * *light.color() * (lambert * light.intensity_at(point));
        }

        color
    }
}
