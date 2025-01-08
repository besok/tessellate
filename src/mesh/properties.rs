use crate::mesh::parts::edge::MeshEdge;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{Mesh, MeshResult};
use std::collections::{HashMap, HashSet};

/// This struct provides various methods to analyze and query the properties
/// of a mesh, such as checking if it is manifold, watertight, or solid.
///
/// # Examples
///
/// ```
/// use crate::tessellate::mesh::shape::pyramid::Pyramid;
/// use crate::tessellate::mesh::{Mesh, properties::MeshProperties};
/// use crate::tessellate::mesh::parts::vertex::Vertex;
/// use crate::tessellate::mesh::parts::face::Face;
///
/// let pyramid = Pyramid::default();
///
/// assert!(&pyramid.props().is_manifold());
/// assert!(&pyramid.props().is_watertight());
/// ```
pub struct MeshProperties<'a> {
    mesh: &'a Mesh,
}

impl MeshProperties<'_> {
    pub fn new(mesh: &Mesh) -> MeshProperties {
        MeshProperties { mesh }
    }

    pub fn mesh(&self) -> &Mesh {
        self.mesh
    }
}

impl<'a> From<&'a Mesh> for MeshProperties<'a> {
    fn from(value: &'a Mesh) -> Self {
        MeshProperties::new(value)
    }
}

impl<'a> MeshProperties<'a> {
    pub fn is_manifold(&self) -> bool {
        let mut edge_count = HashMap::new();
        for face in self.mesh.faces() {
            for MeshEdge(a, b) in face.edges() {
                let edge = (a.min(b), a.max(b));
                *edge_count.entry(edge).or_insert(0) += 1;
            }
        }
        edge_count.values().all(|&count| count == 2)
    }

    pub fn is_watertight(&self) -> bool {
        self.is_manifold()
            && self.boundary_loops().is_empty()
            && self.isolated_vertices().is_empty()
    }

    pub fn is_solid(&self) -> bool {
        self.is_watertight()
            && self.is_non_self_intersecting()
    }

    pub fn is_non_self_intersecting(&self) -> bool {
        unimplemented!()
    }

    pub fn boundary_loops(&self) -> Vec<Vec<MeshEdge>> {
        let mut edge_count = HashMap::new();
        for face in self.mesh.faces() {
            for MeshEdge(a, b) in face.edges() {
                let edge = (a.min(b), a.max(b));
                *edge_count.entry(edge).or_insert(0) += 1;
            }
        }

        let boundary_edges: HashSet<_> = edge_count
            .iter()
            .filter(|&(_, &count)| count == 1)
            .map(|(&edge, _)| edge)
            .collect();

        let mut loops = Vec::new();
        let mut visited = HashSet::new();

        for &start_edge in &boundary_edges {
            if visited.contains(&start_edge) {
                continue;
            }

            let mut loop_edges = Vec::new();
            let mut current_edge = start_edge;

            loop {
                loop_edges.push(MeshEdge(current_edge.0, current_edge.1));
                visited.insert(current_edge);

                let next_vertex =
                    if current_edge.0 == start_edge.0 || current_edge.0 == start_edge.1 {
                        current_edge.1
                    } else {
                        current_edge.0
                    };

                let next_edge = boundary_edges.iter().find(|&&edge| {
                    (edge.0 == next_vertex || edge.1 == next_vertex) && !visited.contains(&edge)
                });

                if let Some(&next_edge) = next_edge {
                    current_edge = next_edge;
                } else {
                    break;
                }
            }

            loops.push(loop_edges);
        }

        loops
    }
    pub fn isolated_vertices(&self) -> Vec<&Vertex> {
        let mut all_vertices: HashSet<_> = self.mesh.vertices().into_iter().collect();
        for face in self.mesh.faces() {
            for vertex_idx in face.flatten() {
                if let Ok(v) = self.mesh.get(vertex_idx) {
                    all_vertices.remove(v);
                }
            }
        }
        all_vertices.into_iter().collect()
    }

    pub fn is_volume(&self) -> MeshResult<bool> {
        Ok(self.is_watertight()
            && self.is_manifold()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::material::Color;
    use crate::mesh::properties::MeshProperties;

    #[test]
    fn is_manifold() {
        use crate::mesh::parts::face::Face;
        use crate::mesh::parts::vertex::Vertex;
        use crate::mesh::Mesh;

        let vertices = vec![
            Vertex::new(0.0, 0.0, 0.0),
            Vertex::new(1.0, 0.0, 0.0),
            Vertex::new(1.0, 1.0, 0.0),
            Vertex::new(0.0, 1.0, 0.0),
        ];
        let faces = vec![
            Face::from((0, 1, 2)),
            Face::from((0, 2, 3)),
            Face::from((0, 3, 1)),
            Face::from((1, 3, 2)),
        ];
        let mesh = Mesh::from_vertices(vertices, faces, Color::default().into());
        let properties = MeshProperties::new(&mesh);
        assert!(properties.is_manifold());
    }
}
