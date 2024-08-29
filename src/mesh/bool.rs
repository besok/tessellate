mod split_poly;

use crate::mesh::bool::split_poly::split_polygons;
use crate::mesh::material::Color;
use crate::mesh::parts::polygon::Polygon;
use crate::mesh::parts::vertex::{Axis, Vertex};
use crate::mesh::query::MeshQuery;
use crate::mesh::shape::beam::Beam;
use crate::mesh::{Mesh, MeshError, MeshResult};
use std::collections::HashSet;

pub enum BoolType {
    Union,
    Intersection,
    Difference,
}

pub fn perform_bool(
    mesh_a: &Mesh,
    mesh_b: &Mesh,
    op: BoolType,
    depth: Option<usize>,
    color: Option<Color>,
) -> MeshResult<Mesh> {
    let props_a = mesh_a.props();
    let props_b = mesh_b.props();
    if !props_a.is_volume()? || !props_b.is_volume()? {
        println!("props_a has self: {:?}", props_a.has_self_intersections());
        println!("props_b has self: {:?}", props_b.has_self_intersections());
        Err(MeshError::WrongMesh("Meshes must be volumes".to_string()))
    } else {
        let a_tree = MeshQuery::new(mesh_a).try_octree(depth)?;
        let b_tree = MeshQuery::new(mesh_b).try_octree(depth)?;

        let mut inter_polygons = Vec::new();
        let mut non_inter_polygons_a = HashSet::new();
        let mut non_inter_polygons_b = HashSet::new();

        for a_leaf in a_tree.iter_leafs() {
            for b_leaf in b_tree.iter_leafs() {
                if a_leaf.is_overlapping(b_leaf) {
                    let a_polygons = a_leaf.polygons();
                    let b_polygons = b_leaf.polygons();
                    for a_poly in a_polygons.iter() {
                        for b_poly in b_polygons.iter() {
                            if a_poly.intersects(b_poly)? {
                                inter_polygons.extend(split_polygons(a_poly, b_poly)?)
                            } else {
                                non_inter_polygons_a.insert(a_poly.clone());
                                non_inter_polygons_b.insert(b_poly.clone());
                            }
                        }
                    }
                }
            }
        }

        let mut final_polygons = Vec::new();

        let max_coord_a = a_tree.max_coords();
        let max_coord_b = b_tree.max_coords();

        match op {
            BoolType::Union => {
                final_polygons.extend(non_inter_polygons_a);
                final_polygons.extend(non_inter_polygons_b);
                final_polygons.extend(inter_polygons);
            }
            BoolType::Intersection => {
                for p in inter_polygons.iter() {
                    if is_inside(p, max_coord_a, mesh_b)? && is_inside(p, max_coord_b, mesh_a)? {
                        final_polygons.push(p.clone());
                    }
                }
            }
            BoolType::Difference => {
                final_polygons.extend(non_inter_polygons_a);
                for p in inter_polygons.iter() {
                    if is_outside(p, max_coord_a, mesh_b)? {
                        final_polygons.push(p.clone());
                        final_polygons.push(p.clone());
                    }
                }
            }
        }
        let color = color.unwrap_or(mesh_a.color().clone());
        Ok(reconstruct(final_polygons, color)?)
    }
}

fn is_outside(p: &Polygon, max_coord_p: &Vertex, mesh: &Mesh) -> MeshResult<bool> {
    Ok(!is_inside(p, max_coord_p, mesh)?)
}
fn is_inside(p: &Polygon, max_coord_p: &Vertex, mesh: &Mesh) -> MeshResult<bool> {
    let ray = Ray::from_poly(p, max_coord_p)?;

    let intersections = mesh
        .try_polygons()?
        .into_iter()
        .flat_map(|poly| poly.triangulate())
        .filter(|poly| ray_intersects_triangle(&ray, poly))
        .count();

    Ok(intersections % 2 == 1)
}

enum PLocation {
    Above,
    Below,
    On,
}
fn point_on_poly(p: &Vertex, a: &Vertex, b: &Vertex, c: &Vertex) -> PLocation {
    let vol = (a.x - p.x) * ((b.y - p.y) * (c.z - p.z) - (b.z - p.z) * (c.y - p.y))
        - (a.y - p.y) * ((b.x - p.x) * (c.z - p.z) - (b.z - p.z) * (c.x - p.x))
        + (a.z - p.z) * ((b.x - p.x) * (c.y - p.y) - (b.y - p.y) * (c.x - p.x));

    if vol > 0.0 {
        PLocation::Above
    } else if vol < 0.0 {
        PLocation::Below
    } else {
        PLocation::On
    }
}

fn ray_intersects_triangle(ray: &Ray, triangle: &Polygon) -> bool {
    let vs = triangle.vertices();
    let tv0 = vs[0];
    let tv1 = vs[1];
    let tv2 = vs[2];
    let orf = point_on_poly(&ray.origin, &tv0, &tv1, &tv2);
    let ors = point_on_poly(&ray.direction, &tv0, &tv1, &tv2);

    match (orf, ors) {
        (PLocation::Above, PLocation::Below) | (PLocation::Below, PLocation::Above) => {
            let or01f = point_on_poly(&tv0, &tv1, &ray.origin, &ray.direction);
            let or12f = point_on_poly(&tv1, &tv2, &ray.origin, &ray.direction);
            let or20f = point_on_poly(&tv2, &tv0, &ray.origin, &ray.direction);
            match (or01f, or12f, or20f) {
                (PLocation::Above, PLocation::Above, PLocation::Above) => true,
                (PLocation::Below, PLocation::Below, PLocation::Below) => true,
                _ => false,
            }
        }
        _ => false,
    }
}

fn reconstruct(polygon: Vec<Polygon>, color: Color) -> MeshResult<Mesh> {
    Ok(Mesh::from_polygons(polygon, color))
}

pub struct Ray {
    origin: Vertex,
    direction: Vertex,
}

impl Ray {
    pub fn new(origin: Vertex, direction: Vertex) -> Self {
        Self { origin, direction }
    }

    pub fn to_beam<C: Into<Color>>(&self, diam: f32, color: C) -> Beam {
        Beam::create(self.origin, self.direction, diam, color)
    }

    pub fn from_poly(p: &Polygon, max_coords: &Vertex) -> MeshResult<Self> {
        let centroid = p.centroid()?;
        match dominant_normal_component(p.normal()) {
            Axis::X => Ok(Ray::new(centroid, Vertex::new(max_coords.x, centroid.y, centroid.z))),
            Axis::Y => Ok(Ray::new(centroid, Vertex::new(centroid.x, max_coords.y, centroid.z))),
            Axis::Z => Ok(Ray::new(centroid, Vertex::new(centroid.x, centroid.y, max_coords.z))),
        }
    }
}

fn dominant_normal_component(normal: Vertex) -> Axis {
    let abs_x = normal.x.abs();
    let abs_y = normal.y.abs();
    let abs_z = normal.z.abs();

    if abs_x > abs_y && abs_x > abs_z {
        Axis::X
    } else if abs_y > abs_x && abs_y > abs_z {
        Axis::Y
    } else {
        Axis::Z
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::bool::{ray_intersects_triangle, Ray};
    use crate::mesh::Vertex;
    use crate::mesh::{Mesh, Polygon};
    use crate::poly;
    use crate::v;

    #[test]
    fn ray_intersects_triangle_test() {
        let p1 = poly!(1, 0, 0; 0, 1, 0; 0, 0, 1);
        let p2 = poly!(1, 0, 0; 0, 2, 0; 1, 1, 1);

        let r = ray_intersects_triangle(&Ray::from_poly(&p1, &v!(1, 1, 1)).unwrap(), &p2);
        assert_eq!(r, true);
    }
}
