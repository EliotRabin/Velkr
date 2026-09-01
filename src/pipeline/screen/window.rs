use minifb::{Key, KeyRepeat, Window as MinifbWindow, WindowOptions};

use crate::pipeline::screen::framebuffer::Framebuffer;

pub struct Window {
    window: MinifbWindow,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Result<Self, minifb::Error> {
        let mut window = MinifbWindow::new(title, width, height, WindowOptions::default())?;
        window.set_target_fps(60);

        Ok(Window { window })
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.window.is_key_pressed(key, KeyRepeat::No)
    }

    pub fn display(&mut self, framebuffer: &Framebuffer) -> Result<(), minifb::Error> {
        let buffer: Vec<u32> = framebuffer
            .color_buffer()
            .iter()
            .map(|color| ((color.r() as u32) << 16) | ((color.g() as u32) << 8) | color.b() as u32)
            .collect();

        self.window
            .update_with_buffer(&buffer, framebuffer.width(), framebuffer.height())
    }
}
