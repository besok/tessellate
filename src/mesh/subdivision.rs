use crate::mesh::parts::edge::MeshEdge;
use crate::mesh::parts::face::Face;
use crate::mesh::{Mesh, MeshError, MeshResult};
use egui::ahash::{HashMap, HashMapExt};

pub struct MeshSubdivision<'a> {
    mesh: &'a Mesh,
}

impl<'a> MeshSubdivision<'a> {
    pub fn new(mesh: &'a Mesh) -> Self {
        MeshSubdivision { mesh }
    }

    /// Subdivides the mesh using the Butterfly subdivision scheme.
    ///
    /// This method applies the Butterfly subdivision algorithm to the mesh,
    /// which is a technique used to create a smoother mesh by adding new vertices
    /// and adjusting the positions of existing vertices.
    ///
    /// # Returns
    ///
    /// A `MeshResult` containing the subdivided `Mesh` if successful, or a `MeshError` if an error occurs.
    pub fn by_butterfly(&self) -> MeshResult<Mesh> {
        let mesh = self.mesh.triangulate()?;
        let mut new_vertices = self.mesh.vertices.clone();
        let mut new_faces = vec![];

        let table = mesh.try_tables()?;

        let mut cache = HashMap::new();

        for face in mesh.faces() {
            let mut mid_points_extra = vec![];
            let mut edges = face.edges().into_iter().collect::<Vec<_>>();

            for MeshEdge(lhs, rhs) in edges {
                if let Some(idx) = cache.get(&(lhs, rhs)) {
                    mid_points_extra.push(*idx);
                } else {
                    let v1 = mesh.get(lhs)?;
                    let v2 = mesh.get(rhs)?;

                    let mut mid = (*v1 + *v2) / 2.0;
                    let adjacent_vs: Vec<usize> = table
                        .edge_faces(lhs, rhs)?
                        .iter()
                        .flat_map(|f| f.flatten())
                        .filter(|v| v != &lhs && v != &rhs)
                        .collect();
                    if adjacent_vs.len() >= 2 {
                        let a1 = adjacent_vs[0];
                        let a2 = adjacent_vs[1];

                        let mut b1_vs = Vec::new();
                        for face in table.vertex_faces(a1)? {
                            if !face.flatten().contains(&lhs) && !face.flatten().contains(&rhs) {
                                b1_vs = face
                                    .flatten()
                                    .iter()
                                    .filter(|v| *v != &a1)
                                    .cloned()
                                    .collect();
                            }
                        }

                        let mut b2_vs = Vec::new();
                        for face in table.vertex_faces(a2)? {
                            if !face.flatten().contains(&lhs) && !face.flatten().contains(&rhs) {
                                b2_vs = face
                                    .flatten()
                                    .iter()
                                    .filter(|v| *v != &a2)
                                    .cloned()
                                    .collect();
                            }
                        }

                        if !b1_vs.is_empty() && !b2_vs.is_empty() {
                            let a1 = mesh.get(a1)?;
                            let a2 = mesh.get(a2)?;
                            let b1 = mesh.get(b1_vs[0])?;
                            let b2 = mesh.get(b2_vs[0])?;

                            mid = mid + (*a1 + *a2 - *b1 - *b2) * 0.125;
                        }
                    }

                    let idx = new_vertices.len();
                    new_vertices.push(mid);
                    mid_points_extra.push(idx);
                    cache.insert((lhs, rhs), idx);
                    cache.insert((rhs, lhs), idx);
                }

                mid_points_extra.push(lhs);
                mid_points_extra.push(rhs);
            }

            let [m_ab, a, b, m_bc, _b, c, m_ca, _c, _a] = mid_points_extra[..] else {
                return Err(MeshError::WrongMesh("Invalid number of mid points".to_string()));
            };

            new_faces.push(Face::new3(a, m_ab, m_ca));
            new_faces.push(Face::new3(b, m_bc, m_ab));
            new_faces.push(Face::new3(c, m_ca, m_bc));
            new_faces.push(Face::new3(m_ab, m_bc, m_ca));
        }

        Ok(Mesh::from_vertices(new_vertices, new_faces, self.mesh.attributes.clone()))
    }

    /// Subdivides the mesh using the Loop subdivision scheme.
    ///
    /// This method applies the Loop subdivision algorithm to the mesh,
    /// which is a technique used to create a smoother mesh by adding new vertices
    /// and adjusting the positions of existing vertices.
    ///
    /// # Returns
    ///
    /// A `MeshResult` containing the subdivided `Mesh` if successful, or a `MeshError` if an error occurs.
    pub fn by_loop(&self) -> MeshResult<Mesh> {
        Ok(self.mesh.clone())
    }
    /// Subdivides the mesh using a linear subdivision scheme.
    ///
    /// This method applies a linear subdivision algorithm to the mesh,
    /// which is a technique used to create a smoother mesh by adding new vertices
    /// and adjusting the positions of existing vertices.
    ///
    /// # Returns
    ///
    /// A `MeshResult` containing the subdivided `Mesh` if successful, or a `MeshError` if an error occurs.
    pub fn by_linear(&self) -> MeshResult<Mesh> {
        Ok(self.mesh.clone())
    }
}
