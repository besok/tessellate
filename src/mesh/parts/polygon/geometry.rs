use crate::mesh::parts::polygon::Polygon;
use crate::mesh::parts::vertex::{Vertex, Vertex2};
use crate::mesh::{MeshError, MeshResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimplexIntersection {
    DoNotIntersect,
    SimplicialComplex,
    Intersect,
    Overlap,
}

pub(crate) fn polys_tri_intersect(p1: &Polygon, p2: &Polygon) -> MeshResult<SimplexIntersection> {
    triangle_intersects_triangle(
        (&p1.vertices()[0], &p1.vertices()[1], &p1.vertices()[2]),
        (&p2.vertices()[0], &p2.vertices()[1], &p2.vertices()[2]),
    )
}

pub(crate) fn triangle_intersects_triangle(
    (a0, a1, a2): (&Vertex, &Vertex, &Vertex),
    (b0, b1, b2): (&Vertex, &Vertex, &Vertex),
) -> MeshResult<SimplexIntersection> {
    let mut shared_vs = Vec::new();

    for a in [a0, a1, a2].iter() {
        for b in [b0, b1, b2].iter() {
            if a == b {
                shared_vs.push(*a);
            }
        }
    }
    match &shared_vs[..] {
        [_, _, _] => Ok(SimplexIntersection::SimplicialComplex),
        [x1, x2] => {
            let find_opposite_vertex = |vertices: &[&Vertex]| {
                vertices
                    .iter()
                    .find(|&v| v != x1 && v != x2)
                    .ok_or_else(|| MeshError::Custom("opposite vertex not found".to_string()))
            };

            let a_opposite = find_opposite_vertex(&[a0, a1, a2])?;
            let b_opposite = find_opposite_vertex(&[b0, b1, b2])?;

            if orient3d(a0, a1, a2, b_opposite) == 0.0 {
                Ok(SimplexIntersection::SimplicialComplex)
            } else {
                let check_intersection = |drop_fn: fn(&Vertex) -> Vertex2| -> bool {
                    let e0 = drop_fn(*x1);
                    let e1 = drop_fn(*x2);
                    let opp0 = drop_fn(a_opposite);
                    let opp1 = drop_fn(b_opposite);
                    let opp0_wrt_e = orient2d(&e0, &e1, &opp0);
                    let opp1_wrt_e = orient2d(&e0, &e1, &opp1);
                    (opp0_wrt_e > 0.0 && opp1_wrt_e < 0.0) || (opp0_wrt_e < 0.0 && opp1_wrt_e > 0.0)
                };

                if check_intersection(Vertex2::yz)
                    || check_intersection(Vertex2::xz)
                    || check_intersection(Vertex2::xy)
                {
                    Ok(SimplexIntersection::SimplicialComplex)
                } else {
                    Ok(SimplexIntersection::Intersect)
                }
            }
        }
        [x1] => {
            let [ao1, ao2] = &[a0, a1, a2].iter().filter(|&v| v != x1).collect::<Vec<_>>()[..];
            let [bo1, bo2] = &[b0, b1, b2].iter().filter(|&v| v != x1).collect::<Vec<_>>()[..];

            let a_inter = segment_triangle_intersect_3d(ao1, ao2, b0, b1, b2)?;
            let b_inter = segment_triangle_intersect_3d(bo1, bo2, a0, a1, a2)?;

            if a_inter == SimplexIntersection::Intersect
                || a_inter == SimplexIntersection::Overlap
                || b_inter == SimplexIntersection::Intersect
                || b_inter == SimplexIntersection::Overlap
            {
                Ok(SimplexIntersection::Intersect)
            } else {
                Ok(SimplexIntersection::SimplicialComplex)
            }
        }
        [] => Ok(SimplexIntersection::SimplicialComplex,)
        _ => unreachable!("Invalid number of shared vertices"),
    }
}

pub(crate) fn triangle_is_colinear(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> bool {
    let v0_drop_x = [v0[1], v0[2]];
    let v0_drop_y = [v0[0], v0[2]];
    let v0_drop_z = [v0[0], v0[1]];
    let v1_drop_x = [v1[1], v1[2]];
    let v1_drop_y = [v1[0], v1[2]];
    let v1_drop_z = [v1[0], v1[1]];
    let v2_drop_x = [v2[1], v2[2]];
    let v2_drop_y = [v2[0], v2[2]];
    let v2_drop_z = [v2[0], v2[1]];

    triangle_is_colinear2d(&v0_drop_x, &v1_drop_x, &v2_drop_x)
        && triangle_is_colinear2d(&v0_drop_y, &v1_drop_y, &v2_drop_y)
        && triangle_is_colinear2d(&v0_drop_z, &v1_drop_z, &v2_drop_z)
}
pub(crate) fn triangle_is_colinear2d(
    (x0, y0): &(f32, f32),
    (x1, y1): &(f32, f32),
    (x2, y2): &(f32, f32),
) -> bool {
    (x1 - x0) * (y2 - y0) == (x2 - x0) * (y1 - y0)
}
pub(crate) fn calculate_segment_wntv(start: Vertex, end: Vertex, reference: Vertex) -> f32 {
    let cross = (end - start).cross(&(reference - start));
    if cross.z > 0.0 {
        1.0
    } else {
        -1.0
    }
}

pub(crate) fn segment_triangle_intersect_3d(
    s0: &Vertex,
    s1: &Vertex,
    t0: &Vertex,
    t1: &Vertex,
    t2: &Vertex,
) -> MeshResult<SimplexIntersection> {
    if s0 == s1 && triangle_is_colinear(t0, t1, t2) {
        return Err(MeshError::Custom("colinear vertices".to_string()));
    }

    if (s0 == t0 || s0 == t1 || s0 == t2) && (s1 == t0 || s1 == t1 || s1 == t2) {
        return Ok(SimplexIntersection::SimplicialComplex);
    }

    let (vol_s0_t, vol_s1_t) = (orient3d(s0, t0, t1, t2), orient3d(s1, t0, t1, t2));

    if vol_s0_t > 0.0 && vol_s1_t > 0.0 {
        return Ok(SimplexIntersection::DoNotIntersect); // s is above t
    }
    if vol_s0_t < 0.0 && vol_s1_t < 0.0 {
        return Ok(SimplexIntersection::DoNotIntersect); // s is below t
    }
    if vol_s0_t == 0.0 && vol_s1_t == 0.0 {
        // s and t are coplanar
        if point_in_triangle_3d(s0, t0, t1, t2) || point_in_triangle_3d(s1, t0, t1, t2) {
            return SimplexIntersection::Intersect;
        }

        let mut simpl_complex = 0;

        match segment_segment_intersect_3d(s0, s1, t0, t1) {
            SimplexIntersection::SimplicialComplex => simpl_complex += 1,
            SimplexIntersection::Intersect => return SimplexIntersection::Intersect,
            SimplexIntersection::Overlap => return SimplexIntersection::Intersect,
            SimplexIntersection::DoNotIntersect => (),
        }

        match segment_segment_intersect_3d(s0, s1, t1, t2) {
            SimplexIntersection::SimplicialComplex => simpl_complex += 1,
            SimplexIntersection::Intersect => return SimplexIntersection::Intersect,
            SimplexIntersection::Overlap => return SimplexIntersection::Intersect,
            SimplexIntersection::DoNotIntersect => (),
        }

        match segment_segment_intersect_3d(s0, s1, t2, t0) {
            SimplexIntersection::SimplicialComplex => simpl_complex += 1,
            SimplexIntersection::Intersect => return SimplexIntersection::Intersect,
            SimplexIntersection::Overlap => return SimplexIntersection::Intersect,
            SimplexIntersection::DoNotIntersect => (),
        }

        if simpl_complex == 3 {
            return Ok(SimplexIntersection::SimplicialComplex);
        } else {
            return Ok(SimplexIntersection::DoNotIntersect);
        }
    }

    if s0 == t0 || s0 == t1 || s0 == t2 || s1 == t0 || s1 == t1 || s1 == t2 {
        return Ok(SimplexIntersection::SimplicialComplex);
    }

    let vol_s_t01 = orient3d(s0, s1, t0, t1);
    let vol_s_t12 = orient3d(s0, s1, t1, t2);
    let vol_s_t20 = orient3d(s0, s1, t2, t0);

    if (vol_s_t01 > 0.0 && vol_s_t12 < 0.0) || (vol_s_t01 < 0.0 && vol_s_t12 > 0.0) {
        return Ok(SimplexIntersection::DoNotIntersect);
    }
    if (vol_s_t12 > 0.0 && vol_s_t20 < 0.0) || (vol_s_t12 < 0.0 && vol_s_t20 > 0.0) {
        return Ok(SimplexIntersection::DoNotIntersect);
    }
    if (vol_s_t20 > 0.0 && vol_s_t01 < 0.0) || (vol_s_t20 < 0.0 && vol_s_t01 > 0.0) {
        return Ok(SimplexIntersection::DoNotIntersect);
    }

    Ok(SimplexIntersection::Intersect)
}

pub(crate) fn orient3d(a: &Vertex, b: &Vertex, c: &Vertex, d: &Vertex) -> f32 {
    let adx = a.x - d.x;
    let bdx = b.x - d.x;
    let cdx = c.x - d.x;
    let ady = a.y - d.y;
    let bdy = b.y - d.y;
    let cdy = c.y - d.y;
    let adz = a.z - d.z;
    let bdz = b.z - d.z;
    let cdz = c.z - d.z;

    adz * (bdx * cdy - cdx * bdy) + bdz * (cdx * ady - adx * cdy) + cdz * (adx * bdy - bdx * ady)
}

pub(crate) fn orient2d<T: Into<Vertex2>>(pa: &T, pb: &T, pc: &T) -> f32 {
    let pa = pa.into();
    let pb = pb.into();
    let pc = pc.into();

    let adx = pa.x - pc.x;
    let bdx = pb.x - pc.x;
    let ady = pa.y - pc.y;
    let bdy = pb.y - pc.y;

    adx * bdy - bdx * ady
}
