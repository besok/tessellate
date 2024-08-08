# Tessellate 
3D scientific data visualization and plotting tool for Rust.

Inspired by pyvista and VTK. 

## Features

### Data Loading and Creation.

#### Loading Existing Data.

Tessellate supports a wide range of file formats commonly used in scientific computing, 
including VTK (.vtk), STL (.stl), PLY (.ply), and many more (through integration with libraries like meshio). 
You can directly read data from these files into the objects for visualization and analysis.

#### Creating Basic Geometries.

The library provides functions to create fundamental geometric shapes like:
spheres, cubes, cuboids, cylinders, cones, rings, spheres, torus, planes and more from scratch. 
These objects serve as building blocks for more complex visualizations.
The detailed example can be found in the [basic_shapes](examples/sskd_tree/README.md) example.


### Mesh Manipulation.

#### Mesh Filtering. 

Tessellate offers a vast array of filters to manipulate and process existing meshes. 
    - Simplify meshes by reducing the number of points and faces while preserving overall shape.
    - Smooth surfaces to remove unwanted noise or sharp edges.
    - Extract specific features from a mesh, such as surfaces, edges, or individual points of interest.
    - Decimate meshes to reduce their complexity for faster rendering.
    - Apply various transformations like scaling, rotation, and translation for positioning meshes within your visualization.

#### Boolean Operations. 

Tessellate allows performing boolean operations like union, intersection, and difference on meshes. 
This is useful for combining or separating different 3D objects.

##### KDTree.

The library provides a KDTree implementation for efficient nearest neighbor searches in 3D space.

```rust
use crate::mesh::bool::kdtree::KDTree;
    use crate::mesh::parts::Vertex;
    use crate::mesh::shape::cone::Cone;
    use crate::mesh::HasMesh;

    #[test]
    fn kdtree_test() {
        let cone = Cone::default();
        let mesh = cone.mesh();
        let kdtree: KDTree = mesh.try_into().unwrap();

        let full_len = kdtree.nearest_neighbors(&Vertex::default(), None).count();
        let part_len = kdtree.nearest_neighbors(&Vertex::default(), Some(0.7)).count();

        assert_eq!(full_len, 62);
        assert_eq!(part_len, 14);

    }
```
 



##### BSP Tree.

The library provides a BSP Tree implementation for efficient point-in-polygon tests and spatial partitioning of 3D objects.

```rust
    fn bsp_tree_test() {
        let cone = Cone::default();
        let mesh = cone.mesh();
        let bsp: BSPTree = mesh.try_into().unwrap();
        for node in bsp.iter_inorder() {
            println!("{:?}", node);
        }
    }
    fn bsp_to_mesh_test() {
        turn_on_test_logs();
        let fig = Cone::default();
        let mesh = fig.mesh();
        let bsp: BSPTree = mesh.try_into().unwrap();
    
        let bsp_mesh = &bsp.mesh(Default::default());
        let planes = &bsp.plane_meshes(10.0,Color::default());
    
    }
```

### Visualization.

#### Interactive Plotting. 
The library facilitates creating interactive 3D plots using a plotter object. 

##### Color Mapping.
Assign colors to data points based on scalar values associated with the data. 

##### Lighting and Shading. 

Control lighting effects and shading models to enhance the visual representation of your data. 
Explore options like smooth shading, ambient lighting, and directional lighting.

##### Transparency.

Adjust the transparency of objects to reveal underlying structures or highlight specific features.

##### Camera Control.
Set the camera position and orientation to view your data from different angles.

##### Multiple Plots.

Create side-by-side comparisons of different datasets or visualizations.

##### Animations.
Tessellate enables generating animations to visualize dynamic processes or showcase different views of your data over time.

### Point Cloud Processing.

##### Loading and Visualizing Point Clouds.
The library can work with point cloud data, representing 3D objects 
as a collection of points with associated properties like color or intensity. 
You can load point cloud data from various formats and visualize them as point sets.

##### Point Cloud Analysis. 

Tessellate provides tools for analyzing point cloud data, such as:
    - Calculating distances between points.
    - Identifying nearest neighbors for each point.
    - Filtering points based on specific criteria.

