use crate::mesh::parts::polygon::intersection::orient3d;
use crate::mesh::parts::polygon::Polygon;
use crate::mesh::query::MeshQuery;
use crate::mesh::{HasMesh, Mesh, MeshError, MeshResult};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Key {
    A(usize),
    B(usize),
}

impl Key {
    fn idx(&self) -> usize {
        match self {
            Key::A(idx) => *idx,
            Key::B(idx) => *idx,
        }
    }
}

pub(crate) struct IntersectionAnalyzer {
    a_polys: Vec<Polygon>,
    b_polys: Vec<Polygon>,
    inter_list: Vec<(Key, Key)>,
    coplanar_tris: HashMap<Key, Key>,
}

impl<'a> IntersectionAnalyzer {
    pub fn new(
        mesh_a: &Mesh,
        mesh_b: &Mesh,
        depth: Option<usize>,
    ) -> MeshResult<IntersectionAnalyzer> {
        prepare(mesh_a, mesh_b, depth)
    }

    fn get(&self, key: Key) -> MeshResult<&Polygon> {
        match key {
            Key::A(idx) => self
                .a_polys
                .get(idx)
                .ok_or(MeshError::InvalidIndex("Invalid index".to_string())),
            Key::B(idx) => self
                .b_polys
                .get(idx)
                .ok_or(MeshError::InvalidIndex("Invalid index".to_string())),
        }
    }

    pub(crate) fn check_tri_tri_intersections(&mut self, left: Key, right: Key) -> MeshResult<()> {
        let (a1, a2, a3) = self.get(left)?.to_triangle()?;
        let (b1, b2, b3) = self.get(right)?.to_triangle()?;

        let or_ba = normalize_orients(&(
            orient3d(a1, b1, b2, b3),
            orient3d(a2, b1, b2, b3),
            orient3d(a3, b1, b2, b3),
        ));

        if same_orient(or_ba.0, or_ba.1) && same_orient(or_ba.1, or_ba.2) && or_ba.1 != 0.0 {
            return Ok(()); //no intersection found
        }
        let mut coplanar = false;
        // all edge of tB are coplanar to all edges of tA   (orBA: 0 0 0)
        if all_coplanar_edges(&or_ba) {
            self.coplanar_tris.insert(left, right);
            self.coplanar_tris.insert(right, left);
            coplanar = true;
        }

        Ok(())
    }
}

fn prepare(mesh_a: &Mesh, mesh_b: &Mesh, depth: Option<usize>) -> MeshResult<IntersectionAnalyzer> {
    let mut all_polys_a = vec![];
    let mut all_polys_b = vec![];
    let mut inter_list = vec![];
    let a_tree = MeshQuery::new(mesh_a).try_octree(depth)?;
    let b_tree = MeshQuery::new(mesh_b).try_octree(depth)?;
    for a_leaf in a_tree.iter_leafs() {
        let a_polygons: Vec<_> = a_leaf.triangles();
        let a_idx_start = all_polys_a.len();
        all_polys_a.extend(a_polygons.iter().cloned());
        for b_leaf in b_tree.iter_leafs() {
            let b_polygons: Vec<_> = b_leaf.triangles();
            let b_idx_start = all_polys_b.len();
            all_polys_b.extend(b_polygons.iter().cloned());

            if a_leaf.is_overlapping(b_leaf) {
                for (a_idx, a_triangle) in a_polygons.iter().enumerate() {
                    for (b_idx, b_triangle) in b_polygons.iter().enumerate() {
                        if a_triangle.intersects_precise(b_triangle)?.intersect() {
                            inter_list
                                .push((Key::A(a_idx + a_idx_start), Key::B(b_idx + b_idx_start)));
                        }
                    }
                }
            }
        }
    }
    Ok(IntersectionAnalyzer {
        a_polys: all_polys_a,
        b_polys: all_polys_b,
        coplanar_tris: HashMap::new(),
        inter_list,
    })
}
fn normalize_orients((o1, o2, o3): &(f32, f32, f32)) -> (f32, f32, f32) {
    (
        if *o1 < 0.0 { -1.0 } else { 1.0 },
        if *o2 < 0.0 { -1.0 } else { 1.0 },
        if *o3 < 0.0 { -1.0 } else { 1.0 },
    )
}

fn same_orient(o1: f32, o2: f32) -> bool {
    (o1 < 0.0 && o2 < 0.0) || (o1 > 0.0 && o2 > 0.0) || (o1 == 0.0 && o2 == 0.0)
}

fn all_coplanar_edges((a, b, c): &(f32, f32, f32)) -> bool {
    *a == 0.0 && *b == 0.0 && *c == 0.0
}
