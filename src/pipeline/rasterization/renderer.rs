use crate::pipeline::geometry::world::World;
use crate::pipeline::math::mat4::{Mat4, MatrixType};
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::rasterization::framebuffer::Framebuffer;

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Renderer
    }

    pub fn render(&self, world: &World, framebuffer: &mut Framebuffer, color: [u8; 3]) {
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

            for triangle in model.indices() {
                for edge in 0..3 {
                    let from = projected_vertices[triangle[edge] as usize];
                    let to = projected_vertices[triangle[(edge + 1) % 3] as usize];
                    self.draw_line(framebuffer, &from, &to, color);
                }
            }
        }
    }

    fn draw_line(&self, framebuffer: &mut Framebuffer, from: &Vec3, to: &Vec3, color: [u8; 3]) {
        let dx = to.x() - from.x();
        let dy = to.y() - from.y();
        let steps = dx.abs().max(dy.abs()).ceil() as usize;

        if steps == 0 {
            framebuffer.set_pixel(from.x().round() as isize, from.y().round() as isize, color);
            return;
        }

        let x_step = dx / steps as f64;
        let y_step = dy / steps as f64;

        for step in 0..=steps {
            let x = from.x() + x_step * step as f64;
            let y = from.y() + y_step * step as f64;
            framebuffer.set_pixel(x.round() as isize, y.round() as isize, color);
        }
    }
}
