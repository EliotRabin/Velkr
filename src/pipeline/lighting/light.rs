use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::screen::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightType {
    Point(Vec3),
    Directional(Vec3),
    Spot(Vec3, Vec3, f64, f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    light_type: LightType,
    color: Color,
    intensity: f64,
}

impl Light {
    pub fn new(light_type: LightType, color: Color, intensity: f64) -> Self {
        Light { light_type, color, intensity }
    }

    pub fn point(position: Vec3, color: Color, intensity: f64) -> Self {
        Light::new(LightType::Point(position), color, intensity)
    }

    pub fn directional(direction: Vec3, color: Color, intensity: f64) -> Self {
        Light::new(LightType::Directional(direction), color, intensity)
    }

    pub fn spot(position: Vec3, direction: Vec3, inner_angle: f64, outer_angle: f64, color: Color, intensity: f64) -> Self {
        Light::new(LightType::Spot(position, direction, inner_angle, outer_angle), color, intensity)
    }

    pub fn light_type(&self) -> &LightType {
        &self.light_type
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn intensity(&self) -> f64 {
        self.intensity
    }

    pub fn set_light_type(&mut self, light_type: LightType) {
        self.light_type = light_type;
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_intensity(&mut self, intensity: f64) {
        self.intensity = intensity;
    }

    pub fn direction_from(&self, point: &Vec3) -> Vec3 {
        match self.light_type {
            LightType::Point(position) | LightType::Spot(position, ..) => (position - *point).normalize(),
            LightType::Directional(direction) => -direction.normalize(),
        }
    }

    pub fn distance_from(&self, point: &Vec3) -> f64 {
        match self.light_type {
            LightType::Point(position) | LightType::Spot(position, ..) => position.distance(point),
            LightType::Directional(_) => f64::INFINITY,
        }
    }

    pub fn intensity_at(&self, point: &Vec3) -> f64 {
        match self.light_type {
            LightType::Directional(_) => self.intensity,
            LightType::Point(position) => self.intensity * Self::attenuation(&position, point),
            LightType::Spot(position, direction, inner_angle, outer_angle) => {
                let to_point = (*point - position).normalize();
                let cosine = to_point.dot(&direction.normalize());

                self.intensity
                    * Self::attenuation(&position, point)
                    * Self::cone_falloff(cosine, inner_angle, outer_angle)
            }
        }
    }

    fn attenuation(position: &Vec3, point: &Vec3) -> f64 {
        let distance_squared = (*position - *point).length_squared();
        if distance_squared == 0.0 {
            1.0
        } else {
            1.0 / distance_squared
        }
    }

    fn cone_falloff(cosine: f64, inner_angle: f64, outer_angle: f64) -> f64 {
        let inner = inner_angle.to_radians().cos();
        let outer = outer_angle.to_radians().cos();

        if cosine >= inner {
            1.0
        } else if cosine <= outer {
            0.0
        } else {
            (cosine - outer) / (inner - outer)
        }
    }
}
