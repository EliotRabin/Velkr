use crate::pipeline::screen::color::Color;
use crate::pipeline::screen::viewport::Viewport;

pub struct Framebuffer {
    viewport: Viewport,
    color_buffer: Vec<Color>,
    depth_buffer: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let color_buffer = vec![Color::new(0, 0, 0); width * height];
        let depth_buffer = vec![f32::INFINITY; width * height];
        let viewport = Viewport::new(width, height);
        Framebuffer {
            viewport,
            color_buffer,
            depth_buffer,
        }
    }

    pub fn viewport(&self) -> &Viewport {
        &self.viewport
    }

    pub fn width(&self) -> usize {
        self.viewport.width()
    }

    pub fn height(&self) -> usize {
        self.viewport.height()
    }

    pub fn color_buffer(&self) -> &[Color] {
        &self.color_buffer
    }

    pub fn color_buffer_mut(&mut self) -> &mut [Color] {
        &mut self.color_buffer
    }

    pub fn depth_buffer(&self) -> &[f32] {
        &self.depth_buffer
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in self.color_buffer.iter_mut() {
            *pixel = color;
        }
        for depth in self.depth_buffer.iter_mut() {
            *depth = f32::INFINITY;
        }
    }

    pub fn set_pixel(&mut self, x: isize, y: isize, depth: f32, color: Color) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.viewport.width() || y >= self.viewport.height() {
            return;
        }

        let index = y * self.viewport.width() + x;

        if depth >= self.depth_buffer[index] {
            return;
        }

        self.depth_buffer[index] = depth;
        self.color_buffer[index] = color;
    }

}
