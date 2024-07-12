use crate::mesh::parts::Vertex;
use crate::mesh::Mesh;

/// A solid object bounded by six square faces, with three meeting at each vertex.
/// Regular hexahedron, Platonic solid, consists of 6 faces, 12 edges, and 8 vertices.
pub struct Cube {
    center: Vertex,
    size: f32,
    mesh: Mesh,
}

/// A solid object bounded by six rectangular faces, with three meeting at each vertex.
/// Rectangular cuboid, consists of 6 faces, 12 edges, and 8 vertices.
pub struct RectCuboid {
    center: Vertex,
    width: f32,
    length: f32,
    depth: f32,
    mesh: Mesh,
}
