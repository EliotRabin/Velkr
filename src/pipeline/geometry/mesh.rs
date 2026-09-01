use std::f64::consts::PI;

use crate::pipeline::geometry::fragment::{Attribut, AttributKind, Fragment};
use crate::pipeline::geometry::triangle::Triangle;
use crate::pipeline::math::mat4::Mat4;
use crate::pipeline::math::vec3::Vec3;
use crate::pipeline::screen::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    vertices: Vec<Fragment>,
    indices: Vec<[u64; 3]>,
}

impl Mesh {
    pub fn new(vertices: Vec<Fragment>, indices: Vec<[u64; 3]>) -> Self {
        Mesh { vertices, indices }
    }

    pub fn from_positions(positions: Vec<Vec3>, indices: Vec<[u64; 3]>) -> Self {
        let vertices = positions.into_iter().map(Fragment::from_position).collect();
        Mesh::new(vertices, indices)
    }

    pub fn cube(size: f64) -> Self {
        let half_size = size / 2.0;
        let positions = vec![
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

        Mesh::from_positions(positions, indices)
    }

    pub fn sphere(size: f64, detailing : u64) -> Self {
        let radius = size / 2.0;
        let rings = detailing.max(2);
        let segments = detailing.max(3);

        let mut positions = Vec::with_capacity(((rings + 1) * (segments + 1)) as usize);
        for ring in 0..=rings {
            let phi = PI * ring as f64 / rings as f64;
            for segment in 0..=segments {
                let theta = 2.0 * PI * segment as f64 / segments as f64;
                positions.push(Vec3::new(
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

        Mesh::from_positions(positions, indices)
    }

    pub fn pyramid(size: f64, height: f64) -> Self {
        let half_size = size / 2.0;
        let half_height = height / 2.0;

        let positions = vec![
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

        Mesh::from_positions(positions, indices)
    }

    pub fn cone(size: f64, height: f64, detailing: u64) -> Self {
        let radius = size / 2.0;
        let half_height = height / 2.0;
        let segments = detailing.max(3);

        let mut positions = Vec::with_capacity(segments as usize + 2);
        positions.push(Vec3::new(0.0, half_height, 0.0)); // apex
        positions.push(Vec3::new(0.0, -half_height, 0.0)); // base center

        for segment in 0..segments {
            let theta = 2.0 * PI * segment as f64 / segments as f64;
            positions.push(Vec3::new(radius * theta.cos(), -half_height, radius * theta.sin()));
        }

        let mut indices = Vec::with_capacity((2 * segments) as usize);
        for segment in 0..segments {
            let current = 2 + segment;
            let next = 2 + (segment + 1) % segments;

            indices.push([0, next, current]); // side
            indices.push([1, current, next]); // base
        }

        Mesh::from_positions(positions, indices)
    }

    pub fn cylinder(size: f64, height: f64, detailing: u64) -> Self {
        let radius = size / 2.0;
        let half_height = height / 2.0;
        let segments = detailing.max(3);

        let mut positions = Vec::with_capacity(2 * segments as usize + 2);
        positions.push(Vec3::new(0.0, -half_height, 0.0)); // bottom center
        positions.push(Vec3::new(0.0, half_height, 0.0)); // top center

        for segment in 0..segments {
            let theta = 2.0 * PI * segment as f64 / segments as f64;
            positions.push(Vec3::new(radius * theta.cos(), -half_height, radius * theta.sin()));
        }

        for segment in 0..segments {
            let theta = 2.0 * PI * segment as f64 / segments as f64;
            positions.push(Vec3::new(radius * theta.cos(), half_height, radius * theta.sin()));
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

        Mesh::from_positions(positions, indices)
    }

    pub fn plane(size: f64) -> Self {
        let positions = vec![
            Vec3::new(-size/2.0, 0.0, -size/2.0),
            Vec3::new(-size/2.0, 0.0, size/2.0),
            Vec3::new(size/2.0, 0.0, -size/2.0),
            Vec3::new(size/2.0, 0.0, size/2.0),
        ];

        let indices =  vec![[0, 1, 2], [1, 3, 2]];

        Mesh::from_positions(positions, indices)
    }

    pub fn with_color(mut self, color: Color) -> Self {
        for vertex in self.vertices.iter_mut() {
            vertex.set_attribut(Attribut::color(AttributKind::Color, color));
        }

        self
    }

    pub fn with_reflectivity(mut self, reflectivity: f64) -> Self {
        for vertex in self.vertices.iter_mut() {
            vertex.set_attribut(Attribut::scalar(AttributKind::Reflectivity, reflectivity));
        }

        self
    }

    pub fn vertices(&self) -> &Vec<Fragment> {
        &self.vertices
    }

    pub fn indices(&self) -> &Vec<[u64; 3]> {
        &self.indices
    }

    pub fn triangles(&self) -> Vec<Triangle<'_>> {
        self.triangles_from(&self.vertices)
    }

    pub fn triangles_from<'a>(&self, vertices: &'a [Fragment]) -> Vec<Triangle<'a>> {
        self.indices
            .iter()
            .map(|index| {
                Triangle::new(
                    &vertices[index[0] as usize],
                    &vertices[index[1] as usize],
                    &vertices[index[2] as usize],
                )
            })
            .collect()
    }

    pub fn transformed_vertices(&self, matrix: &Mat4) -> Vec<Fragment> {
        self.vertices.iter().map(|vertex| vertex.transformed(matrix)).collect()
    }
}
