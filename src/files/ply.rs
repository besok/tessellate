use ply_rs::parser::Parser;
use ply_rs::ply::{DefaultElement, PropertyAccess};

use crate::files::FileError;
use crate::mesh::Mesh;

pub fn import_ply(path: &str) -> Result<Mesh, FileError> {
    let mut f = std::io::BufReader::new(std::fs::File::open(path)?);
    let p = Parser::<DefaultElement>::new();
    let ply = p.read_ply(&mut f)?;
    ply.payload.iter().for_each(|(name,elems)| {
        println!("Element: {}", name);
        elems.iter().for_each(|p| {
            println!("Property: {:?}", p);
        });
    });
    Ok(Mesh::default())
    // Ok(Mesh::from_vertices(vertex_list, face_list, Color::default()))
}
