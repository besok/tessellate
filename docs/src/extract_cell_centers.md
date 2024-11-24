# Extract polygon and edges centers

It provides the means to extract the centers of the polygons and edges of a mesh. 
The centers are computed as the average of the vertices that define the polygon or edge. 
The centers are stored in a new mesh object. 
The new mesh object is a point cloud with the centers as vertices.


The running example can be found
in the <a href="https://github.com/besok/tessellate/tree/main/examples/polygon_and_edges_centers" target="_blank">repository</a>

For details see [query](./query.md) and `extract_poly_centers` and `extract_edge_centers` methods.
