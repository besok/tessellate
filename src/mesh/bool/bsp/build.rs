///
/// Pseudocode for creating a 3D BSP tree:
///
/// 1. Define BSPTree and BSPNode Structures
/// - `BSPTree` contains a root node.
/// - `BSPNode` can be a `Leaf` containing polygons
///     or a `Node` containing a dividing plane, front, and back child nodes.
///
/// 2. Start Building the Tree
/// - Input: A list of polygons representing the 3D model.
/// - Output: A `BSPTree` with a structured hierarchy of nodes.
///
/// 3. BSPTreeBuilder Functionality
/// - Initialize the tree builder with the list of polygons.
/// - Call the `build` method to start constructing the tree.
///
/// 4. Build Method
/// - If the list of polygons is empty or meets a specific condition (e.g., below a certain count),
///     create a `Leaf` node with those polygons.
/// - Otherwise, choose a dividing plane. This can be based on various heuristics,
///     such as the polygon that minimizes splits, covers the most space, etc.
/// - Split polygons relative to the chosen plane into three groups: front, back, and coplanar.
/// - Coplanar polygons are added directly to the current node.
/// - Recursively call `build` for the front and back lists of polygons to create child nodes.
/// - Return a `Node` containing the dividing plane, front child, and back child.
///
/// 5. Choosing a Dividing Plane
/// - Evaluate each polygon as a potential dividing plane.
/// - Calculate the cost of using each polygon as the divider based
///      on factors like the number of splits, balance, etc.
/// - Select the polygon/plane with the lowest cost.
///
/// 6. Splitting Polygons
/// - For each polygon, determine if it is in front of, behind, or intersecting the dividing plane.
/// - If intersecting, split the polygon into two new polygons, one on each side of the plane.
///
/// 7. Recursive Construction
/// - Recursively apply the build process to the front and back lists of polygons
///     until all polygons are in leaf nodes or a stopping condition is met.
///
/// 8. Finalizing the Tree
/// - Once all polygons are assigned to leaf nodes or the recursion depth limit is reached,
///     finalize the tree construction and return the root node.
use crate::mesh::bool::bsp::{BSPNode, BSPTree, Plane, Polygon};
use crate::mesh::normals::{calculate_normal, MeshNormals};
use crate::mesh::parts::{Face, Vertex};
use crate::mesh::{Mesh, MeshError, MeshResult};
use glam::Vec3;
use log::{debug, info};
use rand::Rng;

const MAX_REC_DEPTH: usize = 60;

pub fn try_build_bsp_tree(polygons: &Vec<Polygon>, depth: Option<usize>) -> MeshResult<BSPTree> {
    Ok(BSPTree {
        root: Box::new(build_node(&polygons, depth.unwrap_or(MAX_REC_DEPTH))?),
    })
}
fn build_node(polygons: &Vec<Polygon>, residual_depth: usize) -> MeshResult<BSPNode> {
    if polygons.is_empty() || residual_depth == 0 {
        debug!("Polygons number is {} and depth is {residual_depth}", polygons.len());
        Ok(BSPNode::Leaf {
            polygons: polygons.clone(),
        })
    } else {
        let plane_polygon = &select_polygon_for_plane(polygons)?;
        let plane = create_plane(plane_polygon)?;

        let mut fronts = vec![];
        let mut backs = vec![];
        let mut ons = vec![];

        for poly in polygons {
            match classify(poly, &plane) {
                PolygonToPlane::Front => fronts.push(poly.clone()),
                PolygonToPlane::Back => backs.push(poly.clone()),
                PolygonToPlane::Spanning => {
                    let (front, back) = split(poly, &plane);
                    fronts.push(front);
                    backs.push(back);
                }
                PolygonToPlane::Coplanar => ons.push(poly.clone()),
            }
        }

        if fronts.is_empty() && backs.is_empty() {
            return Ok(BSPNode::Leaf {
                polygons: polygons.clone(),
            });
        } else {
            Ok(BSPNode::Node {
                front: Box::new(build_node(&fronts, residual_depth - 1)?),
                back: Box::new(build_node(&backs, residual_depth - 1)?),
                polygons: ons.clone(),
                plane,
            })
        }
    }
}
fn classify(polygon: &Polygon, plane: &Plane) -> PolygonToPlane {
    let mut front = 0;
    let mut back = 0;

    for v in polygon.vertices() {
        let d = plane.distance(v);
        if d > f32::EPSILON {
            front += 1;
        } else if d < -f32::EPSILON {
            back += 1;
        }
    }

    match (front, back) {
        (0, 0) => PolygonToPlane::Coplanar,
        (0, _) => PolygonToPlane::Back,
        (_, 0) => PolygonToPlane::Front,
        _ => PolygonToPlane::Spanning,
    }
}

/// Split a polygon by a plane into two new polygons.
/// The resulting polygons will be on the front and back side of the plane.
/// If the polygon intersects the plane, it will be split into two new polygons.
/// If the polygon is entirely on one side of the plane, it will be added to that side.
/// If the polygon is coplanar, it will be added to both sides.
/// The split operation is done by iterating over the polygon's vertices
/// and determining their position relative to the plane.
fn split(polygon: &Polygon, plane: &Plane) -> (Polygon, Polygon) {
    let mut front_vs = vec![];
    let mut back_vs = vec![];
    let len = polygon.vertices().len();

    for i in 0..polygon.vertices().len() {
        let current = polygon.vertices()[i];
        let next = polygon.vertices()[(i + 1) % len];

        let curr_d = plane.distance(current);
        let next_d = plane.distance(next);

        if curr_d >= 0.0 {
            front_vs.push(current);
        }
        if curr_d <= 0.0 {
            back_vs.push(current);
        }

        if curr_d > 0.0 && next_d < 0.0 || curr_d < 0.0 && next_d > 0.0 {
            let t = curr_d / (curr_d - next_d);
            let v = interpolate(current, next, t);
            front_vs.push(v);
            back_vs.push(v);
        }
    }
    (Polygon::from(front_vs), Polygon::from(back_vs))
}
fn interpolate(v1: Vertex, v2: Vertex, t: f32) -> Vertex {
    v1 * (1.0 - t) + v2 * t
}

/// Select a polygon from the list to use as a dividing plane.
/// For simplicity, we just pick the first polygon.
fn select_polygon_for_plane(polygons: &Vec<Polygon>) -> MeshResult<Polygon> {
    let rand_idx = rand::thread_rng().gen_range(0..polygons.len());
    polygons
        .get(rand_idx)
        .cloned()
        .ok_or(MeshError::InvalidIndex("Invalid index".to_string()))
}

fn create_plane(polygon: &Polygon) -> MeshResult<Plane> {
    let normal = calculate_normal(polygon.vertices());
    let point = polygon
        .vertices()
        .first()
        .ok_or(MeshError::InvalidIndex("No vertices".into()))?;
    Ok(Plane::new(normal.clone(), point.into()))
}

enum PolygonToPlane {
    Front,
    Back,
    Spanning,
    Coplanar,
}
