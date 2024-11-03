<h1 align="center">Tessellate</h1> 

<p align="center">
    <img width="150" alt="Logo" src="pics/logo.jpeg">
</p>
<h1 align="center">3D scientific data visualization and plotting tool for Rust.</h1>


--- 

Inspired by pyvista and VTK. 
Fully written in Rust for speed and safety.

# Features

## Data Loading and Creation.

Tessellate supports a range of file formats commonly used in scientific computing, 
including PLY (.ply), STL (.stl), OBJ (.obj). 
You can directly read data from these files into the objects for visualization and analysis.

- Import Obj Files
- Import Ply Files 
- Import Stl Files

The details can be found in the [import_models](examples/import_models/README.md) example.
The module `files` provides functions to read and write mesh data from and to files.


## Mesh Manipulation.

### Creating Basic Geometries.

The library provides functions to create fundamental geometric shapes like:
spheres, cubes, cuboids, cylinders, cones, rings, spheres, torus, planes and more from scratch.
These objects serve as building blocks for more complex visualizations.
The detailed example can be found in the [basic_shapes](examples/basic_shapes/README.md) example.

### Creating parametric Geometric Objects

The library provides functions to create parametric geometric shapes like:
supertoroids, parametric ellipsoids, partial parametric ellisoids, pseudospheres,spirals and more.
The detailed example can be found in the [parametric_shapes](examples/parametric_shapes/README.md) example.

### Creating an Explicit Structured Grid
### Creating a Structured Surface
### Creating a Triangulated Surface
### Platonic Solids
### Point Cloud


## Filtering
### Boolean Operations.
### Extract Cell Centers
The library provides functions to extract the centers of polygons and edges.
The detailed example can be found in the [extract_cell_centers](examples/polygon_and_edges_centers/README.md) example.

### Clipping with a Surface, plane and boxes
### Collision Detection
### Volumetric Analysis
### Find and label connected regions.
### Decimate a mesh
### Extract Edges
The library provides functions to extract the edges of a polygons.
The detailed example can be found in the [extract_edges](examples/extract_edges/README.md) example.

### Extract Surface
### Gaussian Smoothing
### Geodesic Paths
### Interpolating
### Computing Mesh Quality
### Resampling
### Surface Smoothing
### Surface Reconstruction
### Voxelize a Surface Mesh
### Subdivide Cells

## Advanced
### Visualize the Moeller-Trumbore Algorithm    
https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
https://cadxfem.org/inf/Fast%20MinimumStorage%20RayTriangle%20Intersection.pdf

### Ray Tracing
### Project points to a plane and Tessellate
generate a 3D point cloud, project it to a plane, and tessellate it.

## Visualization.

### Interactive Plotting. 
The library facilitates creating interactive 3D plots using a plotter object. 

### Color Mapping.
Assign colors to data points based on scalar values associated with the data. 

### Anti-Aliasing
### Measuring distances and angles
### Show Edges
### Legends and glyphs

### Lighting and Shading.
Control lighting effects and shading models to enhance the visual representation of your data. 
Explore options like smooth shading, ambient lighting, and directional lighting.

### Applying Textures
### Transparency.
### Camera Control.
### Animations.

## Auxiliary Tools and Data Structures.

### KDTree.

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

### BSP Tree.

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
