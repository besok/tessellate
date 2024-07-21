use crate::mesh::material::Color;
use crate::mesh::parts::{Edge, Face, FaceType, Vertex};
use crate::mesh::HasMesh;
use crate::mesh::Mesh;
#[derive(Debug, Clone)]
pub struct RectCuboid {
    mesh: Mesh,
    center: Vertex,
    size_x: f32,
    size_y: f32,
    size_z: f32,
}

impl HasMesh for RectCuboid {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}

impl RectCuboid {
    pub fn create<V,C>(
        center: V,
        size_x: f32,
        size_y: f32,
        size_z: f32,
        face_type: FaceType,
        color: C,
    ) -> Self
    where
        V: Into<Vertex>,
        C: Into<Color>,
    {
        let half_size_x = size_x / 2.0;
        let half_size_y = size_y / 2.0;
        let half_size_z = size_z / 2.0;
        let center = center.into();
        let color = color.into();
        let vertices = vec![
            Vertex::new(center.x - half_size_x, center.y - half_size_y, center.z - half_size_z),
            Vertex::new(center.x + half_size_x, center.y - half_size_y, center.z - half_size_z),
            Vertex::new(center.x + half_size_x, center.y + half_size_y, center.z - half_size_z),
            Vertex::new(center.x - half_size_x, center.y + half_size_y, center.z - half_size_z),
            Vertex::new(center.x - half_size_x, center.y - half_size_y, center.z + half_size_z),
            Vertex::new(center.x + half_size_x, center.y - half_size_y, center.z + half_size_z),
            Vertex::new(center.x + half_size_x, center.y + half_size_y, center.z + half_size_z),
            Vertex::new(center.x - half_size_x, center.y + half_size_y, center.z + half_size_z),
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

        Self {
            mesh: Mesh::from_vertices(vertices, faces, color),
            center,
            size_x,
            size_y,
            size_z,
        }
    }
}

impl Default for RectCuboid {
    fn default() -> Self {
        RectCuboid::create([0.0, 0.0, 0.0], 1.0, 1.0, 1.0, Default::default(), Color::default())
    }
}
