use crate::mesh::query::bsp::BSPTree;
use crate::mesh::query::kdtree::KDTree;
use crate::mesh::query::sskdtree::SSKDTree;
use crate::mesh::{Mesh, MeshResult};

pub mod bsp;
pub mod kdtree;
pub mod sskdtree;

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

    /// Try to build a BSPTree from the mesh
    /// # Arguments
    /// * `depth` - The maximum depth of the tree
    pub fn try_bsp_tree(&self,depth: Option<usize>) -> MeshResult<BSPTree> {
        BSPTree::try_from_mesh(self.0, depth)
    }
}
