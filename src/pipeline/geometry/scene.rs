use crate::pipeline::geometry::fragment::Fragment;
use crate::pipeline::geometry::model::Model;
use crate::pipeline::geometry::triangle::Triangle;

#[derive(Debug, Clone, PartialEq)]
pub struct Scene<'a> {
    models: Vec<Model<'a>>,
}

impl<'a> Scene<'a> {
    pub fn new(models: Vec<Model<'a>>) -> Self {
        Scene { models }
    }

    pub fn models(&self) -> &Vec<Model<'a>> {
        &self.models
    }

    pub fn models_mut(&mut self) -> &mut Vec<Model<'a>> {
        &mut self.models
    }

    pub fn add_model(&mut self, model: Model<'a>) {
        self.models.push(model);
    }

    pub fn remove_model(&mut self, index: usize) {
        if index < self.models.len() {
            self.models.remove(index);
        }
    }

    pub fn clear_models(&mut self) {
        self.models.clear();
    }

    pub fn world_vertices(&self) -> Vec<Vec<Fragment>> {
        self.models.iter().map(|model| model.world_vertices()).collect()
    }

    pub fn world_triangles<'b>(&self, vertices: &'b [Vec<Fragment>]) -> Vec<Triangle<'b>> {
        let mut triangles = Vec::new();

        for (model, buffer) in self.models.iter().zip(vertices.iter()) {
            triangles.extend(model.world_triangles(buffer));
        }

        triangles
    }
}
