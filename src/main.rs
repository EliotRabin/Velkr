mod pipeline;

use crate::pipeline::application::app::App;
use crate::pipeline::geometry::camera::{Camera, ProjectionType};
use crate::pipeline::geometry::mesh::Mesh;
use crate::pipeline::geometry::model::Model;
use crate::pipeline::geometry::scene::Scene;
use crate::pipeline::geometry::world::World;
use crate::pipeline::lighting::lambert::Lambert;
use crate::pipeline::lighting::light::Light;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::raytracing::raytracer::Raytracer;
use crate::pipeline::screen::color::Color;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const SPACING: f64 = 1.7;
const DEPTH: u32 = 3;

fn main() {
    let meshes = build_meshes();
    let ground = Mesh::plane(24.0).with_color(Color::new(190, 190, 190)).with_reflectivity(0.35);
    let lights = build_lights();
    let world = build_world(&meshes, &ground, &lights);

    let raytracer = Raytracer::new(DEPTH, Box::new(Lambert::new()));

    let mut app = App::new("RustRayTracer", WIDTH, HEIGHT, world, raytracer).expect("fenêtre impossible à créer");
    app.run(Color::new(0, 0, 0)).expect("affichage impossible");
}

fn build_meshes() -> Vec<Mesh> {
    vec![
        Mesh::cube(1.0).with_color(Color::new(220, 70, 70)),
        Mesh::sphere(1.0, 12).with_color(Color::new(70, 180, 220)).with_reflectivity(0.5),
        Mesh::pyramid(1.0, 1.4).with_color(Color::new(230, 200, 90)),
        Mesh::cone(1.0, 1.4, 16).with_color(Color::new(120, 210, 120)),
        Mesh::cylinder(1.0, 1.4, 16).with_color(Color::new(200, 130, 230)),
    ]
}

fn build_lights() -> Vec<Light> {
    vec![
        Light::directional(Vec3::new(-0.4, -1.0, -0.6), Color::new(255, 244, 214), 1.0),
        Light::point(Vec3::new(0.0, 2.0, 4.0), Color::new(120, 160, 255), 12.0),
    ]
}

fn build_world<'a>(meshes: &'a [Mesh], ground: &'a Mesh, lights: &'a [Light]) -> World<'a> {
    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 7.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        aspect_ratio,
        ProjectionType::Perspective(45.0, aspect_ratio, 5.0, 1000.0),
    );

    let mut world = World::new(Scene::new(Vec::new()), Vec::new(), camera);

    for light in lights {
        world.add_light(light);
    }

    let count = meshes.len();
    for (index, mesh) in meshes.iter().enumerate() {
        let offset = index as f64 - (count - 1) as f64 / 2.0;

        let mut model = Model::from_mesh(mesh);
        model.set_location(Vec3::new(offset * SPACING, 0.0, 0.0));
        model.set_rotation(Vec3::new(0.0, index as f64 * 0.4, 0.0));
        world.add_model(model);
    }

    let mut floor = Model::from_mesh(ground);
    floor.set_location(Vec3::new(0.0, -1.0, 0.0));
    world.add_model(floor);

    world
}
