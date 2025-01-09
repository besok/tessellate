# Distance 

Calculate the distance in 3D space.

It can be used to calculate the distance between:

 - two vertices
 - a vertex and a mesh
 - two meshes

The running example can be found
in the <a href="https://github.com/besok/tessellate/tree/main/examples/distance" target="_blank">repository</a>

## Notes

The distance is calculated using the Euclidean distance formula.
The meshes must be watertight and closed.
It uses existing KDTree implementation with vertices so the precision is limited to the vertices.