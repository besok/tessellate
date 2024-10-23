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

#[derive(Debug, Clone)]
pub enum MeshType {
    Polygons,
    Cloud(f32),
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
}

impl Default for MeshType {
    fn default() -> Self {
        MeshType::Polygons
    }
}
