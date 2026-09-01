mod pipeline;

use crate::pipeline::application::app::App;
use crate::pipeline::geometry::camera::{Camera, ProjectionType};
use crate::pipeline::geometry::mesh::Mesh;
use crate::pipeline::geometry::model::Model;
use crate::pipeline::geometry::world::World;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::rasterization::color::Color;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const SPACING: f64 = 1.7;

fn main() {
    let meshes = build_meshes();
    let world = build_world(&meshes);

    let mut app = App::new("RustRayTracer", WIDTH, HEIGHT, world).expect("fenêtre impossible à créer");
    app.run(Color::new(0, 0, 0), Color::new(255, 0, 0)).expect("affichage impossible");
}

fn build_meshes() -> Vec<Mesh> {
    vec![
        Mesh::cube(1.0),
        Mesh::sphere(1.0, 12),
        Mesh::pyramid(1.0, 1.4),
        Mesh::cone(1.0, 1.4, 16),
        Mesh::cylinder(1.0, 1.4, 16),
    ]
}

fn build_world(meshes: &[Mesh]) -> World<'_> {
    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 7.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        aspect_ratio,
        ProjectionType::Perspective(45.0, aspect_ratio, 5.0, 1000.0),
    );

    let mut world = World::new(Vec::new(), camera);

    let count = meshes.len();
    for (index, mesh) in meshes.iter().enumerate() {
        let offset = index as f64 - (count - 1) as f64 / 2.0;

        let mut model = Model::from_mesh(mesh);
        model.set_location(Vec3::new(offset * SPACING, 0.0, 0.0));
        model.set_rotation(Vec3::new(0.0, index as f64 * 0.4, 0.0));
        world.add_model(model);
    }

    world
}
