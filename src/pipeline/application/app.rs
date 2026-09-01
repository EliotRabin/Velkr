use std::f64::consts::PI;

use crate::pipeline::geometry::world::World;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::raytracing::raytracer::Raytracer;
use crate::pipeline::screen::color::Color;
use crate::pipeline::screen::framebuffer::Framebuffer;
use crate::pipeline::screen::window::Window;

pub struct App<'a> {
    world: World<'a>,
    framebuffer: Framebuffer,
    raytracer: Raytracer,
    window: Window,
}

impl<'a> App<'a> {
    pub fn new(title: &str, width: usize, height: usize, world: World<'a>) -> Result<Self, minifb::Error> {
        Ok(App {
            world,
            framebuffer: Framebuffer::new(width, height),
            raytracer: Raytracer::new(),
            window: Window::new(title, width, height)?,
        })
    }

    pub fn world(&self) -> &World<'a> {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World<'a> {
        &mut self.world
    }

    pub fn run(&mut self, clear_color: Color) -> Result<(), minifb::Error> {
        while self.window.is_open() {
            self.update();

            self.framebuffer.clear(clear_color);
            self.raytracer.render(&self.world, &mut self.framebuffer);

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
