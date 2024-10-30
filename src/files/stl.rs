use crate::files::FileError;
use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::Mesh;
use ply_rs::ply::{Property, PropertyAccess};
use std::fs::OpenOptions;
use stl_io::{IndexedTriangle, Vector};

pub fn import_stl(path: &str) -> Result<Mesh, FileError> {
    let mut file = OpenOptions::new().read(true).open(path)?;
    let stl = stl_io::read_stl(&mut file)?;

    let vertices = stl
        .vertices
        .into_iter()
        .map(|v| Vertex::new(v[0], v[1], v[2]))
        .collect();

    let faces = stl
        .faces
        .into_iter()
        .map(
            |IndexedTriangle {
                 vertices: [a, b, c],
                 ..
             }| Face::new3(a, b, c),
        )
        .collect();

    Ok(Mesh::from_vertices(vertices, faces, Color::default()))
}
