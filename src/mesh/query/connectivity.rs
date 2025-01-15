use crate::mesh::parts::edge::MeshEdge;
use crate::mesh::{Mesh, MeshResult};
use petgraph::algo::{kosaraju_scc, tarjan_scc};
use petgraph::graph::UnGraph;

pub fn connected_regions(mesh: &Mesh) -> MeshResult<Vec<Mesh>> {
    let mut graph = UnGraph::new_undirected();
    let mut graph_indexes = vec![];
    for v in mesh.vertices() {
        graph_indexes.push(graph.add_node(v));
    }
    for MeshEdge(a, b) in mesh.edges() {
        let lhs = graph_indexes.get(*a).ok_or("Invalid edge index")?;
        let rhs = graph_indexes.get(*b).ok_or("Invalid edge index")?;
        graph.add_edge(lhs.clone(), rhs.clone(), ());
    }

    let mut meshes = vec![];
    let tables = mesh.try_tables()?;

    for v_indexes in tarjan_scc(&graph) {
        let mut polygons = vec![];
        for v in v_indexes {
            let faces = tables
                .vertex_faces_idx(v.index())
                .ok_or("Invalid vertex index")?;
            for f in faces {
                polygons.push(mesh.face_idx_to_polygon(*f)?);
            }
        }
        meshes.push(Mesh::from_polygons(polygons, mesh.attributes.clone()))
    }
    Ok(meshes)
}
