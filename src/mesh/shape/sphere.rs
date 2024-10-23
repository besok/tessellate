use crate::mesh::parts::face::Face;
use crate::mesh::shape::icosahedron::Icosahedron;
use crate::mesh::HasMesh;
use crate::mesh::Mesh;
use std::f32::consts::PI;
use std::ops::Deref;
use crate::mesh::material::Color;
use crate::mesh::parts::vertex::Vertex;

#[derive(Debug, Clone)]
pub struct Sphere {
    radius: f32,
    center: Vertex,
    segments: usize,
    mesh: Mesh,
}
impl Deref for Sphere {
    type Target = Mesh;

    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}
impl HasMesh for Sphere {
    fn mesh(&self) -> &Mesh {
        &self.mesh
    }
    fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }
}

impl Sphere {
    pub fn create_uv<V,C>(center: V, radius: f32, m: usize, n: usize, color: C) -> Self
    where V: Into<Vertex>,
          C: Into<Color>
    {
        let center = center.into();
        let color = color.into();
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for i in 0..=m {
            let theta = i as f32 * PI / m as f32;
            for j in 0..n {
                let phi = j as f32 * 2.0 * PI / n as f32;
                let v = calc_vertex(radius, theta, phi);
                vertices.push(v + center);
            }
        }
        for i in 0..m {
            for j in 0..n {
                let next_j = (j + 1) % n;
                faces.push(Face::Triangle(
                    i * n + j,
                    (i + 1) * n + j,
                    i * n + next_j,
                ));
                faces.push(Face::Triangle(
                    i * n + next_j,
                    (i + 1) * n + j,
                    (i + 1) * n + next_j,
                ));
            }
        }

        Sphere {
            radius,
            center,
            segments: n * m,
            mesh: Mesh::from_vertices(vertices, faces,color),
        }
    }
    pub fn create_ico<V: Into<Vertex>>(center: V, radius: f32, subdivisions: usize, color: Color) -> Self {
        let center = center.into();
        let mut ico = Icosahedron::create(center, radius,color);
        let mesh = (0..subdivisions).fold(ico.mesh_mut(), |acc, _| {
            let _ = acc.subdivide();
            acc
        });

        Sphere {
            radius,
            center,
            segments: subdivisions,
            mesh:mesh.clone(),
        }
    }

    pub fn create<V: Into<Vertex>>(center: V, radius: f32,color:Color) -> Self {
        Sphere::create_uv(center, radius, 32, 32,color)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::create_uv(Vertex::default(), 1.0, 32, 32, Color::default())
    }
}

pub fn calc_vertex(r: f32, theta: f32, phi: f32) -> Vertex {
    let (theta_sin, theta_cos) = theta.sin_cos();
    let (phi_sin, phi_cos) = phi.sin_cos();

    Vertex::new(r * phi_sin * theta_cos, r * phi_cos, r * phi_sin * theta_sin)
}
