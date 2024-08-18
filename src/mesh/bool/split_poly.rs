use crate::mesh::parts::polygon::Polygon;
use crate::mesh::MeshResult;

pub(crate) fn split_polygons(lhs: &Polygon, rhs: &Polygon) -> MeshResult<Vec<Polygon>> {
    let mut res_poly = Vec::new();

    for e1 in lhs.edges().iter() {
        for e2 in rhs.edges().iter() {
            if let Some((v1, v2)) = e1.find_collinear_segment(e2) {

            } else if let Some(v1) = e1.find_intersection(e2) {
            }
        }
    }

    Ok(res_poly)
}
