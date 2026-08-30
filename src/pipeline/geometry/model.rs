use std::f64::consts::PI;

use crate::pipeline::math::vec3::Vec3;

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    vertices: Vec<Vec3>,
    indices: Vec<[u64; 3]>,
    location: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

impl Model {
    pub fn new(vertices: Vec<Vec3>, indices: Vec<[u64; 3]>, location: Vec3, rotation: Vec3, scale: Vec3) -> Self {
        Model { vertices, indices, location, rotation, scale }
    }

    pub fn cube(size: f64) -> Self {
        let half_size = size / 2.0;
        let vertices = vec![
            Vec3::new(-half_size, -half_size, -half_size),
            Vec3::new(half_size, -half_size, -half_size),
            Vec3::new(half_size, half_size, -half_size),
            Vec3::new(-half_size, half_size, -half_size),
            Vec3::new(-half_size, -half_size, half_size),
            Vec3::new(half_size, -half_size, half_size),
            Vec3::new(half_size, half_size, half_size),
            Vec3::new(-half_size, half_size, half_size),
        ];

        let indices = vec![
            [0, 2, 1], [0, 3, 2],
            [4, 5, 6], [4, 6, 7],
            [0, 1, 5], [0, 5, 4],
            [2, 3, 7], [2, 7, 6],
            [0, 7, 3], [0, 4, 7],
            [1, 2, 6], [1, 6, 5],
        ];

        Model { vertices, indices, location: Vec3::new(0.0, 0.0, 0.0), rotation: Vec3::new(0.0, 0.0, 0.0), scale: Vec3::new(1.0, 1.0, 1.0) }
    }

    pub fn sphere(size: f64, detailing : u64) -> Self {
        let radius = size / 2.0;
        let rings = detailing.max(2);
        let segments = detailing.max(3);

        let mut vertices = Vec::with_capacity(((rings + 1) * (segments + 1)) as usize);
        for ring in 0..=rings {
            let phi = PI * ring as f64 / rings as f64;
            for segment in 0..=segments {
                let theta = 2.0 * PI * segment as f64 / segments as f64;
                vertices.push(Vec3::new(
                    radius * phi.sin() * theta.cos(),
                    radius * phi.cos(),
                    radius * phi.sin() * theta.sin(),
                ));
            }
        }

        let mut indices = Vec::with_capacity((2 * rings * segments) as usize);
        for ring in 0..rings {
            for segment in 0..segments {
                let current = ring * (segments + 1) + segment;
                let next = current + segments + 1;

                if ring != 0 {
                    indices.push([current, current + 1, next]);
                }
                if ring != rings - 1 {
                    indices.push([current + 1, next + 1, next]);
                }
            }
        }

        Model::new(vertices, indices, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0))
    }

    pub fn pyramid(size: f64, height: f64) -> Self {
        let half_size = size / 2.0;
        let half_height = height / 2.0;

        let vertices = vec![
            Vec3::new(-half_size, -half_height, -half_size),
            Vec3::new(half_size, -half_height, -half_size),
            Vec3::new(half_size, -half_height, half_size),
            Vec3::new(-half_size, -half_height, half_size),
            Vec3::new(0.0, half_height, 0.0), // apex
        ];

        let indices = vec![
            [0, 1, 2], [0, 2, 3],
            [0, 4, 1],
            [1, 4, 2],
            [2, 4, 3],
            [3, 4, 0],
        ];

        Model::new(vertices, indices, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0))
    }

    pub fn cone(size: f64, height: f64, detailing: u64) -> Self {
        let radius = size / 2.0;
        let half_height = height / 2.0;
        let segments = detailing.max(3);

        let mut vertices = Vec::with_capacity(segments as usize + 2);
        vertices.push(Vec3::new(0.0, half_height, 0.0)); // apex
        vertices.push(Vec3::new(0.0, -half_height, 0.0)); // base center

        for segment in 0..segments {
            let theta = 2.0 * PI * segment as f64 / segments as f64;
            vertices.push(Vec3::new(radius * theta.cos(), -half_height, radius * theta.sin()));
        }

        let mut indices = Vec::with_capacity((2 * segments) as usize);
        for segment in 0..segments {
            let current = 2 + segment;
            let next = 2 + (segment + 1) % segments;

            indices.push([0, next, current]); // side
            indices.push([1, current, next]); // base
        }

        Model::new(vertices, indices, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0))
    }

    pub fn cylinder(size: f64, height: f64, detailing: u64) -> Self {
        let radius = size / 2.0;
        let half_height = height / 2.0;
        let segments = detailing.max(3);

        let mut vertices = Vec::with_capacity(2 * segments as usize + 2);
        vertices.push(Vec3::new(0.0, -half_height, 0.0)); // bottom center
        vertices.push(Vec3::new(0.0, half_height, 0.0)); // top center

        for segment in 0..segments {
            let theta = 2.0 * PI * segment as f64 / segments as f64;
            vertices.push(Vec3::new(radius * theta.cos(), -half_height, radius * theta.sin()));
        }

        for segment in 0..segments {
            let theta = 2.0 * PI * segment as f64 / segments as f64;
            vertices.push(Vec3::new(radius * theta.cos(), half_height, radius * theta.sin()));
        }

        let mut indices = Vec::with_capacity((4 * segments) as usize);
        for segment in 0..segments {
            let bottom = 2 + segment;
            let bottom_next = 2 + (segment + 1) % segments;
            let top = 2 + segments + segment;
            let top_next = 2 + segments + (segment + 1) % segments;

            indices.push([bottom, top, bottom_next]); // side
            indices.push([bottom_next, top, top_next]); // side
            indices.push([0, bottom, bottom_next]); // bottom cap
            indices.push([1, top_next, top]); // top cap
        }

        Model::new(vertices, indices, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0))
    }

    pub fn plane(size: f64) -> Self {
        let mut vertices = vec![
            Vec3::new(-size/2.0, 0.0, -size/2.0),
            Vec3::new(-size/2.0, 0.0, size/2.0),
            Vec3::new(size/2.0, 0.0, -size/2.0),
            Vec3::new(size/2.0, 0.0, size/2.0),
        ];

        let indices =  vec![[0, 1, 2], [1, 3, 2]];

        Model::new(vertices, indices, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0))
    }

    pub fn vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<[u64; 3]> {
        &self.indices
    }

    pub fn location(&self) -> &Vec3 {
        &self.location
    }

    pub fn rotation(&self) -> &Vec3 {
        &self.rotation
    }

    pub fn scale(&self) -> &Vec3 {
        &self.scale
    }

    pub fn set_location(&mut self, location: Vec3) {
        self.location = location;
    }

    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

}

