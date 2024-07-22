/// Binary Space Partitioning (BSP) tree for mesh boolean operations.
/// The BSP tree is a binary tree that splits a mesh into two parts based on a plane.
/// The tree is used to determine the intersection of two meshes.
/// The tree is built by recursively splitting the mesh into two parts based on a plane.
///
/// The plane is chosen based on a heuristic that minimizes the number of splits.

use crate::mesh::normals::MeshNormals;
use crate::mesh::parts::{Face, Vertex};
use crate::mesh::{Mesh, MeshError, MeshResult};
use glam::Vec3;
use rand::Rng;

enum BSPNode {
    Leaf(Vec<Face>),
    Node {
        plane: Plane,
        front: Box<BSPNode>,
        back: Box<BSPNode>,
    },
}
struct Plane {
    normal: Vec3,
    point: Vec3,
    dist: f32,
}

impl Plane {
    fn new(normal: Vec3, point: Vec3) -> Self {
        let dist = normal.dot(point);
        Self {
            normal,
            point,
            dist,
        }
    }

    fn distance(&self, point: Vec3) -> f32 {
        self.normal.dot(point) - self.dist
    }
}

struct BSPTreeBuilder<'a> {
    mesh: &'a Mesh,
    normals: MeshNormals,
}
struct BSPTree {
    root: Box<BSPNode>,
}
impl<'a> BSPTreeBuilder<'a> {
    pub fn new(mesh: &'a Mesh) -> MeshResult<Self> {
        Ok(Self {
            mesh,
            normals: MeshNormals::try_from(mesh)?,
        })
    }

    pub fn build(&self) -> MeshResult<BSPTree> {
        Ok(BSPTree {
            root: Box::new(self.build_node(self.mesh.faces())?),
        })
    }

    fn build_node(&self, faces: &Vec<Face>) -> MeshResult<BSPNode> {
        let plane = self.choose_plane(faces)?;
        let mut fronts = vec![];
        let mut backs = vec![];

        for face in faces.iter() {
            match classify_face(self.get_vertices(face)?, &plane) {
                Pos::Front => fronts.push(face.clone()),
                Pos::Back => backs.push(face.clone()),
                Pos::OnPlane => {
                    let (front_part, back_part) = self.split_face(face, &plane);
                    fronts.push(front_part);
                    backs.push(back_part);
                }
            }
        }

        Ok(BSPNode::Node {
            plane,
            front: Box::new(self.build_node(&fronts)?),
            back: Box::new(self.build_node(&backs)?),
        })
    }

    /// The heuristic might involve minimizing the number of splits, balancing the tree,
    /// or minimizing the sum of all polygons in front and behind the plane.
    /// For simplicity, it is a random pick but in the future,
    /// it should be replaced with a better heuristic.
    fn choose_plane(&self, faces: &Vec<Face>) -> MeshResult<Plane> {
        let mut rng = rand::thread_rng();
        let rand_idx = rng.gen_range(0..faces.len());
        self.create_plane_from_face(
            faces
                .get(rand_idx)
                .ok_or(MeshError::InvalidIndex("Invalid index".to_string()))?,
        )
    }

    fn split_face(&self, face: &Face, plane: &Plane) -> (Face, Face) {
        unimplemented!()
    }

    fn get_vertices(&self, face: &Face) -> MeshResult<Vec<&Vertex>> {
        face.flatten()
            .iter()
            .map(|i| self.mesh.get_v(*i))
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
    }

    fn create_plane_from_face(&self, face: &Face) -> MeshResult<Plane> {
        let normal = self.normals.get_face_normal(face.clone())?;
        let point = self
            .get_vertices(face)?
            .first()
            .ok_or(MeshError::InvalidIndex("Invalid vertex index".to_string()))?
            .clone()
            .into();
        Ok(Plane::new(normal.clone(), point))
    }
}

fn classify_vertex(vertex: Vec3, plane: &Plane) -> Pos {
    match plane.distance(vertex) {
        dist if dist > 0.0 => Pos::Front,
        dist if dist < 0.0 => Pos::Back,
        _ => Pos::OnPlane,
    }
}

fn classify_face(face: Vec<&Vertex>, plane: &Plane) -> Pos {
    let mut front = 0;
    let mut back = 0;
    for vertex in face.iter() {
        match classify_vertex(vertex.clone().into(), &plane) {
            Pos::Front => front += 1,
            Pos::Back => back += 1,
            Pos::OnPlane => {
                front += 1;
                back += 1;
            }
        }
    }

    if front == 0 {
        Pos::Back
    } else if back == 0 {
        Pos::Front
    } else {
        Pos::OnPlane
    }
}

enum Pos {
    Front,
    Back,
    OnPlane,
}


#[cfg(test)]
mod tests{
    use crate::mesh::{HasMesh, Mesh};
    use crate::mesh::bool::bsp::BSPTreeBuilder;
    use crate::mesh::shape::cuboid::cube::Cube;

    #[test]
    fn smoke_test() {
        let cube = Cube::default();
        let mesh = cube.mesh();
        let bsp = BSPTreeBuilder::new(mesh).unwrap().build().unwrap();


    }
}