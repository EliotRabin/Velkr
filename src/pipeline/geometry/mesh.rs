use std::f64::consts::PI;

use crate::pipeline::geometry::fragment::{Attribut, AttributKind, Fragment};
use crate::pipeline::geometry::triangle::Triangle;
use crate::pipeline::math::mat4::Mat4;
use crate::pipeline::math::vec2::Vec2;
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

    pub fn from_positions_and_uvs(positions: Vec<Vec3>, uvs: Vec<Vec2>, indices: Vec<[u64; 3]>) -> Self {
        debug_assert_eq!(positions.len(), uvs.len(), "chaque position doit porter une uv");

        let vertices = positions
            .into_iter()
            .zip(uvs)
            .map(|(position, uv)| Fragment::from_position_uv(position, uv))
            .collect();

        Mesh::new(vertices, indices)
    }

    pub fn cube(size: f64) -> Self {
        let half_size = size / 2.0;
        let corners = [
            Vec3::new(-half_size, -half_size, -half_size),
            Vec3::new(half_size, -half_size, -half_size),
            Vec3::new(half_size, half_size, -half_size),
            Vec3::new(-half_size, half_size, -half_size),
            Vec3::new(-half_size, -half_size, half_size),
            Vec3::new(half_size, -half_size, half_size),
            Vec3::new(half_size, half_size, half_size),
            Vec3::new(-half_size, half_size, half_size),
        ];

        // un coin est partagé par trois faces, donc dédoublé pour porter une uv par face
        let faces = [
            [0, 3, 2, 1],
            [4, 5, 6, 7],
            [0, 1, 5, 4],
            [2, 3, 7, 6],
            [0, 4, 7, 3],
            [1, 2, 6, 5],
        ];

        let mut positions = Vec::with_capacity(4 * faces.len());
        let mut uvs = Vec::with_capacity(4 * faces.len());
        let mut indices = Vec::with_capacity(2 * faces.len());

        for face in faces {
            Self::push_quad(
                &mut positions,
                &mut uvs,
                &mut indices,
                face.map(|corner| corners[corner]),
            );
        }

        Mesh::from_positions_and_uvs(positions, uvs, indices)
    }

    pub fn sphere(size: f64, detailing : u64) -> Self {
        let radius = size / 2.0;
        let rings = detailing.max(2);
        let segments = detailing.max(3);

        let mut positions = Vec::with_capacity(((rings + 1) * (segments + 1)) as usize);
        let mut uvs = Vec::with_capacity(((rings + 1) * (segments + 1)) as usize);

        // la colonne de couture est déjà dupliquée : u vaut 0.0 à gauche et 1.0 à droite
        for ring in 0..=rings {
            let v = ring as f64 / rings as f64;
            let phi = PI * v;

            for segment in 0..=segments {
                let u = segment as f64 / segments as f64;
                let theta = 2.0 * PI * u;

                positions.push(Vec3::new(
                    radius * phi.sin() * theta.cos(),
                    radius * phi.cos(),
                    radius * phi.sin() * theta.sin(),
                ));
                uvs.push(Vec2::new(u, v));
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

        Mesh::from_positions_and_uvs(positions, uvs, indices)
    }

    pub fn pyramid(size: f64, height: f64) -> Self {
        let half_size = size / 2.0;
        let half_height = height / 2.0;

        let base = [
            Vec3::new(-half_size, -half_height, -half_size),
            Vec3::new(half_size, -half_height, -half_size),
            Vec3::new(half_size, -half_height, half_size),
            Vec3::new(-half_size, -half_height, half_size),
        ];
        let apex = Vec3::new(0.0, half_height, 0.0);

        let mut positions = Vec::with_capacity(4 + 3 * base.len());
        let mut uvs = Vec::with_capacity(4 + 3 * base.len());
        let mut indices = Vec::with_capacity(2 + base.len());

        Self::push_quad(&mut positions, &mut uvs, &mut indices, base);

        // l'apex est dédoublé sur chaque face latérale, sinon il ne peut porter qu'une seule uv
        for corner in 0..base.len() {
            let next = (corner + 1) % base.len();
            let side = positions.len() as u64;

            positions.extend([base[corner], apex, base[next]]);
            uvs.extend([
                Vec2::new(0.0, 1.0),
                Vec2::new(0.5, 0.0),
                Vec2::new(1.0, 1.0),
            ]);
            indices.push([side, side + 1, side + 2]);
        }

        Mesh::from_positions_and_uvs(positions, uvs, indices)
    }

    pub fn cone(size: f64, height: f64, detailing: u64) -> Self {
        let radius = size / 2.0;
        let half_height = height / 2.0;
        let segments = detailing.max(3);

        let apex = Vec3::new(0.0, half_height, 0.0);
        let center = Vec3::new(0.0, -half_height, 0.0);

        let mut positions = Vec::with_capacity(6 * segments as usize);
        let mut uvs = Vec::with_capacity(6 * segments as usize);
        let mut indices = Vec::with_capacity(2 * segments as usize);

        // chaque secteur est indépendant : le flanc se déroule en bande, la base se projette en disque
        for segment in 0..segments {
            let u = segment as f64 / segments as f64;
            let next_u = (segment + 1) as f64 / segments as f64;

            let theta = 2.0 * PI * u;
            let next_theta = 2.0 * PI * next_u;

            let current = Vec3::new(radius * theta.cos(), -half_height, radius * theta.sin());
            let next = Vec3::new(radius * next_theta.cos(), -half_height, radius * next_theta.sin());

            let side = positions.len() as u64;
            positions.extend([apex, next, current]);
            uvs.extend([
                Vec2::new((u + next_u) / 2.0, 0.0),
                Vec2::new(next_u, 1.0),
                Vec2::new(u, 1.0),
            ]);
            indices.push([side, side + 1, side + 2]);

            let base = positions.len() as u64;
            positions.extend([center, current, next]);
            uvs.extend([
                Vec2::new(0.5, 0.5),
                Self::disc_uv(theta),
                Self::disc_uv(next_theta),
            ]);
            indices.push([base, base + 1, base + 2]);
        }

        Mesh::from_positions_and_uvs(positions, uvs, indices)
    }

    pub fn cylinder(size: f64, height: f64, detailing: u64) -> Self {
        let radius = size / 2.0;
        let half_height = height / 2.0;
        let segments = detailing.max(3);

        let bottom_center = Vec3::new(0.0, -half_height, 0.0);
        let top_center = Vec3::new(0.0, half_height, 0.0);

        let mut positions = Vec::with_capacity(10 * segments as usize);
        let mut uvs = Vec::with_capacity(10 * segments as usize);
        let mut indices = Vec::with_capacity(4 * segments as usize);

        // le flanc et les deux capuchons ne partagent plus les sommets de couronne
        for segment in 0..segments {
            let u = segment as f64 / segments as f64;
            let next_u = (segment + 1) as f64 / segments as f64;

            let theta = 2.0 * PI * u;
            let next_theta = 2.0 * PI * next_u;

            let bottom = Vec3::new(radius * theta.cos(), -half_height, radius * theta.sin());
            let bottom_next = Vec3::new(radius * next_theta.cos(), -half_height, radius * next_theta.sin());
            let top = Vec3::new(radius * theta.cos(), half_height, radius * theta.sin());
            let top_next = Vec3::new(radius * next_theta.cos(), half_height, radius * next_theta.sin());

            let side = positions.len() as u64;
            positions.extend([bottom, top, bottom_next, top_next]);
            uvs.extend([
                Vec2::new(u, 1.0),
                Vec2::new(u, 0.0),
                Vec2::new(next_u, 1.0),
                Vec2::new(next_u, 0.0),
            ]);
            indices.push([side, side + 1, side + 2]);
            indices.push([side + 2, side + 1, side + 3]);

            let bottom_cap = positions.len() as u64;
            positions.extend([bottom_center, bottom, bottom_next]);
            uvs.extend([
                Vec2::new(0.5, 0.5),
                Self::disc_uv(theta),
                Self::disc_uv(next_theta),
            ]);
            indices.push([bottom_cap, bottom_cap + 1, bottom_cap + 2]);

            let top_cap = positions.len() as u64;
            positions.extend([top_center, top_next, top]);
            uvs.extend([
                Vec2::new(0.5, 0.5),
                Self::disc_uv(next_theta),
                Self::disc_uv(theta),
            ]);
            indices.push([top_cap, top_cap + 1, top_cap + 2]);
        }

        Mesh::from_positions_and_uvs(positions, uvs, indices)
    }

    pub fn plane(size: f64) -> Self {
        let half_size = size / 2.0;

        let positions = vec![
            Vec3::new(-half_size, 0.0, -half_size),
            Vec3::new(-half_size, 0.0, half_size),
            Vec3::new(half_size, 0.0, -half_size),
            Vec3::new(half_size, 0.0, half_size),
        ];

        let uvs = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
        ];

        let indices = vec![[0, 1, 2], [1, 3, 2]];

        Mesh::from_positions_and_uvs(positions, uvs, indices)
    }

    // uv d'un quad parcouru dans l'ordre, v croissant vers le bas comme les lignes d'une image
    fn push_quad(
        positions: &mut Vec<Vec3>,
        uvs: &mut Vec<Vec2>,
        indices: &mut Vec<[u64; 3]>,
        corners: [Vec3; 4],
    ) {
        let base = positions.len() as u64;

        positions.extend(corners);
        uvs.extend([
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ]);

        indices.push([base, base + 1, base + 2]);
        indices.push([base, base + 2, base + 3]);
    }

    // projette une couronne sur le disque unité centré en (0.5, 0.5)
    fn disc_uv(theta: f64) -> Vec2 {
        Vec2::new(0.5 + 0.5 * theta.cos(), 0.5 + 0.5 * theta.sin())
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

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        for vertex in self.vertices.iter_mut() {
            vertex.set_attribut(Attribut::scalar(AttributKind::Shininess, shininess));
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
