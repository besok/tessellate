use tobj::load_obj;
use log::info;
use crate::files::FileError;
use crate::mesh::{Mesh, MeshError};
use crate::mesh::attributes::Attributes;
use crate::mesh::material::Color;
use crate::mesh::parts::face::Face;
use crate::mesh::parts::vertex::Vertex;

/// Imports an OBJ file and converts it into a vector of `Mesh` objects.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the OBJ file.
/// * `options` - A reference to `tobj::LoadOptions` that specifies the loading options.
///
/// # Returns
///
/// A `Result` containing a vector of `Mesh` objects on success, or a `FileError` on failure.
///
/// # Errors
///
/// This function will return a `FileError` if the OBJ file cannot be loaded or if there is an error
/// in converting the loaded data into `Mesh` objects.
///
/// # Examples
///
/// ```no_run
/// use tessellate::files::obj::import_objs;
/// let options = tobj::LoadOptions::default();
/// let meshes = import_objs("path/to/your.obj", &options)?;
/// ```
pub fn import_objs(path: &str, options: &tobj::LoadOptions) -> Result<Vec<Mesh>, FileError> {
    let mut meshes = vec![];
    let (models, _mb_materials) = load_obj(&path, options)?;
    for model in models {
        info!("Importing a model: {:?}", model.name);

        let mesh = &model.mesh;
        let vertices = mesh
            .positions
            .chunks(3)
            .map(|e| Vertex::new(e[0], e[1], e[2]))
            .collect::<Vec<_>>();

        let mut faces = Vec::new();
        let mut next_face = 0;
        if mesh.face_arities.is_empty() {
            faces = mesh
                .indices
                .chunks(3)
                .map(|e| Face::new3(e[0] as usize, e[1] as usize, e[2] as usize))
                .collect::<Vec<_>>();
        } else {
            for face in 0..mesh.face_arities.len() {
                let end = next_face + mesh.face_arities[face] as usize;

                let face_indices = &mesh.indices[next_face..end]
                    .iter()
                    .map(|&i| i as usize)
                    .collect::<Vec<_>>();

                let local_faces = Face::new(face_indices.clone())?;

                faces.extend(local_faces);
                next_face = end;
            }
        }

        meshes.push(Mesh::from_vertices(vertices, faces, Attributes::default()));
    }

    Ok(meshes)
}

/// Imports a single OBJ file and converts it into a `Mesh` object.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the OBJ file.
/// * `options` - A reference to `tobj::LoadOptions` that specifies the loading options.
///
/// # Returns
///
/// A `Result` containing a `Mesh` object on success, or a `FileError` on failure.
///
/// # Errors
///
/// This function will return a `FileError` if the OBJ file cannot be loaded or if there is an error
/// in converting the loaded data into a `Mesh` object.
///
/// # Examples
///
/// ```rust
///
/// use tessellate::files::obj::import_obj;
/// let options = tobj::LoadOptions::default();
/// let mesh = import_obj("path/to/your.obj", &options)?;
/// ```
pub fn import_obj(path: &str, options: &tobj::LoadOptions) -> Result<Mesh, FileError> {
    let meshes = import_objs(path, options)?;
    Ok(meshes
        .into_iter()
        .next()
        .ok_or(FileError::MeshError(MeshError::Custom("No mesh found".to_string())))?)
}