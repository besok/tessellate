use crate::mesh::material::Color;
use crate::mesh::parts::polygon::Triangle;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::shape::beam::Beam;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vertex,
    pub direction: Vertex,
}

impl Ray {
    pub fn new(origin: Vertex, direction: Vertex) -> Self {
        Self { origin, direction }
    }

    pub fn intersects(&self, triangle: Triangle) -> bool {
        let Triangle { v0, v1, v2 } = triangle;
        // re-center the problem at the base point of the ray
        let v0 = v0 - self.origin;
        let v1 = v1 - self.origin;
        let v2 = v2 - self.origin;

        // Then compute volumes of tetrahedra spanning the origin and the triangle
        let vol_01 = det(&v0, &v1, &self.direction);
        let vol_12 = det(&v1, &v2, &self.direction);
        let vol_20 = -det(&v0, &v2, &self.direction);

        let vol_012 = det(&v0, &v1, &v2);

        // if any of the signs of the edge tests
        // disagree with the sign of the whole triangle, then
        // the ray does not pass through the triangle
        if (vol_01 * vol_012 < 0.0) || (vol_12 * vol_012 < 0.0) || (vol_20 * vol_012 < 0.0) {
            false
        } else {
            // otherwise, compute the t - value for the ray to intersect
            // if this is negative, then the client can detect that the
            // ray would have to travel backwards to hit the triangle in question.
            let edge_sum = vol_01 + vol_12 + vol_20;
            if edge_sum == 0.0 {
                false
            } else {
                (vol_012 / edge_sum) > 0.0
            }
        }
    }

    pub fn new_rand(origin: Vertex) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            origin,
            direction: Vertex::new(
                rng.gen_range(0.5..1.5),
                rng.gen_range(0.5..1.5),
                rng.gen_range(0.5..1.5),
            ),
        }
    }

    pub fn to_beam<C>(&self, length: f32, diam: f32, color: Option<Color>) -> Beam {
        Beam::create(
            self.origin,
            self.origin + self.direction * length,
            diam,
            color.unwrap_or_default(),
        )
    }
}

#[rustfmt::skip]
fn det(a: &Vertex, b: &Vertex, c: &Vertex) -> f32 {
    a.x * (b.y * c.z - b.z * c.y) -
    a.y * (b.x * c.z - b.z * c.x) +
    a.z * (b.x * c.y - b.y * c.x)
}
