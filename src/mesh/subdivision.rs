use std::collections::HashSet;
use crate::mesh::parts::edge::MeshEdge;
use crate::mesh::parts::face::Face;
use crate::mesh::{Mesh, MeshError, MeshResult};
use egui::ahash::{HashMap, HashMapExt};
use crate::mesh::parts::vertex::Vertex;
/// Subdivides the mesh using the Butterfly subdivision scheme.
///
/// This method applies the Butterfly subdivision algorithm to the mesh,
/// which is a technique used to create a smoother mesh by adding new vertices
/// and adjusting the positions of existing vertices.
///
/// # Parameters
///
/// * `mesh` - A reference to the `Mesh` to be subdivided.
///
/// # Returns
///
/// A `MeshResult` containing the subdivided `Mesh` if successful, or a `MeshError` if an error occurs.
pub fn by_butterfly(mesh: &Mesh) -> MeshResult<Mesh>{
    if !mesh.properties().is_watertight(){
        return Err(MeshError::Custom("Mesh is not watertight".to_string()));
    }
    let trimesh = mesh.triangulate()?;
    let mut new_vertices = trimesh.vertices.clone();
    let table = trimesh.try_tables()?;
    let mut cache = HashMap::new();
    let add_edge = |lhs:usize,rhs:usize|{
        if let Some(idx) = cache.get(&(lhs,rhs)){
            Ok::<usize, MeshError>(*idx)
        }
        else{
            let p1 = trimesh.get(lhs)?;
            let p2 = trimesh.get(rhs)?;
            let adjacent_vs:Vec<_> =
                table
                    .edge_faces(lhs, rhs)?
                    .iter()
                    .flat_map(|f|f.flatten())
                    .filter(|v| *v != lhs && *v != rhs)
                    .collect();
            let new_point = if adjacent_vs.len() ==2 {
                let p3 = trimesh.get(adjacent_vs[0])?;
                let p4 = trimesh.get(adjacent_vs[1])?;
                (*p1 + *p2) * 0.5  + (*p3 + *p4) *  0.0625
            } else {
                (*p1 + *p2) * 0.5
            };
            let idx = new_vertices.len();
            new_vertices.push(new_point);
            cache.insert((lhs, rhs), idx);
            cache.insert((rhs, lhs), idx);
            Ok(idx)
        }
    };
    let  new_faces = subdivide(&trimesh, add_edge)?;

    Ok(Mesh::from_vertices(new_vertices, new_faces, trimesh.attributes.clone()))
}


/// Subdivides the mesh using the Loop subdivision scheme.
///
/// This method applies the Loop subdivision algorithm to the mesh,
/// which is a technique used to create a smoother mesh by adding new vertices
/// and adjusting the positions of existing vertices.
///
/// # Returns
///
/// A `MeshResult` containing the subdivided `Mesh` if successful, or a `MeshError` if an error occurs.
pub fn by_loop(mesh: &Mesh) -> MeshResult<Mesh> {
    if !mesh.properties().is_watertight(){
        return Err(MeshError::Custom("Mesh is not watertight".to_string()));
    }
    let trimesh = mesh.triangulate()?;
    let start_idx = trimesh.vertices().len();
    let mut new_vertices = vec![];

    let mut cache = HashMap::new();
    let add_edge  = |lhs:usize, rhs:usize|{
        if let Some(idx) = cache.get(&(lhs,rhs)){
            Ok::<usize, MeshError>(*idx)
        }
        else{
            let v1 = trimesh.get(lhs)?;
            let v2 = trimesh.get(rhs)?;

            let next = new_vertices.len() + start_idx;
            new_vertices.push((*v1 + *v2) / 2.0);
            cache.insert((lhs,rhs),next);
            cache.insert((rhs,lhs),next);
            Ok(next)
        }
    };
    let new_faces = subdivide(&trimesh, add_edge)?;

    let mut vertex_neighbors = HashMap::new();
    for (idx,v) in trimesh.vertices().iter().enumerate(){
        vertex_neighbors.insert(idx,HashSet::new());
    }

    for MeshEdge(lhs,rhs) in trimesh.edges().iter(){
        vertex_neighbors.get_mut(&lhs).ok_or(MeshError::idx_vertex(*lhs))?.insert(rhs);
        vertex_neighbors.get_mut(&rhs).ok_or(MeshError::idx_vertex(*rhs))?.insert(lhs);
    }

    let mut updated_vertices = trimesh.vertices.clone();
    for (idx,neighbours) in vertex_neighbors.iter(){
        let n = neighbours.len() as f32;
        let beta = if n > 3.0 {
            3.0 / (8.0 * n)
        } else {
            3.0 / 16.0
        };
        let mut new_vertex = *trimesh.get(*idx)? * (1.0 - n * beta);
        let mut coef = Vertex::default();
        for neighbor in neighbours {
            coef = coef + *trimesh.get(**neighbor)?;
        }
        coef = coef * beta;
        updated_vertices[*idx] = new_vertex + coef;
    }

    Ok(Mesh::from_vertices([updated_vertices, new_vertices].concat(), new_faces, trimesh.attributes.clone()))
}

fn subdivide<F>(mesh: &Mesh, mut add_edge: F) -> MeshResult<Vec<Face>>
where F: FnMut(usize, usize) -> MeshResult<usize> {
    let mut new_faces = vec![];
    for face in mesh.faces(){

        let [a,b,c] = face.flatten()[..] else {
            return Err(MeshError::WrongMesh("Invalid number of vertices".to_string()))
        };
        let m1 = add_edge(a,b)?;
        let m2 = add_edge(b,c)?;
        let m3 = add_edge(c,a)?;

        new_faces.push(Face::new3(a,m1,m3));
        new_faces.push(Face::new3(m1,b,m2));
        new_faces.push(Face::new3(m2,c,m3));
        new_faces.push(Face::new3(m1,m2,m3));
    }
    Ok(new_faces)

}