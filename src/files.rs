use std::io;

pub use tobj::{load_obj, LoadError};

use crate::mesh::MeshError;

pub mod obj;
pub mod ply;
pub mod stl;

#[derive(Debug)]
pub enum FileError {
    ImportObjError(LoadError),
    FileError(io::Error),
    MeshError(MeshError),
    Custom(String),
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
