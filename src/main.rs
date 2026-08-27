mod pipeline;

use crate::pipeline::geometry::camera::{Camera, ProjectionType};
use crate::pipeline::geometry::model::Model;
use crate::pipeline::geometry::world::World;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::rasterization::framebuffer::Framebuffer;
use crate::pipeline::screen::window::Window;

fn main() {
    let cube: Model = Model::cube(1.0);
    let camera: Camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        16.0 / 9.0,
        ProjectionType::Perspective(45.0, 16.0 / 9.0, 0.1, 100.0),
    );

    let world = World::new(vec![cube], camera);
    let framebuffer = Framebuffer::new(1280, 720, world);
    let mut window = Window::new("RustRayTracer", framebuffer).expect("fenêtre impossible à créer");

    while window.is_open() {
        let framebuffer = window.framebuffer_mut();

        for model in framebuffer.world_mut().models_mut() {
            rotate_model(model);
        }

        framebuffer.clear([0, 0, 0]);
        framebuffer.rasterize_wireframe([255, 255, 255]);

        window.display().expect("affichage impossible");
    }
}

fn rotate_model(model: &mut Model) {
    model.set_rotation(*model.rotation() + Vec3::new(0.0174533, 0.0174533, 0.0174533));
}
