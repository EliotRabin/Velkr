use rayon::prelude::*;

use crate::pipeline::geometry::ray::Ray;
use crate::pipeline::geometry::triangle::Triangle;
use crate::pipeline::geometry::world::World;
use crate::pipeline::lighting::shading::Shading;
use crate::pipeline::raytracing::intersection::Intersection;
use crate::pipeline::screen::framebuffer::Framebuffer;

pub struct Raytracer;

impl Raytracer {
    pub fn new() -> Self {
        Raytracer
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
                        *pixel = Shading::lambert(
                            &intersection.point(),
                            &intersection.normal(),
                            intersection.surface(),
                            lights,
                        );
                    }
                }
            });
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
