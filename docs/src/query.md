# Query

A set of queries that can be performed on a mesh. 
It helps to analyze the mesh and extract useful information.
It uses auxiliary structures like `KDTree` and `BSP Tree` to perform the queries.

## Example

```rust
use std::process::Termination;
use crate::tessellate::mesh::shape::cylinder::Cylinder;

fn main() -> TessResult<()> {
    let cylinder = Cylinder::default();
    
    let kd_tree = &cylinder.query().try_kd_tree(None)?;
    let sskd_tree = &cylinder.query().try_sskd_tree(None, None)?;
    let octree = &cylinder.query().try_octree(None)?;
    let bsp_tree = &cylinder.query().try_bsp_tree(None)?;
    
    let poly_centers = &cylinder.query().extract_poly_centers()?;
    let edge_centers = &cylinder.query().extract_edge_centers()?;
    let manifold_edges = &cylinder.query().extract_manifold_edges()?;
    
}
```