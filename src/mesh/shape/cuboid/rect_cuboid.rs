use std::ops::Deref;
use crate::mesh::attributes::Attributes;
use crate::mesh::material::Color;
use crate::mesh::HasMesh;
use crate::mesh::Mesh;
use crate::mesh::parts::face::{Face, FaceType};
use crate::mesh::parts::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct RectCuboid {
    mesh: Mesh,
    center: Vertex,
    size_x: f32,
    size_y: f32,
    size_z: f32,
}
impl Deref for RectCuboid {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
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
        attrs: C
    ) -> Self
    where
        V: Into<Vertex>,
        C: Into<Attributes>,
    {
        let half_size_x = size_x / 2.0;
        let half_size_y = size_y / 2.0;
        let half_size_z = size_z / 2.0;
        let center = center.into();
        let attrs = attrs.into();
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
            mesh: Mesh::from_vertices(vertices, faces, attrs),
            center,
            size_x,
            size_y,
            size_z,
        }
    }

    pub fn create_bbox<V,C>(min_v:V, max_v:V,face_type: FaceType, attributes:  C) -> Self
    where
        V: Into<Vertex>,
        C: Into<Attributes>,
    {
        let min_v = min_v.into();
        let max_v = max_v.into();
        let center = Vertex::new(
            (min_v.x + max_v.x) / 2.0,
            (min_v.y + max_v.y) / 2.0,
            (min_v.z + max_v.z) / 2.0,
        );
        let size_x = max_v.x - min_v.x;
        let size_y = max_v.y - min_v.y;
        let size_z = max_v.z - min_v.z;
        Self::create(center, size_x, size_y, size_z, face_type, attributes)
    }
}

impl Default for RectCuboid {
    fn default() -> Self {
        RectCuboid::create([0.0, 0.0, 0.0], 1.0, 1.0, 1.0, Default::default(), Color::default())
    }
}
