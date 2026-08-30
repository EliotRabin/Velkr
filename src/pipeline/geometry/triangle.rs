use crate::pipeline::geometry::fragment::Fragment;
use crate::pipeline::math::vec3::Vec3;

#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    v0: Fragment,
    v1: Fragment,
    v2: Fragment,
}

impl Triangle {
    pub fn new(v0: Fragment, v1: Fragment, v2: Fragment) -> Self {
        Triangle { v0, v1, v2 }
    }

    pub fn v0(&self) -> &Fragment {
        &self.v0
    }

    pub fn v1(&self) -> &Fragment {
        &self.v1
    }

    pub fn v2(&self) -> &Fragment {
        &self.v2
    }

    pub fn positions(&self) -> (Vec3, Vec3, Vec3) {
        (self.v0.position(), self.v1.position(), self.v2.position())
    }

    pub fn signed_area(&self) -> f64 {
        let (p0, p1, p2) = self.positions();
        Self::edge(&p0, &p1, &p2)
    }

    pub fn bounding_box(&self) -> (Vec3, Vec3) {
        let (p0, p1, p2) = self.positions();
        (p0.min(&p1).min(&p2), p0.max(&p1).max(&p2))
    }

    pub fn edge_weights(&self, p: &Vec3) -> (f64, f64, f64) {
        let (p0, p1, p2) = self.positions();
        (
            Self::edge(&p1, &p2, p),
            Self::edge(&p2, &p0, p),
            Self::edge(&p0, &p1, p),
        )
    }

    pub fn barycentric(&self, p: &Vec3) -> Option<(f64, f64, f64)> {
        let area = self.signed_area();
        if area == 0.0 {
            return None;
        }

        let (w0, w1, w2) = self.edge_weights(p);
        let alpha = w0 / area;
        let beta = w1 / area;
        let gamma = w2 / area;

        if alpha < 0.0 || beta < 0.0 || gamma < 0.0 {
            return None;
        }

        Some((alpha, beta, gamma))
    }

    pub fn interpolate_z(&self, alpha: f64, beta: f64, gamma: f64) -> f64 {
        alpha * self.v0.depth() + beta * self.v1.depth() + gamma * self.v2.depth()
    }

    pub fn fragment_at(&self, p: &Vec3) -> Option<Fragment> {
        let (alpha, beta, gamma) = self.barycentric(p)?;
        Some(Fragment::interpolate(&self.v0, &self.v1, &self.v2, alpha, beta, gamma))
    }

    fn edge(p0: &Vec3, p1: &Vec3, p: &Vec3) -> f64 {
        (p.x() - p0.x()) * (p1.y() - p0.y()) - (p.y() - p0.y()) * (p1.x() - p0.x())
    }
}
