# Properties

A set of properties that can be analyzed from a mesh.


## Example

```rust
use crate::tessellate::mesh::shape::pyramid::Pyramid;
use crate::tessellate::mesh::{Mesh, properties::MeshProperties};
use crate::tessellate::mesh::parts::vertex::Vertex;
use crate::tessellate::mesh::parts::face::Face;

fn main(){
    let pyramid = Pyramid::default();
    
    assert!(&pyramid.props().is_manifold());
    assert!(&pyramid.props().is_watertight());
    assert!(&pyramid.props().isolated_vertices().is_empty());
}
```