use crate::pipeline::geometry::triangle::Triangle;
use crate::pipeline::geometry::world::World;
use crate::pipeline::math::mat4::{Mat4, MatrixType};
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::rasterization::color::Color;
use crate::pipeline::rasterization::framebuffer::Framebuffer;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Renderer
    }

    pub fn render(&self, world: &World, framebuffer: &mut Framebuffer, color: Color) {
        let camera = world.camera();

        let view_matrix = Mat4::from_matrix_type(MatrixType::ViewMatrix(camera));
        let projection_matrix = Mat4::from_matrix_type(MatrixType::ProjectionMatrix(camera));
        let viewport_matrix = Mat4::from_matrix_type(MatrixType::ViewportMatrix(framebuffer.viewport()));
        let view_projection_matrix = viewport_matrix * projection_matrix * view_matrix;

        let mut projected_vertices: Vec<Vec3> = Vec::new();

        for model in world.models() {
            let model_matrix = Mat4::from_matrix_type(MatrixType::ModelMatrix(model));
            let mvp_matrix = view_projection_matrix * model_matrix;

            projected_vertices.clear();
            for vertex in model.vertices() {
                projected_vertices.push(mvp_matrix * *vertex);
            }

            for indices in model.indices() {
                let triangle = Triangle::new(
                    projected_vertices[indices[0] as usize],
                    projected_vertices[indices[1] as usize],
                    projected_vertices[indices[2] as usize],
                );
                self.draw_triangle(framebuffer, &triangle, color);
            }
        }
    }

    fn draw_line(&self, framebuffer: &mut Framebuffer, from: &Vec3, to: &Vec3, color: Color) {
        let dx = to.x() - from.x();
        let dy = to.y() - from.y();
        let steps = dx.abs().max(dy.abs()).ceil() as usize;

        if steps == 0 {
            framebuffer.set_pixel(
                from.x().round() as isize,
                from.y().round() as isize,
                from.z() as f32,
                color,
            );
            return;
        }

        let dz = to.z() - from.z();
        let x_step = dx / steps as f64;
        let y_step = dy / steps as f64;
        let z_step = dz / steps as f64;

        for step in 0..=steps {
            let x = from.x() + x_step * step as f64;
            let y = from.y() + y_step * step as f64;
            let z = from.z() + z_step * step as f64;
            framebuffer.set_pixel(x.round() as isize, y.round() as isize, z as f32, color);
        }
    }

    fn draw_triangle(&self, framebuffer: &mut Framebuffer, triangle: &Triangle, color: Color) {
        let area = triangle.signed_area();
        if area <= 0.0 {
            return;
        }

        let (min, max) = triangle.bounding_box();
        let min_x = min.x().floor().max(0.0) as isize;
        let max_x = max.x().ceil().min(framebuffer.width() as f64 - 1.0) as isize;
        let min_y = min.y().floor().max(0.0) as isize;
        let max_y = max.y().ceil().min(framebuffer.height() as f64 - 1.0) as isize;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec3::new(x as f64 + 0.5, y as f64 + 0.5, 0.0);
                let (w0, w1, w2) = triangle.edge_weights(&p);

                if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                    let alpha = w0 / area;
                    let beta = w1 / area;
                    let gamma = w2 / area;
                    let depth = triangle.interpolate_z(alpha, beta, gamma);

                    framebuffer.set_pixel(x, y, depth as f32, color);
                }
            }
        }
    }
}
