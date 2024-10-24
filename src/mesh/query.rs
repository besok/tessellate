use super::parts::vertex::Vertex;
use crate::mesh::parts::edge::Edge;
use crate::mesh::query::bsp::BSPTree;
use crate::mesh::query::kdtree::KDTree;
use crate::mesh::query::octree::Octree;
use crate::mesh::query::sskdtree::SSKDTree;
use crate::mesh::{Mesh, MeshResult};

pub mod bsp;
pub mod kdtree;
pub mod octree;
pub mod sskdtree;
pub mod edges;

/// A query object for a mesh
pub struct MeshQuery<'a>(&'a Mesh);

impl<'a> From<&'a Mesh> for MeshQuery<'a> {
    fn from(value: &'a Mesh) -> Self {
        Self(value)
    }
}

impl<'a> MeshQuery<'a> {
    pub fn new(mesh: &'a Mesh) -> Self {
        Self(mesh)
    }

    pub fn mesh(&self) -> &Mesh {
        self.0
    }

    /// Try to build a KDTree from the mesh
    /// # Arguments
    /// * `depth` - The maximum depth of the tree
    pub fn try_kd_tree(&self, depth: Option<usize>) -> MeshResult<KDTree> {
        KDTree::try_from_mesh(self.0, depth)
    }
    /// Try to build a SSKDTree from the mesh
    /// # Arguments
    /// * `depth` - The maximum depth of the tree
    /// * `min_polygons` - The minimum number of polygons in a leaf node
    pub fn try_sskd_tree(
        &self,
        depth: Option<usize>,
        min_polygons: Option<usize>,
    ) -> MeshResult<SSKDTree> {
        SSKDTree::try_from_mesh(self.0, depth, min_polygons)
    }

    /// Try to build an Octree from the mesh
    /// # Arguments
    /// * `depth` - The maximum depth of the tree
    pub fn try_octree(&self, depth: Option<usize>) -> MeshResult<Octree> {
        Octree::try_from_mesh(self.0, depth)
    }

    /// Try to build a BSPTree from the mesh
    /// # Arguments
    /// * `depth` - The maximum depth of the tree
    pub fn try_bsp_tree(&self, depth: Option<usize>) -> MeshResult<BSPTree> {
        BSPTree::try_from_mesh(self.0, depth)
    }

    /// Extract the centers of the polygons
    pub fn extract_poly_centers(&self) -> MeshResult<Vec<Vertex>> {
        self.0
            .try_polygons()?
            .into_iter()
            .map(|v| v.centroid())
            .collect::<MeshResult<Vec<Vertex>>>()
    }
    /// Extract the centers of the edges
    pub fn extract_edge_centers(&self) -> MeshResult<Vec<Vertex>> {
        Ok(self
            .0
            .try_edges()?
            .into_iter()
            .map(|Edge { a, b }| (a + b) * 0.5)
            .collect::<Vec<Vertex>>())
    }

    /// Extract the boundary edges of the mesh
    ///
    /// This function identifies and returns the edges that are on the boundary of the mesh.
    /// Boundary edges are those that belong to only one face.
    ///
    /// # Returns
    ///
    /// * `MeshResult<Vec<Edge>>` - A result containing a vector of boundary edges or an error.
    pub fn extract_boundary_edges(&self) -> MeshResult<Vec<Edge>> {
        edges::extract_boundary_edges(self.0)
    }
}
