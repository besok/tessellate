use crate::mesh::material::Color;
use crate::mesh::parts::polygon::Polygon;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::query::MeshQuery;
use crate::mesh::{Mesh, MeshResult};

pub enum BoolType {
    Union,
    Intersection,
    Difference,
}

impl BoolType {
    pub fn is_union(&self) -> bool {
        match self {
            BoolType::Union => true,
            _ => false,
        }
    }
}

pub fn perform_bool(
    mesh_a: &Mesh,
    mesh_b: &Mesh,
    op: BoolType,
    depth: Option<usize>,
    color: Option<Color>
) -> MeshResult<Mesh> {
    let a_tree = MeshQuery::new(mesh_a).try_octree(depth)?;
    let b_tree = MeshQuery::new(mesh_b).try_octree(depth)?;

    let mut inter_polygons = Vec::new();
    let mut non_inter_polygons_a = Vec::new();
    let mut non_inter_polygons_b = Vec::new();

    for a_leaf in a_tree.iter_leafs() {
        for b_leaf in b_tree.iter_leafs() {
            if a_leaf.is_overlapping(b_leaf) {
                let a_polygons = a_leaf.polygons();
                let b_polygons = b_leaf.polygons();
                for a_poly in a_polygons.iter() {
                    let mut already = false;
                    for b_poly in b_polygons.iter() {
                        if a_poly.intersects(b_poly)? {
                            let split_polygons: Vec<Polygon> = split_polygons(a_poly, b_poly)?;
                            inter_polygons.extend(split_polygons)
                        } else {
                            if !already {
                                non_inter_polygons_a.push(a_poly.clone());
                                already = true;
                            }
                            non_inter_polygons_b.push(b_poly.clone());
                        }
                    }
                }
            }
        }
    }

    let mut final_polygons = Vec::new();

    match op {
        BoolType::Union => {
            final_polygons.extend(non_inter_polygons_a);
            final_polygons.extend(non_inter_polygons_b);
            final_polygons.extend(inter_polygons);
        }
        BoolType::Intersection => {
            for p in inter_polygons.iter() {
                if is_inside(p, mesh_b)? && is_inside(p, mesh_a)? {
                    final_polygons.push(p.clone());
                }
            }
        }
        BoolType::Difference => {
            final_polygons.extend(non_inter_polygons_a);
            for p in inter_polygons.iter() {
                if is_outside(p, mesh_b)? {
                    final_polygons.push(p.clone());
                }
            }
        }
    }
    let color = color.unwrap_or(mesh_a.color().clone());
    Ok(reconstruct(final_polygons, color)?)
}

fn split_polygons(lhs: &Polygon, rhs: &Polygon) -> MeshResult<Vec<Polygon>> {
    Ok(vec![lhs.clone(), rhs.clone()])
}

fn is_outside(p: &Polygon, mesh: &Mesh) -> MeshResult<bool> {
    Ok(!is_inside(p, mesh)?)
}
fn is_inside(p: &Polygon, mesh: &Mesh) -> MeshResult<bool> {
    let centroid = p.centroid()?;
    let ray_direction = Vertex::new(1.0, 0.0, 0.0); // Arbitrary direction

    let intersections = mesh
        .try_polygons()?
        .into_iter()
        .filter(|poly| ray_intersects_triangle(&centroid, &ray_direction, poly))
        .count();

    Ok(intersections % 2 == 1)
}
/// Möller–Trumbore intersection algorithm
fn ray_intersects_triangle(origin: &Vertex, direction: &Vertex, triangle: &Polygon) -> bool {
    let epsilon = 1e-8;
    let vertices = triangle.vertices();
    let v0 = vertices[0];
    let v1 = vertices[1];
    let v2 = vertices[2];

    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = direction.cross(&edge2);
    let a = edge1.dot(&h);

    if a > -epsilon && a < epsilon {
        return false; // This ray is parallel to this triangle.
    }

    let f = 1.0 / a;
    let s = *origin - v0;
    let u = f * s.dot(&h);

    if u < 0.0 || u > 1.0 {
        return false;
    }

    let q = s.cross(&edge1);
    let v = f * direction.dot(&q);

    if v < 0.0 || u + v > 1.0 {
        return false;
    }

    (f * edge2.dot(&q)) > epsilon
}

fn reconstruct(polygon: Vec<Polygon>, color: Color) -> MeshResult<Mesh> {
    Ok(Mesh::from_polygons(polygon, color))
}
