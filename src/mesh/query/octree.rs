use crate::mesh::parts::polygon::Polygon;
use crate::mesh::parts::BoundingBox;
use crate::mesh::query::MeshQuery;
use crate::mesh::{Mesh, MeshError, MeshResult};

pub mod build;

pub enum OctNode {
    Leaf {
        bb: BoundingBox,
        polygons: Vec<Polygon>,
        depth: usize,
    },
    Node {
        bb: BoundingBox,
        children: [Box<OctNode>; 8],
    },
}

pub struct Octree {
    root: Box<OctNode>,
}

impl Octree {
    pub fn try_from_mesh(mesh: &Mesh, depth: Option<usize>) -> MeshResult<Self> {
        build::try_build_octree(&mesh.try_polygons()?, depth)
    }
}

impl<'a> TryFrom<MeshQuery<'a>> for Octree {
    type Error = MeshError;
    fn try_from(q: MeshQuery<'a>) -> MeshResult<Self> {
        q.try_octree(None)
    }
}


#[cfg(test)]
mod tests{
    use crate::mesh::material::Color;
    use crate::mesh::Mesh;
    use crate::mesh::parts::polygon::Polygon;
    use crate::mesh::parts::vertex::Vertex;
    use crate::mesh::query::octree::{OctNode, Octree};

    #[test]
    fn smoke() {
        let polygons = vec![
            Polygon::new(vec![
                &Vertex::new(0.0, 0.0, 0.0),
                &Vertex::new(1.0, 0.0, 0.0),
                &Vertex::new(0.0, 1.0, 0.0),
            ]),
            Polygon::new(vec![
                &Vertex::new(1.0, 1.0, 0.0),
                &Vertex::new(2.0, 1.0, 0.0),
                &Vertex::new(1.0, 2.0, 0.0),
            ]),
        ];
        let mesh = Mesh::from_polygons(polygons,Color::default());
        let octree = Octree::try_from_mesh(&mesh, Some(3)).expect("Failed to build octree");

        match *octree.root {
            OctNode::Leaf { ref bb, ref polygons, depth } => {
                assert_eq!(depth, 0);
                assert_eq!(polygons.len(), 2);
                assert!(bb.contains(&Vertex::new(0.0, 0.0, 0.0)));
                assert!(bb.contains(&Vertex::new(2.0, 2.0, 0.0)));
            },
            OctNode::Node { .. } => panic!("Expected root to be a leaf node"),
        }
    }

}