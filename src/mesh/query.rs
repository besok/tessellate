use super::parts::vertex::Vertex;
use super::MeshError;
use crate::mesh::parts::edge::Edge;
use crate::mesh::query::bsp::BSPTree;
use crate::mesh::query::kdtree::KDTree;
use crate::mesh::query::octree::Octree;
use crate::mesh::query::sskdtree::SSKDTree;
use crate::mesh::{Mesh, MeshResult};

pub mod bsp;
pub mod connectivity;
pub mod edges;
pub mod kdtree;
pub mod octree;
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

    /// Extract the manifold edges of the mesh
    ///
    /// This function identifies and returns the edges that are shared by exactly two faces.
    ///
    /// # Returns
    ///
    /// * `MeshResult<Vec<Edge>>` - A result containing a vector of manifold edges or an error.
    pub fn extract_manifold_edges(&self) -> MeshResult<Vec<Edge>> {
        edges::extract_manifold_edges(self.0)
    }

    /// Extract the non-manifold edges of the mesh
    ///
    /// This function identifies and returns the edges that are shared by more than two faces.
    ///
    /// # Returns
    ///
    /// * `MeshResult<Vec<Edge>>` - A result containing a vector of non-manifold edges or an error.
    pub fn extract_non_manifold_edges(&self) -> MeshResult<Vec<Edge>> {
        edges::extract_non_manifold_edges(self.0)
    }

    /// Extract the feature edges of the mesh
    ///
    /// This function identifies and returns the edges that form a feature angle greater than the specified threshold.
    ///
    /// # Arguments
    ///
    /// * `feature_angle` - The angle threshold in degrees.
    ///
    /// # Returns
    ///
    /// * `MeshResult<Vec<Edge>>` - A result containing a vector of feature edges or an error.
    pub fn extract_feature_edges(&self, feature_angle: f32) -> MeshResult<Vec<Edge>> {
        edges::extract_feature_edges(self.0, feature_angle)
    }

    /// Extract the connected regions of the mesh
    /// This function identifies and returns all the connected regions within the mesh.
    pub fn extract_connected_regions(&self) -> MeshResult<Vec<Mesh>> {
        connectivity::connected_regions(self.0)
    }
    pub fn extract_largest_connected_region(&self) -> MeshResult<Mesh> {
        connectivity::connected_regions(self.0)?
            .into_iter()
            .max_by_key(|m| m.faces().len())
            .ok_or(MeshError::Custom("No connected region found".to_string()))
    }

    /// Extract the connected regions of the mesh by vertex indices
    ///
    /// This function identifies and returns the connected regions
    /// within the mesh that contain any of the specified vertex indices.
    ///
    /// # Arguments
    ///
    /// * `seeds` - A vector of vertex indices to use as seeds for identifying connected regions.
    ///
    /// # Returns
    ///
    /// * `MeshResult<Vec<Mesh>>` - A result containing a vector of connected regions or an error.
    pub fn extract_connected_regions_by_vertexes(
        &self,
        seeds: Vec<usize>,
    ) -> MeshResult<Vec<Mesh>> {
        let vertices = self.0.vertices();
        let seeds = seeds
            .into_iter()
            .map(|i| vertices.get(i).ok_or(MeshError::idx_vertex(i)))
            .collect::<MeshResult<Vec<&Vertex>>>()?;

        Ok(connectivity::connected_regions(self.0)?
            .into_iter()
            .filter(|m| seeds.iter().any(|v| m.vertices().contains(v)))
            .collect())
    }

    /// Extract the closest connected region to a given point
    ///
    /// This function identifies and returns the connected region within the mesh
    /// that is closest to the specified point.
    ///
    /// # Arguments
    ///
    /// * `point` - A reference to a `Vertex` representing the point to which the closest connected region is to be found.
    ///
    /// # Returns
    ///
    /// * `MeshResult<Mesh>` - A result containing the closest connected region or an error.
    pub fn extract_closest_connected_region(&self, point: &Vertex) -> MeshResult<Mesh> {
        connectivity::connected_regions(self.0)?
            .into_iter()
            .min_by_key(|m| {
                m.vertices()
                    .iter()
                    .map(|v| v.distance_rounded(point))
                    .min()
                    .unwrap_or(usize::MAX)
            })
            .ok_or(MeshError::Custom("No connected region found".to_string()))
    }
}
