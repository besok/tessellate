use crate::mesh::material::{Color, Material, RgbaColor};
use std::hash::Hash;

/// Represents the attributes of a mesh.
#[derive(Debug, Clone)]
pub struct Attributes {
    mesh_type: MeshType,
    material: Material,
    color: Color,
    affected_by_light: bool,
}

impl Default for Attributes {
    fn default() -> Self {
        Attributes {
            mesh_type: Default::default(),
            material: Default::default(),
            color: Color::default(),
            affected_by_light: true,
        }
    }
}

impl Attributes {
    pub fn new(mesh_type: MeshType) -> Self {
        Attributes {
            mesh_type,
            color: Color::default(),
            material: Default::default(),
            affected_by_light: true,
        }
    }

    pub fn new_with_material(mesh_type: MeshType, material: Material, color: Color) -> Self {
        Attributes {
            mesh_type,
            material,
            color,
            affected_by_light: true,
        }
    }

    pub fn new_with_material_and_light(
        mesh_type: MeshType,
        material: Material,
        affected_by_light: bool,
        color: Color,
    ) -> Self {
        Attributes {
            mesh_type,
            material,
            affected_by_light,
            color,
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

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_mesh_type(&mut self, mesh_type: MeshType) {
        self.mesh_type = mesh_type;
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

impl From<Color> for Attributes {
    fn from(color: Color) -> Self {
        Attributes {
            color,
            mesh_type: Default::default(),
            material: Default::default(),
            affected_by_light: true,
        }
    }
}


impl From<RgbaColor> for Attributes {
    fn from(color: RgbaColor) -> Self {
        Attributes {
            color: color.into(),
            mesh_type: Default::default(),
            material: Default::default(),
            affected_by_light: true,
        }
    }
}