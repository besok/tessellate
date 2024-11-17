use crate::mesh::material::Material;
use std::hash::Hash;

#[derive(Debug,Clone)]
pub struct Attributes {
    mesh_type: MeshType,
    material: Material,
    affected_by_light: bool,
}

impl Default for Attributes {
    fn default() -> Self {
        Attributes {
            mesh_type: Default::default(),
            material: Default::default(),
            affected_by_light: true,
        }
    }
}

impl Attributes {
    pub fn new(mesh_type: MeshType) -> Self {
        Attributes {
            mesh_type,
            material: Default::default(),
            affected_by_light: true,
        }
    }

    pub fn new_with_material(mesh_type: MeshType, material: Material) -> Self {
        Attributes {
            mesh_type,
            material,
            affected_by_light: true,
        }
    }

    pub fn new_with_material_and_light(
        mesh_type: MeshType,
        material: Material,
        affected_by_light: bool,
    ) -> Self {
        Attributes {
            mesh_type,
            material,
            affected_by_light,
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_affected_by_light(&mut self, affected_by_light: bool) -> &Self {
        self.affected_by_light = affected_by_light;
        self
    }

    pub fn mesh_type(&self) -> MeshType {
        self.mesh_type.clone()
    }

    pub fn material(&self) -> Material {
        self.material.clone()
    }

    pub fn affected_by_light(&self) -> bool {
        self.affected_by_light
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
