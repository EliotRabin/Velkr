mod pipeline;

use std::f64::consts::PI;

use crate::pipeline::geometry::camera::{Camera, ProjectionType};
use crate::pipeline::geometry::model::Model;
use crate::pipeline::geometry::world::World;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::rasterization::framebuffer::Framebuffer;
use crate::pipeline::screen::window::Window;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const SPACING: f64 = 1.7;

fn main() {
    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let camera: Camera = Camera::new(
        Vec3::new(0.0, 0.0, 7.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        aspect_ratio,
        ProjectionType::Perspective(45.0, aspect_ratio, 0.1, 100.0),
    );

    let mut world = World::new(Vec::new(), camera);

    let mut models = vec![
        Model::cube(1.0),
        Model::sphere(1.0, 5),
        Model::pyramid(1.0, 1.4),
        Model::cone(1.0, 1.4, 16),
        Model::cylinder(1.0, 1.4, 16),
    ];

    models[0].set_scale(Vec3::new(1.0, 1.5, 1.0));

    let count = models.len();
    for (index, mut model) in models.into_iter().enumerate() {
        let offset = index as f64 - (count - 1) as f64 / 2.0;
        model.set_location(Vec3::new(offset * SPACING, 0.0, 0.0));
        model.set_rotation(Vec3::new(0.0, index as f64 * 0.4, 0.0));
        world.add_model(model);
    }

    let framebuffer = Framebuffer::new(WIDTH, HEIGHT, world);
    let mut window = Window::new("RustRayTracer", framebuffer).expect("fenêtre impossible à créer");

    while window.is_open() {
        let framebuffer = window.framebuffer_mut();

        for model in framebuffer.world_mut().models_mut() {
            rotate_model(model);
        }

        framebuffer.clear([0, 0, 0]);
        framebuffer.rasterize_wireframe([255, 0, 0]);

        window.display().expect("affichage impossible");
    }
}

fn rotate_model(model: &mut Model) {
    let angle = PI / 360.0;
    model.set_rotation(*model.rotation() + Vec3::new(angle / 2.0, angle, 0.0));
}
