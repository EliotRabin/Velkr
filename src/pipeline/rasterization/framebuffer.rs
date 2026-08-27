
use crate::pipeline::geometry::world::World;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::math::mat4::{Mat4, MatrixType};
use crate::pipeline::rasterization::viewport::Viewport;

pub struct Framebuffer {
    viewport: Viewport,
    color_buffer: Vec<[u8; 3]>,
    depth_buffer: Vec<f32>,
    world: World,
 }

 impl Framebuffer {
     pub fn new(width: usize, height: usize, world: World) -> Self {
         let color_buffer = vec![[0, 0, 0]; width * height];
         let depth_buffer = vec![f32::INFINITY; width * height];
        let viewport = Viewport::new(width, height);
         Framebuffer {
             viewport,
             color_buffer,
             depth_buffer,
             world,
         }
     }

     pub fn width(&self) -> usize {
        self.viewport.width()
     }

     pub fn height(&self) -> usize {
        self.viewport.height()
     }

     pub fn color_buffer(&self) -> &Vec<[u8; 3]> {
         &self.color_buffer
     }

     pub fn depth_buffer(&self) -> &Vec<f32> {
         &self.depth_buffer
     }

     pub fn clear(&mut self, color: [u8; 3]) {
         for pixel in self.color_buffer.iter_mut() {
             *pixel = color;
         }
         for depth in self.depth_buffer.iter_mut() {
             *depth = f32::INFINITY;
         }
     }

     pub fn world(&self) -> &World {
         &self.world
     }

     pub fn world_mut(&mut self) -> &mut World {
         &mut self.world
     }

     pub fn project_world(&self) -> Vec<Vec<Vec3>> {
         let camera = self.world.camera();

         let view_matrix = Mat4::from_matrix_type(MatrixType::ViewMatrix(camera));
         let projection_matrix = Mat4::from_matrix_type(MatrixType::ProjectionMatrix(camera));
         let viewport_matrix = Mat4::from_matrix_type(MatrixType::ViewportMatrix(&self.viewport));
         let view_projection_matrix = viewport_matrix * projection_matrix * view_matrix;

         let mut projected_models: Vec<Vec<Vec3>> = Vec::with_capacity(self.world.models().len());

         for model in self.world.models() {
             let model_matrix = Mat4::from_matrix_type(MatrixType::ModelMatrix(model));
             let mvp_matrix = view_projection_matrix * model_matrix;

             let mut projected_vertices: Vec<Vec3> = Vec::with_capacity(model.vertices().len());
             for vertex in model.vertices() {
                 projected_vertices.push(mvp_matrix * *vertex);
             }

             projected_models.push(projected_vertices);
         }

         projected_models
     }

     pub fn set_pixel(&mut self, x: isize, y: isize, color: [u8; 3]) {
         if x < 0 || y < 0 {
             return;
         }

         let x = x as usize;
         let y = y as usize;

         if x >= self.viewport.width() || y >= self.viewport.height() {
             return;
         }

         let index = y * self.viewport.width() + x;
         self.color_buffer[index] = color;
     }

     pub fn draw_line(&mut self, from: &Vec3, to: &Vec3, color: [u8; 3]) {
         let dx = to.x() - from.x();
         let dy = to.y() - from.y();
         let steps = dx.abs().max(dy.abs()).ceil() as usize;

         if steps == 0 {
             self.set_pixel(from.x().round() as isize, from.y().round() as isize, color);
             return;
         }

         let x_step = dx / steps as f64;
         let y_step = dy / steps as f64;

         for step in 0..=steps {
             let x = from.x() + x_step * step as f64;
             let y = from.y() + y_step * step as f64;
             self.set_pixel(x.round() as isize, y.round() as isize, color);
         }
     }

     pub fn rasterize_wireframe(&mut self, color: [u8; 3]) {
         let projected_models = self.project_world();

         for model_index in 0..projected_models.len() {
             let triangle_count = self.world.models()[model_index].indices().len();

             for triangle_index in 0..triangle_count {
                 let triangle = self.world.models()[model_index].indices()[triangle_index];
                 let projected_vertices = &projected_models[model_index];

                 for edge in 0..3 {
                     let from = projected_vertices[triangle[edge] as usize];
                     let to = projected_vertices[triangle[(edge + 1) % 3] as usize];
                     self.draw_line(&from, &to, color);
                 }
             }
         }
     }
}

