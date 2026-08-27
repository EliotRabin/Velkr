use minifb::{Key, Window as MinifbWindow, WindowOptions};

use crate::pipeline::rasterization::framebuffer::Framebuffer;

pub struct Window {
    window: MinifbWindow,
    framebuffer: Framebuffer,
}

impl Window {
    pub fn new(title: &str, framebuffer: Framebuffer) -> Result<Self, minifb::Error> {
        let mut window = MinifbWindow::new(
            title,
            framebuffer.width(),
            framebuffer.height(),
            WindowOptions::default(),
        )?;
        window.set_target_fps(60);

        Ok(Window { window, framebuffer })
    }

    pub fn framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }

    pub fn framebuffer_mut(&mut self) -> &mut Framebuffer {
        &mut self.framebuffer
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn display(&mut self) -> Result<(), minifb::Error> {
        let buffer: Vec<u32> = self
            .framebuffer
            .color_buffer()
            .iter()
            .map(|color| ((color[0] as u32) << 16) | ((color[1] as u32) << 8) | color[2] as u32)
            .collect();

        self.window.update_with_buffer(
            &buffer,
            self.framebuffer.width(),
            self.framebuffer.height(),
        )
    }
}
