use std::fs::OpenOptions;
use stl_io::IndexedTriangle;

use crate::files::FileError;
use crate::mesh::material::Color;
use crate::mesh::Mesh;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;

/// Imports an STL file and converts it into a `Mesh`.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the STL file.
///
/// # Returns
///
/// * `Result<Mesh, FileError>` - Returns a `Mesh` on success, or a `FileError` on failure.
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
