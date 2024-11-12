use std::hash::Hash;
use crate::mesh::material::Material;

#[derive(Debug, Default, Clone)]
pub struct Attributes {
    mesh_type: MeshType,
    material: Material
}

impl Attributes {
    pub fn new(mesh_type: MeshType) -> Self {
        Attributes { mesh_type, material: Default::default() }
    }

    pub fn new_with_material(mesh_type: MeshType, material: Material) -> Self {
        Attributes { mesh_type, material }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn mesh_type(&self) -> MeshType {
        self.mesh_type.clone()
    }


}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MeshType {
    Polygons,
    Lines,
    Cloud(usize),
}



impl MeshType {
    pub fn is_polygons(&self) -> bool {
        match self {
            MeshType::Polygons => true,
            _ => false,
        }
    }

    pub fn is_cloud(&self) -> bool {
        match self {
            MeshType::Cloud(_) => true,
            _ => false,
        }
    }
    pub fn is_lines(&self) -> bool {
        match self {
            MeshType::Lines => true,
            _ => false,
        }
    }
}

impl Default for MeshType {
    fn default() -> Self {
        MeshType::Polygons
    }
}
