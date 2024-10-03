use crate::mesh::parts::edge::MeshEdge;
use crate::mesh::parts::polygon::Triangle;
use crate::mesh::parts::ray::Ray;
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

pub(crate) struct MeshBoolAnalyzer<'a, T> {
    mesh: &'a Mesh,
    flags: Vec<u8>,
    cache: HashMap<MeshEdge, CacheEntry<T>>,
}

impl<'a, T: Default> MeshBoolAnalyzer<'a, T> {
    pub(crate) fn new(mesh: &'a Mesh) -> MeshResult<MeshBoolAnalyzer<'a, T>> {
        let mut cache = HashMap::new();
        let mut tri_load = Vec::new();
        for (idx, face) in mesh.faces().iter().enumerate() {
            for edge in face.edges() {
                tri_load.push(0);
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

        Ok(Self {
            mesh,
            flags: tri_load,
            cache,
        })
    }
    pub fn iter_mut<F>(&mut self, mut operation: F)
    where
        F: FnMut(&MeshEdge, &mut CacheEntry<T>, Vec<&u8>) -> T,
    {
        for (edge, entry) in self.cache.iter_mut() {
            let mut loads = Vec::new();
            for idx in entry.triangles.iter() {
                self.flags.get(*idx).map(|load| loads.push(load));
            }
            entry.data = operation(edge, entry, loads);
        }
    }

    pub fn is_inside(&self, face: usize, flag: u8) -> MeshResult<bool> {
        let centroid = self
            .mesh
            .faces
            .get(face)
            .ok_or(MeshError::InvalidIndex("a".to_string()))
            .and_then(|f| self.mesh.face_to_polygon(f))
            .and_then(|p| p.centroid())?;

        let ray = Ray::new_rand(centroid);
        let mut winding = 0;
        for (i, f) in self.mesh.faces.iter().enumerate() {
            if self.flags.get(i).map(|f| *f == flag).unwrap_or(false) {
                continue;
            }
            let poly = self.mesh.face_to_polygon(f)?;
            if ray.intersects(poly.clone().try_into()?) {
                if poly.normal().dot(&ray.direction) > 0.0 {
                    winding += 1;
                } else {
                    winding -= 1;
                }
            }
        }

        Ok(winding > 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::bool::analyzer::MeshBoolAnalyzer;
    use crate::mesh::shape::cuboid::rect_cuboid::RectCuboid;

    #[test]
    fn smoke() {
        let cube = RectCuboid::default();
        let mut cache: MeshBoolAnalyzer<bool> = MeshBoolAnalyzer::new(&cube).unwrap();

        cache.iter_mut(|_, entry, loads| {
            if loads.windows(2).all(|w| w[0] == w[1]) {
                true
            } else {
                false
            }
        });
    }
}
