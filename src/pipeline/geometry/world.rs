use crate::pipeline::geometry::camera::Camera;
use crate::pipeline::geometry::model::Model;

pub struct World {
    models: Vec<Model>,
    camera: Camera,
}

impl World {
    pub fn new(models: Vec<Model>, camera: Camera) -> Self {
        World { models, camera }
    }

    pub fn models(&self) -> &Vec<Model> {
        &self.models
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn models_mut(&mut self) -> &mut Vec<Model> {
        &mut self.models
    }

    pub fn add_model(&mut self, model: Model) {
        self.models.push(model);
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn remove_model(&mut self, index: usize) {
        if index < self.models.len() {
            self.models.remove(index);
        }
    }

   pub fn clear_models(&mut self) {
        self.models.clear();
    }

    
}