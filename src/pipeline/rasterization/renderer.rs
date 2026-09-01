use crate::pipeline::geometry::fragment::Fragment;
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

        let mut projected_vertices: Vec<Fragment> = Vec::new();

        for model in world.models() {
            let model_matrix = Mat4::from_matrix_type(MatrixType::ModelMatrix(model));
            let mvp_matrix = view_projection_matrix * model_matrix;

            projected_vertices.clear();
            for vertex in model.vertices() {
                projected_vertices.push(Fragment::from_position(mvp_matrix * *vertex));
            }

            for indices in model.indices() {
                let triangle = Triangle::new(
                    projected_vertices[indices[0] as usize].clone(),
                    projected_vertices[indices[1] as usize].clone(),
                    projected_vertices[indices[2] as usize].clone(),
                );
                self.draw_triangle(framebuffer, &triangle, color);
            }
        }
    }

    fn pixel_center(x: isize, y: isize) -> Vec3 {
        Vec3::new(x as f64 + 0.5, y as f64 + 0.5, 0.0)
    }

    fn plot(framebuffer: &mut Framebuffer, point: &Vec3, color: Color) {
        framebuffer.set_pixel(
            point.x().round() as isize,
            point.y().round() as isize,
            point.z() as f32,
            color,
        );
    }

    fn draw_line(&self, framebuffer: &mut Framebuffer, from: &Vec3, to: &Vec3, color: Color) {
        let delta = *to - *from;
        let steps = delta.x().abs().max(delta.y().abs()).ceil() as usize;

        if steps == 0 {
            Self::plot(framebuffer, from, color);
            return;
        }

        for step in 0..=steps {
            let point = from.lerp(to, step as f64 / steps as f64);
            Self::plot(framebuffer, &point, color);
        }
    }

    fn draw_triangle(&self, framebuffer: &mut Framebuffer, triangle: &Triangle, color: Color) {
        if triangle.signed_area() <= 0.0 {
            return;
        }

        let (min, max) = triangle.bounding_box();
        let min_x = min.x().floor().max(0.0) as isize;
        let max_x = max.x().ceil().min(framebuffer.width() as f64 - 1.0) as isize;
        let min_y = min.y().floor().max(0.0) as isize;
        let max_y = max.y().ceil().min(framebuffer.height() as f64 - 1.0) as isize;

        let mut fragment = Fragment::new();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Self::pixel_center(x, y);

                if let Some((alpha, beta, gamma)) = triangle.barycentric(&p) {
                    fragment.interpolate_from(
                        triangle.v0(),
                        triangle.v1(),
                        triangle.v2(),
                        alpha,
                        beta,
                        gamma,
                    );

                    framebuffer.set_pixel(x, y, fragment.depth() as f32, color);
                }
            }
        }
    }
}
