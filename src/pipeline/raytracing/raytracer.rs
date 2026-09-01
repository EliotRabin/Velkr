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
        let vertices = world.world_vertices();
        let triangles = world.world_triangles(&vertices);

        for y in 0..framebuffer.height() {
            for x in 0..framebuffer.width() {
                let ray = camera.ray(x, y, framebuffer.viewport());

                if let Some(intersection) = Self::closest_intersection(&triangles, &ray) {
                    let color = Shading::lambert(
                        &intersection.point(),
                        &intersection.normal(),
                        intersection.surface(),
                        world.lights(),
                    );

                    framebuffer.set_pixel(x as isize, y as isize, intersection.distance() as f32, color);
                }
            }
        }
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
