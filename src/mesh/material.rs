use crate::mesh::parts::vertex::Vertex;
use glam::Vec3;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Material {
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
    shininess: f32,
}

impl Material {
    pub fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, shininess: f32) -> Self {
        Material {
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    pub fn ambient(&self) -> Vec3 {
        self.ambient
    }

    pub fn diffuse(&self) -> Vec3 {
        self.diffuse
    }

    pub fn specular(&self) -> Vec3 {
        self.specular
    }

    pub fn shininess(&self) -> f32 {
        self.shininess
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            ambient: Vec3::new(0.1, 0.1, 0.1),
            diffuse: Vec3::new(0.7, 0.7, 0.7),
            specular: Vec3::new(1.0, 1.0, 1.0),
            shininess: 32.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RgbaColor(pub [u8; 4]);
impl From<RgbaColor> for [f32; 4] {
    fn from(value: RgbaColor) -> Self {
        let RgbaColor([r, g, b, a]) = value;
        [
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        ]
    }
}

impl Default for RgbaColor {
    fn default() -> Self {
        Self::random()
    }
}

impl From<[u8; 4]> for RgbaColor {
    fn from(color: [u8; 4]) -> Self {
        RgbaColor(color)
    }
}

impl RgbaColor {
    pub const BLACK: Self = Self([0, 0, 0, 255]);
    pub const WHITE: Self = Self([255, 255, 255, 255]);

    pub const GRAY: Self = Self([128, 128, 128, 255]);

    pub const RED: Self = Self([255, 0, 0, 255]);
    pub const GREEN: Self = Self([0, 255, 0, 255]);
    pub const BLUE: Self = Self([0, 0, 255, 255]);

    pub const YELLOW: Self = Self([255, 255, 0, 255]);

    pub const MAGENTA: Self = Self([255, 0, 255, 255]);

    pub const CYAN: Self = Self([0, 255, 255, 255]);
    pub const TRANSPARENT: Self = Self([0, 0, 0, 0]);

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b, 255])
    }

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let color = [
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            255,
        ];
        RgbaColor(color)
    }
}

#[derive(Clone, Debug)]
pub enum Color {
    Func(fn(&Vertex, usize) -> RgbaColor),
    Vertex(Vec<RgbaColor>),
    Face(Vec<RgbaColor>),
    Line(Vec<RgbaColor>),
    Mesh(RgbaColor),
}

impl Color {
    pub fn v_random() -> Self {
        Self::Func(|_, _| RgbaColor::random())
    }
}

impl From<RgbaColor> for Color {
    fn from(value: RgbaColor) -> Self {
        Self::Mesh(value)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Mesh(RgbaColor::random())
    }
}
