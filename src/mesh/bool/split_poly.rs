use crate::mesh::normals::calculate_normal;
use crate::mesh::parts::edge::{MeshEdge, Edge};
use crate::mesh::parts::polygon::Polygon;
use crate::mesh::parts::vertex::Vertex;
use crate::mesh::{MeshError, MeshResult};
use glam::Vec3;
use std::cmp::Ordering;
use std::hash::Hash;
use std::vec;

pub(crate) fn split_polygons(lhs: &Polygon, rhs: &Polygon) -> MeshResult<Vec<Polygon>> {
    let mut inter_points = vec![];

    for e1 in lhs.edges().iter() {
        for e2 in rhs.edges().iter() {
            if let Some(Edge{a:v1, b:v2}) = e1.find_collinear_segment(e2) {
            } else if let Some(v1) = e1.find_intersection(e2) {
                inter_points.push(v1);
            }
        }
    }

    if inter_points.is_empty() {
        Ok(vec![lhs.clone(), rhs.clone()])
    } else {
        let inter_segments = sort_inter_points(inter_points);
        let mut res_poly = vec![];
        res_poly.extend(split_at(lhs, inter_segments.clone())?);
        res_poly.extend(split_at(rhs, inter_segments.clone())?);
        Ok(res_poly)
    }
}

fn sort_inter_points(inter_points: Vec<Vertex>) -> Vec<Edge> {
    if inter_points.len() < 2 {
        vec![]
    } else {
        let normal = calculate_normal(&inter_points);
        println!("Normal = {:?}", normal);
        println!("inter_points = {:?}", inter_points);
        let points_2d: Vec<(f32, f32)> = inter_points
            .iter()
            .map(|v| {
                let v: Vec3 = v.into();
                let projected = v - normal * v.dot(normal);
                (projected.x, projected.y)
            })
            .collect();

        let mut sorted_points_2d = points_2d.clone();
        sorted_points_2d.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        println!("2d = > {:?}", &sorted_points_2d);

        let mut sorted_points_3d = Vec::new();
        for point_2d in sorted_points_2d {
            for point_3d in &inter_points {
                let point_3d_temp:Vec3 = point_3d.into();
                let projected = point_3d_temp - normal * point_3d_temp.dot(normal);
                if (projected.x, projected.y) == point_2d {
                    sorted_points_3d.push(*point_3d);
                    break;
                }
            }
        }
        println!("3d = > {:?}", sorted_points_3d);
        (0..sorted_points_3d.len() - 1)
            .step_by(2)
            .map(|i| Edge::new(sorted_points_3d[i], sorted_points_3d[i + 1]))
            .collect()
    }
}

fn split_at(poly: &Polygon, segments: Vec<Edge>) -> MeshResult<Vec<Polygon>> {
    let mut sub_polygons = vec![];
    for segment in segments {
        for triangle in poly.triangulate() {
            sub_polygons.extend(from_segment(&triangle, segment)?);
        }
    }

    Ok(sub_polygons)
}

fn from_segment(triangle: &Polygon, segment: Edge) -> MeshResult<Vec<Polygon>> {
    let vertices = triangle.vertices();
    let (v1, v2, v3) = (vertices[0], vertices[1], vertices[2]);
    let Edge { a: s1, b: s2 } = segment;

    let mut shared = vec![];

    for vertex in &[v1, v2, v3] {
        if vertex.eq(&s1) || vertex.eq(&s2) {
            shared.push(*vertex);
        }
    }

    match shared.len() {
        2 => {
            let other = *[v1, v2, v3]
                .iter()
                .find(|&&v| !shared.contains(&v))
                .ok_or(MeshError::Custom("No other vertex found".to_string()))?;
            Ok(vec![Polygon::take(vec![shared[0], shared[1], other])])
        }
        1 => {
            let mut remaining_vertices = Vec::with_capacity(2);
            for &v in &[v1, v2, v3] {
                if v != shared[0] {
                    remaining_vertices.push(v);
                }
            }

            if remaining_vertices.len() != 2 {
                Err(MeshError::Custom("Found more then 2 vertices".to_string()))
            } else {
                Ok(vec![
                    Polygon::take(vec![shared[0], s1, remaining_vertices[0]]),
                    Polygon::take(vec![shared[0], s2, remaining_vertices[1]]),
                ])
            }
        }
        _ => Ok(vec![
            Polygon::take(vec![v1, s1, s2]),
            Polygon::take(vec![v2, s1, s2]),
            Polygon::take(vec![v3, s1, s2]),
        ]),
    }
}

#[cfg(test)]
mod tests {}
