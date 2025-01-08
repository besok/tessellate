pub mod bbox;
pub mod edge;
pub mod face;
mod r#macro;
pub mod polygon;
pub mod vertex;
pub mod plane;
pub mod ray;

pub type Idx = usize;

#[cfg(test)]
mod tests {
    use crate::mesh::parts::edge::MeshEdge;
    use crate::mesh::Vertex;
    use crate::{v, edge};
    use crate::mesh::parts::edge::Edge;
    #[test]
    fn is_intersected_edges() {
        let e1 = edge!(v!(), v!(1, 1, 1));
        let e2 = edge!(v!(1,,), v!(,1,1));
        assert!(e1.is_intersected(&e2).unwrap());
    }
    #[test]
    fn is_intersected_parallel_edges() {
        let e1 = edge!(v!(,1,), v!(1, 2, 1));
        let e2 = edge!(v!(), v!(1, 1, 1));
        assert!(!e1.is_intersected(&e2).unwrap());
    }
    #[test]
    fn is_intersected_identical_edges() {
        let e1 = edge!(v!(), v!(1, 1, 1));

        assert!(e1.is_intersected(&e1).unwrap());
    }
}
