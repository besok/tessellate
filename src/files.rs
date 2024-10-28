pub mod obj;
pub mod ply;

use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{Mesh, MeshError};
use log::info;
use std::fs::File;
use std::io;
use std::io::BufReader;
use ply_rs::parser::Parser;
use ply_rs::ply::DefaultElement;
pub use tobj::{load_obj, LoadError};

#[derive(Debug)]
pub enum FileError {
    ImportObjError(LoadError),
    FileError(io::Error),
    MeshError(MeshError),
}

impl From<LoadError> for FileError {
    fn from(e: LoadError) -> Self {
        FileError::ImportObjError(e)
    }
}

impl From<io::Error> for FileError {
    fn from(e: io::Error) -> Self {
        FileError::FileError(e)
    }
}

impl From<MeshError> for FileError {
    fn from(e: MeshError) -> Self {
        FileError::MeshError(e)
    }
}
