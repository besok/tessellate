use std::ops::Deref;
use crate::mesh::attributes::Attributes;
use crate::mesh::material::Color;
use crate::mesh::HasMesh;
use crate::mesh::Mesh;
use crate::mesh::parts::face::{Face, FaceType};
use crate::mesh::parts::vertex::Vertex;

/// A solid object bounded by six square faces, with three meeting at each vertex.
/// Regular hexahedron, Platonic solid, consists of 6 faces, 12 edges, and 8 vertices.

#[derive(Debug, Clone)]
pub struct Cube {
    mesh: Mesh,
    center: Vertex,
    size: f32,
}

impl Deref for Cube {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl HasMesh for Cube {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}

impl Default for Cube {
    fn default() -> Self {
        Cube::create([0.0, 0.0, 0.0], 1.0, Default::default(), Color::default())
    }
}

impl Cube {
    pub fn create<V,C>(center: V, size: f32, face_type: FaceType,attrs: C) -> Self
    where
        V: Into<Vertex>,
        C: Into<Attributes>
    {
        let half_size = size / 2.0;
        let center = center.into();
        let attrs = attrs.into();
        let vertices = vec![
            Vertex::new(center.x - half_size, center.y - half_size, center.z - half_size),
            Vertex::new(center.x + half_size, center.y - half_size, center.z - half_size),
            Vertex::new(center.x + half_size, center.y + half_size, center.z - half_size),
            Vertex::new(center.x - half_size, center.y + half_size, center.z - half_size),
            Vertex::new(center.x - half_size, center.y - half_size, center.z + half_size),
            Vertex::new(center.x + half_size, center.y - half_size, center.z + half_size),
            Vertex::new(center.x + half_size, center.y + half_size, center.z + half_size),
            Vertex::new(center.x - half_size, center.y + half_size, center.z + half_size),
        ];

        let faces: Vec<Face> = match face_type {
            FaceType::Triangle =>
            {
                #[cfg_attr(any(), rustfmt::skip)]
                vec![
                    (0, 1, 2), (0, 2, 3),
                    (1, 5, 6), (1, 6, 2),
                    (5, 4, 7), (5, 7, 6),
                    (4, 0, 3), (4, 3, 7),
                    (3, 2, 6), (3, 6, 7),
                    (4, 5, 1), (4, 1, 0),
                ].into_iter().map(Into::into).collect()
            }
            FaceType::Quad =>
            {
                #[cfg_attr(any(), rustfmt::skip)]
                vec![
                    (0, 1, 2, 3),
                    (0, 1, 5, 4),
                    (1, 2, 6, 5),
                    (2, 3, 7, 6),
                    (3, 0, 4, 7),
                    (4, 5, 6, 7),
                ].into_iter().map(Into::into).collect()
            }
        };

        Cube {
            mesh: Mesh::from_vertices(vertices, faces, attrs),
            center,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::shape::cuboid::cube::Cube;

    #[test]
    fn test_cuboid() {
        let cube = Cube::default();
        assert_eq!(cube.mesh.vertices().len(), 8);
    }
}
