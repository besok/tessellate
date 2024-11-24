# Extract Edges

It provides the means to extract the edges of a mesh.


The running example can be found
in the <a href="https://github.com/besok/tessellate/tree/main/examples/extract_edges" target="_blank">repository</a>

For details see [query](./query.md) and the methods: 

### extract_boundary_edges

This function identifies and returns the edges that are on the boundary of the mesh. Boundary edges are those that belong to only one face.

### extract_manifold_edges

This function identifies and returns the edges that are shared by exactly two faces.

### extract_non_manifold_edges

This function identifies and returns the edges that are shared by more than two faces.

### extract_feature_edges

This function identifies and returns the edges that
