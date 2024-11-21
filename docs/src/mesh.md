# Mesh

The `Mesh` struct represents a 3D mesh consisting of vertices, edges, and faces. 
It is the core data structure for representing 3D geometries in Tessellate. 
The `Mesh` struct provides various methods for creating, manipulating, and analyzing 3D meshes.

## Fields

- **vertices**: A vector of vertices in the mesh.
- **edges**: A vector of edges in the mesh.
- **faces**: A vector of faces in the mesh.
- **attributes**: Additional attributes associated with the mesh.
 

## Example of creating a mesh

```rust
use tessellate::mesh::attributes::Attributes;
use tessellate::mesh::parts::polygon::Polygon;
use tessellate::mesh::parts::vertex::Vertex;
use tessellate::mesh::Mesh;

fn main() {
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
    ];
 
    let mesh = Mesh::from_vertices(vertices, faces, Attributes::default());

    // Print the mesh details
    println!("{:?}", mesh);
}
```

or like that 
```rust
fn main() {
    let mesh = Mesh::from_polygons(
        vec![
            poly!(-2.5, -2.5, 0.0; 2.5, -2.5, 0.0; 0.0, 0.0, 5.0),
            poly!(2.5, -2.5, 0.0; 2.5, 2.5, 0.0; 0.0, 0.0, 5.0),
        ],
        Attributes::default(),
    );
}
```