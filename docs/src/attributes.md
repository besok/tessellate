# Attributes

The attributes that can be set up on the mesh and provided to use for visualization or other purposes.


## MeshType

The type of mesh that can be created. The following types are available:
 - Polygons: A mesh that is made up of polygons.
 - Lines: A mesh that is made up of lines.
 - Cloud: A mesh that is made up of points.

## Material
The material for the mesh that affects how it will interact with the light. 
The following properties can be set:
 - Ambient: The ambient color of the material.
 - Diffuse: The diffuse color of the material.
 - Specular: The specular color of the material.
 - Shininess: The shininess of the material.
 
## Color
The color of the mesh. 
The color can be set as a single color or as a gradient.
The color is set up as a tuple of 4 values (r, g, b, a) 
where r, g, b are the red, green, blue values and a is the alpha value.

Can be set up for the following parts of the mesh:
 - Vertex: A color assigned to each vertex.
 - Face: A color assigned to each face.
 - Line: A color assigned to each line.
 - Mesh: A single color for the entire mesh.
 - Func: A function that assigns a color a vertex with some logic.



 