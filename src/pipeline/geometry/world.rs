use crate::pipeline::geometry::camera::Camera;
use crate::pipeline::geometry::fragment::Fragment;
use crate::pipeline::geometry::model::Model;
use crate::pipeline::geometry::scene::Scene;
use crate::pipeline::geometry::triangle::Triangle;
use crate::pipeline::lighting::light::Light;

pub struct World<'a> {
    scene: Scene<'a>,
    lights: Vec<&'a Light>,
    camera: Camera,
}

impl<'a> World<'a> {
    pub fn new(scene: Scene<'a>, lights: Vec<&'a Light>, camera: Camera) -> Self {
        World { scene, lights, camera }
    }

    pub fn scene(&self) -> &Scene<'a> {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene<'a> {
        &mut self.scene
    }

    pub fn models(&self) -> &Vec<Model<'a>> {
        self.scene.models()
    }

    pub fn models_mut(&mut self) -> &mut Vec<Model<'a>> {
        self.scene.models_mut()
    }

    pub fn lights(&self) -> &Vec<&'a Light> {
        &self.lights
    }

    pub fn lights_mut(&mut self) -> &mut Vec<&'a Light> {
        &mut self.lights
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn world_vertices(&self) -> Vec<Vec<Fragment>> {
        self.scene.world_vertices()
    }

    pub fn world_triangles<'b>(&self, vertices: &'b [Vec<Fragment>]) -> Vec<Triangle<'b>> {
        self.scene.world_triangles(vertices)
    }

    pub fn add_model(&mut self, model: Model<'a>) {
        self.scene.add_model(model);
    }

    pub fn add_light(&mut self, light: &'a Light) {
        self.lights.push(light);
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn remove_model(&mut self, index: usize) {
        self.scene.remove_model(index);
    }

    pub fn remove_light(&mut self, index: usize) {
        if index < self.lights.len() {
            self.lights.remove(index);
        }
    }

    pub fn clear_models(&mut self) {
        self.scene.clear_models();
    }

    pub fn clear_lights(&mut self) {
        self.lights.clear();
    }
}
