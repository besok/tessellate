# Parts and components of mesh

## Vertex
A `Vertex` represents a point in 3D space with x, y, and z coordinates.
```rust
use tessellate::v;
fn main(){
    let vertex1 = v!(1.0, 2.0, 3.0);
    let vertex2 = Vertex::new(1.0, 2.0, 3.0);
    let vertex3 = [1.0, 2.0, 3.0].into();
    
    println!("{:?}", [vertex2, vertex3, vertex1]);
}
```

## Edge
An `Edge` represents a connection between two vertices in a mesh.

There are two types of edges:
 -  MeshEdge - An edge that connects two vertices in a mesh using their indices.
 -  Edge - An edge that connects two vertices in a mesh using the vertices themselves.

```rust
fn main(){
    let edge:Edge = (v!(0.0, 0.0, 0.0), v!(1.0, 1.0, 1.0)).into();
}

```

## Face
A `Face` represents a polygonal surface in a mesh, typically defined by three or more vertices.
There are two types of faces:
  - Triangle - A face with three vertices.
  - Quad - A face with four vertices.

The face works with indices of vertices in the mesh.

```rust

use tessellate::mesh::parts::face::Face;
fn main(){
    let triangle = Face::new3(0, 1, 2);
 
    let quad = Face::new4(0, 1, 2, 3);

    // Creating faces from a vector of elements with an attempt to triangulate it.
    let faces = Face::new(vec![0, 1, 2, 3, 4, 5, 6]).unwrap();
}

```

## Polygon
A `Polygon` is a flat shape consisting of straight, non-intersecting lines that are joined to form a closed chain or circuit.

```rust
use tessellate::mesh::parts::polygon::Polygon;
use tessellate::mesh::parts::vertex::Vertex;

fn main() { 
    // Creating a polygon with three vertices
    let v0 = Vertex::new(0.0, 0.0, 0.0);
    let v1 = Vertex::new(1.0, 0.0, 0.0);
    let v2 = Vertex::new(0.0, 1.0, 0.0);
    let triangle = Polygon::new(vec![v0, v1, v2]);
    
    // Creating a polygon with four vertices
    let v3 = Vertex::new(1.0, 1.0, 0.0);
    let quad:Polygon = vec![v0, v1, v2, v3].into(); 
}

```

## Plane
A `Plane` is a flat, two-dimensional surface that extends infinitely in 3D space.
```rust
use glam::Vec3;
use tessellate::mesh::attributes::Attributes;
use tessellate::mesh::parts::plane::Plane;

fn main(){ 
    // Create a plane from a normal vector and a point
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let point = Vec3::new(0.0, 0.0, 0.0);
    let plane = Plane::new(normal, point);
    
    // Calculate the distance from a point to the plane
    let distance = plane.distance(Vec3::new(0.0, 2.0, 0.0));
    assert_eq!(distance, 2.0);
}
```
## BBox
A `BBox` (Bounding Box) is a rectangular box that completely contains a 3D object, 
used for spatial indexing and collision detection.

