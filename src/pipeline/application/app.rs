use std::f64::consts::PI;

use minifb::Key;

use crate::pipeline::geometry::world::World;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::rasterization::color::Color;
use crate::pipeline::rasterization::framebuffer::Framebuffer;
use crate::pipeline::rasterization::renderer::Renderer;
use crate::pipeline::screen::window::Window;

pub struct App<'a> {
    world: World<'a>,
    framebuffer: Framebuffer,
    renderer: Renderer,
    window: Window,
    debug_depth: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &str, width: usize, height: usize, world: World<'a>) -> Result<Self, minifb::Error> {
        Ok(App {
            world,
            framebuffer: Framebuffer::new(width, height),
            renderer: Renderer::new(),
            window: Window::new(title, width, height)?,
            debug_depth: false,
        })
    }

    pub fn debug_depth(&self) -> bool {
        self.debug_depth
    }

    pub fn set_debug_depth(&mut self, debug_depth: bool) {
        self.debug_depth = debug_depth;
    }

    pub fn world(&self) -> &World<'a> {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World<'a> {
        &mut self.world
    }

    pub fn run(&mut self, clear_color: Color, wireframe_color: Color) -> Result<(), minifb::Error> {
        while self.window.is_open() {
            if self.window.is_key_pressed(Key::D) {
                self.debug_depth = !self.debug_depth;
            }

            self.update();

            self.framebuffer.clear(clear_color);
            self.renderer.render(&self.world, &mut self.framebuffer, wireframe_color);

            if self.debug_depth {
                self.framebuffer.draw_depth_buffer();
            }

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
