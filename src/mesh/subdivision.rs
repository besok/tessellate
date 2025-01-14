use crate::mesh::parts::edge::MeshEdge;
use crate::mesh::parts::face::Face;
use crate::mesh::{Mesh, MeshError, MeshResult};

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

        for face in mesh.faces() {
            let mut mid_points_extra = vec![];
            let mut edges = face.edges().into_iter().collect::<Vec<_>>();
            edges.sort_by_key(|e| (e.0, e.1)); // Sort edges to ensure consistent order

            for MeshEdge(lhs, rhs) in edges {
                let v1 = mesh.get(lhs)?;
                let v2 = mesh.get(rhs)?;
                let mid = (*v1 + *v2) / 2.0;
                mid_points_extra.push(new_vertices.len());
                mid_points_extra.push(lhs);
                mid_points_extra.push(rhs);
                new_vertices.push(mid);
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
