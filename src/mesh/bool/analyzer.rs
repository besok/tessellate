use crate::mesh::parts::edge::MeshEdge;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::polygon::{Polygon, Triangle};
use crate::mesh::parts::ray::Ray;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{Mesh, MeshError, MeshResult};
use std::collections::HashMap;

struct CacheEntry<T> {
    data: T,
    triangles: [usize; 2],
}

impl<T: Default> CacheEntry<T> {
    fn add_triangle(&mut self, idx: usize) {
        if self.triangles.is_empty() {
            self.triangles[0] = idx;
        } else {
            self.triangles[1] = idx;
        }
    }
    fn new(idx: usize) -> Self {
        Self {
            data: T::default(),
            triangles: [idx, 2],
        }
    }
}

struct MeshCache<T: Default> {
    cache: HashMap<MeshEdge, CacheEntry<T>>,
}

impl<T: Default> MeshCache<T> {
    fn new(faces: &Vec<Face>) -> MeshResult<Self> {
        let mut cache = HashMap::new();
        for (idx, face) in faces.iter().enumerate() {
            for edge in face.edges() {
                cache
                    .entry(edge)
                    .or_insert(CacheEntry::new(idx))
                    .add_triangle(idx);

                cache
                    .entry(edge.inv())
                    .or_insert(CacheEntry::new(idx))
                    .add_triangle(idx);
            }
        }

        Ok(Self { cache })
    }
}

pub(crate) struct MeshBoolAnalyzer<T> {
    vertices: Vec<Vertex>,
    faces: Vec<Face>,
    flags: Vec<u8>,
}

impl<'a, T: Default> MeshBoolAnalyzer<T> {
    pub(crate) fn new(mesh: &'a Mesh) -> MeshResult<MeshBoolAnalyzer<T>> {
        Ok(Self {
            vertices: mesh.vertices().to_vec(),
            faces: mesh.faces().to_vec(),
            flags: mesh.faces().iter().map(|_| 0).collect(),
        })
    }

    fn v(&self, idx: usize) -> MeshResult<Vertex> {
        self.vertices
            .get(idx)
            .cloned()
            .ok_or(MeshError::InvalidIndex(format!("vertex {:?}", idx)))
    }

    fn face_idx_to_poly(&self, idx: usize) -> MeshResult<Polygon> {
        Ok(Polygon::new(
            self.faces
                .get(idx)
                .ok_or(MeshError::InvalidIndex(format!("face {:?}", idx)))?
                .flatten()
                .iter()
                .map(|i| self.v(*i))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    pub fn is_inside(&self, face: usize, flag: u8) -> MeshResult<bool> {
        let centroid = self.face_idx_to_poly(face)?.centroid()?;

        let ray = Ray::new_rand(centroid);
        let mut winding = 0;
        for (i, _) in self.faces.iter().enumerate() {
            if self.flags.get(i).map(|f| *f != flag).unwrap_or(false) {
                let poly = self.face_idx_to_poly(i)?;
                if ray.intersects(poly.clone().try_into()?) {
                    if poly.normal().dot(&ray.direction) > 0.0 {
                        winding += 1;
                    } else {
                        winding -= 1;
                    }
                }
            }
        }

        Ok(winding > 0)
    }

    pub fn prepare(&mut self, rhs: &Mesh) -> MeshResult<()> {
        let rhs_vertices = rhs.vertices().to_vec();
        let rhs_faces = rhs.faces().to_vec();

        let offset = self.faces.len();
        self.vertices.extend(rhs_vertices);
        for face in rhs_faces {
            self.faces.push(face.with_offset(offset));
            self.flags.push(1);
        }


        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::shape::cuboid::rect_cuboid::RectCuboid;

    #[test]
    fn smoke() {
        let cube = RectCuboid::default();
    }
}
