use std::hash::Hash;

#[derive(Debug, Default, Clone)]
pub struct Attributes {
    mesh_type: MeshType,
}

impl Attributes {
    pub fn new(mesh_type: MeshType) -> Self {
        Attributes { mesh_type }
    }

    pub fn mesh_type(&self) -> MeshType {
        self.mesh_type.clone()
    }


}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MeshType {
    Polygons,
    Cloud,
}

impl Hash for MeshType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            MeshType::Polygons => "Polygons".hash(state),
            MeshType::Cloud => "Cloud".hash(state),
        }
    }
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
            MeshType::Cloud => true,
            _ => false,
        }
    }
}

impl Default for MeshType {
    fn default() -> Self {
        MeshType::Polygons
    }
}
