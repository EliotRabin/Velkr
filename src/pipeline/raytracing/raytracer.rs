use rayon::prelude::*;

use crate::pipeline::geometry::ray::Ray;
use crate::pipeline::geometry::triangle::Triangle;
use crate::pipeline::geometry::world::World;
use crate::pipeline::lighting::light::Light;
use crate::pipeline::lighting::shading_model::ShadingModel;
use crate::pipeline::lighting::surface::Surface;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::raytracing::intersection::Intersection;
use crate::pipeline::screen::color::Color;
use crate::pipeline::screen::framebuffer::Framebuffer;

const SHADOW_EPSILON: f64 = 1e-4;

pub struct Raytracer {
    depth: u32,
    shading: Box<dyn ShadingModel>,
}

impl Raytracer {
    pub fn new(depth: u32, shading: Box<dyn ShadingModel>) -> Self {
        Raytracer { depth, shading }
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn set_depth(&mut self, depth: u32) {
        self.depth = depth;
    }

    pub fn set_shading(&mut self, shading: Box<dyn ShadingModel>) {
        self.shading = shading;
    }

    pub fn render(&self, world: &World<'_>, framebuffer: &mut Framebuffer) {
        let camera = world.camera();
        let lights = world.lights();
        let vertices = world.world_vertices();
        let triangles = world.world_triangles(&vertices);

        let viewport = *framebuffer.viewport();
        let width = viewport.width();

        framebuffer
            .color_buffer_mut()
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                for (x, pixel) in row.iter_mut().enumerate() {
                    let ray = camera.ray(x, y, &viewport);

                    if let Some(intersection) = Self::closest_intersection(&triangles, &ray) {
                        *pixel = self.shade(&intersection, &triangles, lights, self.depth);
                    }
                }
            });
    }

    fn trace(&self, ray: &Ray, triangles: &[Triangle<'_>], lights: &[&Light], depth: u32) -> Color {
        match Self::closest_intersection(triangles, ray) {
            Some(intersection) => self.shade(&intersection, triangles, lights, depth),
            None => Color::from_f64(0.0, 0.0, 0.0),
        }
    }

    fn shade(&self, intersection: &Intersection<'_>, triangles: &[Triangle<'_>], lights: &[&Light], depth: u32) -> Color {
        let surface = Surface::new(
            intersection.point(),
            intersection.normal(),
            intersection.view(),
            intersection.albedo(),
            intersection.shininess(),
        );

        let mut color = Color::from_f64(0.0, 0.0, 0.0);

        for light in lights {
            if Self::occluded(triangles, &surface, light) {
                continue;
            }

            color = color + self.shading.shade(&surface, light);
        }

        let reflectivity = intersection.reflectivity();

        if depth == 0 || reflectivity <= 0.0 {
            return color;
        }

        let direction = intersection.ray().direction().reflect(&surface.normal());
        let reflected = Self::offset(surface.point(), surface.normal(), direction);

        color + self.trace(&reflected, triangles, lights, depth - 1) * reflectivity
    }

    fn occluded(triangles: &[Triangle<'_>], surface: &Surface, light: &Light) -> bool {
        let point = surface.point();
        let shadow = Self::offset(point, surface.normal(), light.direction_from(&point));
        let distance = light.distance_from(&point);

        triangles
            .iter()
            .any(|triangle| triangle.intersect(&shadow).is_some_and(|hit| hit.distance() < distance))
    }

    fn offset(point: Vec3, normal: Vec3, direction: Vec3) -> Ray {
        Ray::new(point, direction).shifted(normal * SHADOW_EPSILON)
    }

    fn closest_intersection<'a>(triangles: &[Triangle<'a>], ray: &Ray) -> Option<Intersection<'a>> {
        let mut closest: Option<Intersection<'a>> = None;

        for triangle in triangles {
            let Some(hit) = triangle.intersect(ray) else {
                continue;
            };

            let nearer = match closest {
                Some(best) => hit.distance() < best.distance(),
                None => true,
            };

            if nearer {
                closest = Some(Intersection::new(*triangle, hit, *ray));
            }
        }

        closest
    }
}
