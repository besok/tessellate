use glam::Vec3;
use std::ops::{Add, Div, Mul, Sub};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn get(depth: usize) -> Axis {
        match depth % 3 {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => unreachable!("Invalid axis index"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

impl From<Vec3> for Vertex {
    fn from(value: Vec3) -> Self {
        Vertex {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl Into<Vec3> for Vertex {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Into<Vec3> for &Vertex {
    fn into(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Mul<f32> for Vertex {
    type Output = Vertex;

    fn mul(self, rhs: f32) -> Self::Output {
        Vertex {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<Vertex> for Vertex {
    type Output = Vertex;

    fn mul(self, rhs: Vertex) -> Self::Output {
        Vertex {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Add for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Self) -> Self::Output {
        Vertex {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vertex {
    type Output = Vertex;

    fn sub(self, rhs: Self) -> Self::Output {
        Vertex {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div for Vertex {
    type Output = Vertex;

    fn div(self, rhs: Self) -> Self::Output {
        Vertex {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex::new(0.0, 0.0, 0.0)
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex { x, y, z }
    }
    pub fn flatten(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
    pub fn normalize(&self) -> Vertex {
        <&Vertex as Into<Vec3>>::into(&self).normalize().into()
    }
    pub fn distance(&self, other: &Vertex) -> f32 {
        self.flatten()
            .iter()
            .zip(other.flatten().iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    pub fn get(&self, axis: &Axis) -> f32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }

    pub fn set(&mut self, axis: &Axis, value: f32) {
        match axis {
            Axis::X => self.x = value,
            Axis::Y => self.y = value,
            Axis::Z => self.z = value,
        }
    }

    pub fn cross(&self, other: &Vertex) -> Vertex {
        let a:Vec3 = self.into();
        let b:Vec3 = other.into();
        a.cross(b).into()
    }

    pub fn dot(&self, other: &Vertex) -> f32 {
        let a:Vec3 = self.into();
        let b:Vec3 = other.into();
        a.dot(b)
    }

    pub fn magnitude(&self) -> f32 {
        self.flatten().iter().map(|v| v.powi(2)).sum::<f32>().sqrt()
    }
}

impl From<[f32; 3]> for Vertex {
    fn from(value: [f32; 3]) -> Self {
        Vertex {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<[i32; 3]> for Vertex {
    fn from(value: [i32; 3]) -> Self {
        Vertex {
            x: value[0] as f32,
            y: value[1] as f32,
            z: value[2] as f32,
        }
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}