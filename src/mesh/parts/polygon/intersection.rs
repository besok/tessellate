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

impl SimplexIntersection {
    pub fn do_not_intersect(&self) -> bool {
        matches!(self, SimplexIntersection::DoNotIntersect)
    }

    pub fn intersect(&self) -> bool {
        !self.do_not_intersect()
    }
}
#[derive(Debug, PartialEq)]
pub enum PointInSimplex {
    OnVert0,
    OnVert1,
    OnVert2,
    OnEdge0,
    OnEdge1,
    OnEdge2,
    StrictlyInside,
    StrictlyOutside,
}

impl PointInSimplex {
    pub fn is_outside(&self) -> bool {
        matches!(self, PointInSimplex::StrictlyOutside)
    }
    pub fn not_outside(&self) -> bool {
        !self.is_outside()
    }
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
    let a_vs = [a0, a1, a2];
    let b_vs = [b0, b1, b2];
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
            let a_opposite = &a_vs
                .iter()
                .find(|&v| v != x1 && v != x2)
                .ok_or(MeshError::Custom("opposite vertex not found".to_string()))?;

            let b_opposite = &b_vs
                .iter()
                .find(|&v| v != x1 && v != x2)
                .ok_or(MeshError::Custom("opposite vertex not found".to_string()))?;

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
            let [ao1, ao2] = &a_vs.iter().filter(|&v| v != x1).collect::<Vec<_>>()[..] else {
                return Err(MeshError::Custom("wrong triangles".to_string()));
            };
            let [bo1, bo2] = &b_vs.iter().filter(|&v| v != x1).collect::<Vec<_>>()[..] else {
                return Err(MeshError::Custom("wrong triangles".to_string()));
            };

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
        [] => Ok(SimplexIntersection::SimplicialComplex),
        _ => unreachable!("Invalid number of shared vertices"),
    }
}

pub(crate) fn triangle_is_colinear(v0: &Vertex, v1: &Vertex, v2: &Vertex) -> bool {
    let v0_drop_x = (v0.y, v0.z);
    let v0_drop_y = (v0.x, v0.z);
    let v0_drop_z = (v0.x, v0.y);
    let v1_drop_x = (v1.y, v1.z);
    let v1_drop_y = (v1.x, v1.z);
    let v1_drop_z = (v1.x, v1.y);
    let v2_drop_x = (v2.y, v2.z);
    let v2_drop_y = (v2.x, v2.z);
    let v2_drop_z = (v2.x, v2.y);

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
    println!("segment_triangle_intersect_3d: {} {} {} {} {}", s0, s1, t0, t1, t2);
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
        if point_in_triangle_3d(s0, t0, t1, t2).not_outside()
            || point_in_triangle_3d(s1, t0, t1, t2).not_outside()
        {
            return Ok(SimplexIntersection::Intersect);
        }

        let mut simpl_complex = 0;

        match segment_segment_intersect_3d(s0, s1, t0, t1)? {
            SimplexIntersection::SimplicialComplex => simpl_complex += 1,
            SimplexIntersection::Intersect => return Ok(SimplexIntersection::Intersect),
            SimplexIntersection::Overlap => return Ok(SimplexIntersection::Intersect),
            SimplexIntersection::DoNotIntersect => (),
        }

        match segment_segment_intersect_3d(s0, s1, t1, t2)? {
            SimplexIntersection::SimplicialComplex => simpl_complex += 1,
            SimplexIntersection::Intersect => return Ok(SimplexIntersection::Intersect),
            SimplexIntersection::Overlap => return Ok(SimplexIntersection::Intersect),
            SimplexIntersection::DoNotIntersect => (),
        }

        match segment_segment_intersect_3d(s0, s1, t2, t0)? {
            SimplexIntersection::SimplicialComplex => simpl_complex += 1,
            SimplexIntersection::Intersect => return Ok(SimplexIntersection::Intersect),
            SimplexIntersection::Overlap => return Ok(SimplexIntersection::Intersect),
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

pub(crate) fn orient2d(pa: &Vertex2, pb: &Vertex2, pc: &Vertex2) -> f32 {
    let adx = pa.x - pc.x;
    let bdx = pb.x - pc.x;
    let ady = pa.y - pc.y;
    let bdy = pb.y - pc.y;

    adx * bdy - bdx * ady
}

pub fn point_in_triangle_3d(p: &Vertex, t0: &Vertex, t1: &Vertex, t2: &Vertex) -> PointInSimplex {
    if p == t0 {
        return PointInSimplex::OnVert0;
    }
    if p == t1 {
        return PointInSimplex::OnVert1;
    }
    if p == t2 {
        return PointInSimplex::OnVert2;
    }

    if point_in_segment_3d(p, t0, t1) == PointInSimplex::StrictlyInside {
        return PointInSimplex::OnEdge0;
    }
    if point_in_segment_3d(p, t1, t2) == PointInSimplex::StrictlyInside {
        return PointInSimplex::OnEdge1;
    }
    if point_in_segment_3d(p, t2, t0) == PointInSimplex::StrictlyInside {
        return PointInSimplex::OnEdge2;
    }

    let p_drop_x = Vertex2::new(p.y, p.z);
    let t0_drop_x = Vertex2::new(t0.y, t0.z);
    let t1_drop_x = Vertex2::new(t1.y, t1.z);
    let t2_drop_x = Vertex2::new(t2.y, t2.z);

    if point_in_triangle_2d(&p_drop_x, &t0_drop_x, &t1_drop_x, &t2_drop_x)
        == PointInSimplex::StrictlyOutside
    {
        return PointInSimplex::StrictlyOutside;
    }

    let p_drop_y = Vertex2::new(p.x, p.z);
    let t0_drop_y = Vertex2::new(t0.x, t0.z);
    let t1_drop_y = Vertex2::new(t1.x, t1.z);
    let t2_drop_y = Vertex2::new(t2.x, t2.z);

    if point_in_triangle_2d(&p_drop_y, &t0_drop_y, &t1_drop_y, &t2_drop_y)
        == PointInSimplex::StrictlyOutside
    {
        return PointInSimplex::StrictlyOutside;
    }

    let p_drop_z = Vertex2::new(p.x, p.y);
    let t0_drop_z = Vertex2::new(t0.x, t0.y);
    let t1_drop_z = Vertex2::new(t1.x, t1.y);
    let t2_drop_z = Vertex2::new(t2.x, t2.y);

    if point_in_triangle_2d(&p_drop_z, &t0_drop_z, &t1_drop_z, &t2_drop_z)
        == PointInSimplex::StrictlyOutside
    {
        return PointInSimplex::StrictlyOutside;
    }

    PointInSimplex::StrictlyInside
}
pub fn point_in_segment_3d(p: &Vertex, s0: &Vertex, s1: &Vertex) -> PointInSimplex {
    if p == s0 {
        PointInSimplex::OnVert0
    } else if p == s1 {
        PointInSimplex::OnVert1
    } else if !triangle_is_colinear(s0, s1, p) {
        PointInSimplex::StrictlyOutside
    } else if (p.x > s0.x.min(s1.x) && p.x < s0.x.max(s1.x))
        || (p.y > s0.y.min(s1.y) && p.y < s0.y.max(s1.y))
        || (p.z > s0.z.min(s1.z) && p.z < s0.z.max(s1.z))
    {
        PointInSimplex::StrictlyInside
    } else {
        PointInSimplex::StrictlyOutside
    }
}

pub fn point_in_triangle_2d(
    p: &Vertex2,
    t0: &Vertex2,
    t1: &Vertex2,
    t2: &Vertex2,
) -> PointInSimplex {
    if p == t0 {
        return PointInSimplex::OnVert0;
    }
    if p == t1 {
        return PointInSimplex::OnVert1;
    }
    if p == t2 {
        return PointInSimplex::OnVert2;
    }

    let e0p_area = orient2d(t0, t1, p);
    let e1p_area = orient2d(t1, t2, p);
    let e2p_area = orient2d(t2, t0, p);

    let hit = (e0p_area >= 0.0 && e1p_area >= 0.0 && e2p_area >= 0.0)
        || (e0p_area <= 0.0 && e1p_area <= 0.0 && e2p_area <= 0.0);

    if hit {
        if e0p_area == 0.0 {
            return PointInSimplex::OnEdge0;
        }
        if e1p_area == 0.0 {
            return PointInSimplex::OnEdge1;
        }
        if e2p_area == 0.0 {
            return PointInSimplex::OnEdge2;
        }
        return PointInSimplex::StrictlyInside;
    }

    PointInSimplex::StrictlyOutside
}

pub fn segment_segment_intersect_3d(
    s00: &Vertex,
    s01: &Vertex,
    s10: &Vertex,
    s11: &Vertex,
) -> MeshResult<SimplexIntersection> {
    if s00 == s01 && s10 == s11 {
        return Err(MeshError::Custom("degenerate segments".to_string()));
    }

    if !(orient3d(s00, s01, s10, s11) == 0.0) {
        return Ok(SimplexIntersection::DoNotIntersect);
    }

    let s00_is_shared = s00 == s10 || s00 == s11;
    let s01_is_shared = s01 == s10 || s01 == s11;
    let s10_is_shared = s10 == s00 || s10 == s01;
    let s11_is_shared = s11 == s00 || s11 == s01;

    if s00_is_shared && s01_is_shared && s10_is_shared && s11_is_shared {
        return Ok(SimplexIntersection::SimplicialComplex);
    }

    let s00_drop_x = Vertex2::new(s00.y, s00.z);
    let s01_drop_x = Vertex2::new(s01.y, s01.z);
    let s10_drop_x = Vertex2::new(s10.y, s10.z);
    let s11_drop_x = Vertex2::new(s11.y, s11.z);
    let x_res = segment_segment_intersect_2d(&s00_drop_x, &s01_drop_x, &s10_drop_x, &s11_drop_x);
    if x_res == SimplexIntersection::DoNotIntersect {
        return Ok(SimplexIntersection::DoNotIntersect);
    }

    let s00_drop_y = Vertex2::new(s00.x, s00.z);
    let s01_drop_y = Vertex2::new(s01.x, s01.z);
    let s10_drop_y = Vertex2::new(s10.x, s10.z);
    let s11_drop_y = Vertex2::new(s11.x, s11.z);
    let y_res = segment_segment_intersect_2d(&s00_drop_y, &s01_drop_y, &s10_drop_y, &s11_drop_y);
    if y_res == SimplexIntersection::DoNotIntersect {
        return Ok(SimplexIntersection::DoNotIntersect);
    }

    let s00_drop_z = Vertex2::new(s00.x, s00.y);
    let s01_drop_z = Vertex2::new(s01.x, s01.y);
    let s10_drop_z = Vertex2::new(s10.x, s10.y);
    let s11_drop_z = Vertex2::new(s11.x, s11.y);
    let z_res = segment_segment_intersect_2d(&s00_drop_z, &s01_drop_z, &s10_drop_z, &s11_drop_z);
    if z_res == SimplexIntersection::DoNotIntersect {
        return Ok(SimplexIntersection::DoNotIntersect);
    }

    if (x_res == SimplexIntersection::Overlap && y_res == SimplexIntersection::Overlap)
        || (x_res == SimplexIntersection::Overlap && z_res == SimplexIntersection::Overlap)
        || (y_res == SimplexIntersection::Overlap && z_res == SimplexIntersection::Overlap)
    {
        return Ok(SimplexIntersection::Overlap);
    }

    Ok(SimplexIntersection::Intersect)
}

pub(crate) fn segment_segment_intersect_2d(
    s00: &Vertex2,
    s01: &Vertex2,
    s10: &Vertex2,
    s11: &Vertex2,
) -> SimplexIntersection {
    fn sign(det: f32) -> i32 {
        if det > 0.0 {
            1
        } else if det < 0.0 {
            -1
        } else {
            0
        }
    }

    let det_s00 = orient2d(s10, s11, s00);
    let det_s01 = orient2d(s10, s11, s01);
    let det_s10 = orient2d(s00, s01, s10);
    let det_s11 = orient2d(s00, s01, s11);

    let s00_wrt_s1 = sign(det_s00);
    let s01_wrt_s1 = sign(det_s01);
    let s10_wrt_s0 = sign(det_s10);
    let s11_wrt_s0 = sign(det_s11);

    if s00_wrt_s1 != s01_wrt_s1 && s10_wrt_s0 != s11_wrt_s0 {
        if [s00, s01].contains(&s10) || [s00, s01].contains(&s11) {
            return SimplexIntersection::SimplicialComplex;
        }
        return SimplexIntersection::Intersect;
    }

    if [s00_wrt_s1, s01_wrt_s1, s10_wrt_s0, s11_wrt_s0]
        .iter()
        .all(|&x| x == 0)
    {
        if (s00 == s10 && s01 == s11) || (s00 == s11 && s01 == s10) {
            return SimplexIntersection::SimplicialComplex;
        }

        let (x_min_s1, x_max_s1) = (s10.x.min(s11.x), s10.x.max(s11.x));
        let (y_min_s1, y_max_s1) = (s10.y.min(s11.y), s10.y.max(s11.y));
        let (x_min_s0, x_max_s0) = (s00.x.min(s01.x), s00.x.max(s01.x));
        let (y_min_s0, y_max_s0) = (s00.y.min(s01.y), s00.y.max(s01.y));


        if (s00.x > x_min_s1 && s00.x < x_max_s1)
            || (s00.y > y_min_s1 && s00.y < y_max_s1)
            || (s01.x > x_min_s1 && s01.x < x_max_s1)
            || (s01.y > y_min_s1 && s01.y < y_max_s1)
            || (s10.x > x_min_s0 && s10.x < x_max_s0)
            || (s10.y > y_min_s0 && s10.y < y_max_s0)
            || (s11.x > x_min_s0 && s11.x < x_max_s0)
            || (s11.y > y_min_s0 && s11.y < y_max_s0)
        {
            return SimplexIntersection::Intersect;
        }
    }

    SimplexIntersection::DoNotIntersect
}
pub fn vertices_are_colinear(p: &Polygon) -> bool {
    let ps = p.triangulate();
    ps.iter().any(|p| {
        let v0 = p.vertices()[0];
        let v1 = p.vertices()[1];
        let v2 = p.vertices()[2];
        triangle_is_colinear(&v0, &v1, &v2)
    })
}
