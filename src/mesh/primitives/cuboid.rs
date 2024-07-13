use crate::mesh::parts::{Edge, Face, FaceType, Vertex};
use crate::mesh::Mesh;

/// A solid object bounded by six square faces, with three meeting at each vertex.
/// Regular hexahedron, Platonic solid, consists of 6 faces, 12 edges, and 8 vertices.
pub struct Cube {
    mesh: Mesh,
}

impl Default for Cube {
    fn default() -> Self {
        Cube::from_center([0.0, 0.0, 0.0], 1.0, Default::default())
    }
}

impl Cube {
    pub fn from_center<V>(center: V, size: f32, face_type: FaceType) -> Self
    where
        V: Into<Vertex>,
    {
        let half_size = size / 2.0;
        let center = center.into();
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
        #[rustfmt::skip]
        let edges:Vec<Edge> = vec![
            (0usize, 1usize), (1, 2), (2, 3), (3, 0),
            (4, 5), (5, 6), (6, 7), (7, 4),
            (0, 4), (1, 5), (2, 6), (3, 7),
        ].into_iter().map(Into::into).collect();

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
            mesh: Mesh::from_vertices(vertices, edges, faces),
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::mesh::primitives::cuboid::Cube;

    #[test]
    fn test_cuboid() {
        let cube = Cube::default();
        assert_eq!(cube.mesh.vertices().len(), 8);
    }
}
