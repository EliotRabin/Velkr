use std::f64::consts::PI;

use crate::pipeline::geometry::world::World;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::rasterization::framebuffer::Framebuffer;
use crate::pipeline::rasterization::renderer::Renderer;
use crate::pipeline::screen::window::Window;

pub struct App {
    world: World,
    framebuffer: Framebuffer,
    renderer: Renderer,
    window: Window,
}

impl App {
    pub fn new(title: &str, width: usize, height: usize, world: World) -> Result<Self, minifb::Error> {
        Ok(App {
            world,
            framebuffer: Framebuffer::new(width, height),
            renderer: Renderer::new(),
            window: Window::new(title, width, height)?,
        })
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn run(&mut self, clear_color: [u8; 3], wireframe_color: [u8; 3]) -> Result<(), minifb::Error> {
        while self.window.is_open() {
            self.update();

            self.framebuffer.clear(clear_color);
            self.renderer.render(&self.world, &mut self.framebuffer, wireframe_color);

            self.window.display(&self.framebuffer)?;
        }

        Ok(())
    }

    fn update(&mut self) {
        let angle = PI / 360.0;
        for model in self.world.models_mut() {
            model.set_rotation(*model.rotation() + Vec3::new(angle / 2.0, angle, 0.0));
        }
    }
}
