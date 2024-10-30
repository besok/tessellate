use ply_rs::parser::Parser;
use ply_rs::ply::{Property, PropertyAccess};

use crate::files::FileError;
use crate::mesh::material::Color;
use crate::mesh::Mesh;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;

/// Imports a PLY file and converts it into a `Mesh` object.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the PLY file.
///
/// # Returns
///
/// * `Result<Mesh, FileError>` - On success, returns a `Mesh` object. On failure, returns a `FileError`.
///
/// # Errors
///
/// This function will return an error if the file cannot be opened, read, or if the PLY data cannot be parsed correctly.
pub fn import_ply(path: &str) -> Result<Mesh, FileError> {
    let mut f = std::io::BufReader::new(std::fs::File::open(path)?);
    let vertex_parser =  Parser::<PlyVertex>::new();
    let face_parser =  Parser::<PlyFace>::new();
    let header = vertex_parser.read_header(&mut f)?;

    let mut vertex_list = Vec::new();
    let mut face_list = Vec::new();
    for (_ignore_key, element) in &header.elements {

        match element.name.as_ref() {
            "vertex" => {vertex_list = vertex_parser.read_payload_for_element(&mut f, &element, &header).unwrap();},
            "face" => {face_list = face_parser.read_payload_for_element(&mut f, &element, &header).unwrap();},
            _ => {},
        }
    }

    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    for ply_vertex in vertex_list {
        match ply_vertex {
            PlyVertex::Vertex(x, y, z) => vertices.push(Vertex::new(x, y, z)),
            e => return Err(FileError::Custom(format!("Failed to parse vertex {:?}",e))),
        }
    }

    for ply_face in face_list {
        match ply_face {
            PlyFace::Face(v) => faces.extend(Face::new(v)?),
            e => return Err(FileError::Custom(format!("Failed to parse face {:?}",e))),
        }

    }

    Ok(Mesh::from_vertices(vertices, faces, Color::default()))
}

#[derive(Debug)]
enum PlyVertex {
    New,
    Vertex(f32, f32, f32),
    Failed(String),
}

#[derive(Debug)]
enum PlyFace {
    New,
    Face(Vec<i32>),
    Failed(String),
}

impl PropertyAccess for PlyFace {
    fn new() -> Self {
        PlyFace::New
    }
    fn set_property(&mut self, key: String, property: Property) {
        match (key.as_ref(), property) {
            ("vertex_indices", Property::ListInt(v)) => match self {
                PlyFace::New => *self = PlyFace::Face(v),
                _ => *self = PlyFace::Failed("Unexpected key/value combination".to_string()),
            },
            (k, _) => {}//*self = PlyFace::Failed(format!("Unexpected key: {}", k)),
        }
    }
}
impl PropertyAccess for PlyVertex {
    fn new() -> Self {
        PlyVertex::New
    }
    fn set_property(&mut self, key: String, property: Property) {
        match (key.as_ref(), property) {
            ("x", Property::Float(v)) => match self {
                PlyVertex::New => *self = PlyVertex::Vertex(v, 0.0, 0.0),
                PlyVertex::Vertex(_, y, z) => *self = PlyVertex::Vertex(v, *y, *z),
                _ => *self = PlyVertex::Failed("Unexpected key/value combination".to_string()),
            },
            ("y", Property::Float(v)) => match self {
                PlyVertex::Vertex(x, _, z) => *self = PlyVertex::Vertex(*x, v, *z),
                _ => *self = PlyVertex::Failed("Unexpected key/value combination".to_string()),
            },
            ("z", Property::Float(v)) => match self {
                PlyVertex::Vertex(x, y, _) => *self = PlyVertex::Vertex(*x, *y, v),
                _ => *self = PlyVertex::Failed("Unexpected key/value combination".to_string()),
            },

            (k, _) => {} // *self = PlyVertex::Failed(format!("Unexpected key: {}", k)),
        }
    }
}
