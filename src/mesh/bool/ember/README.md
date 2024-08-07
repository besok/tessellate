The EMBER (Exact Mesh Booleans via Efficient and Robust Local Arrangements) algorithm 
is a method for performing Boolean operations on 3D meshes, ensuring robustness and efficiency. 
Here are the steps to implement the EMBER algorithm, 
assuming you already have an implementation for the SS KD-tree and winding numbers for polygons:

Steps to Implement the EMBER Algorithm
1. Preprocess the Input Meshes
   Input: Two meshes, A and B.
   Output: Prepared meshes with spatial subdivision and annotated winding numbers.
   Action: Use a spatial subdivision structure, such as an SS KD-tree, to preprocess the meshes. 
           This involves dividing the space into axis-aligned bounding boxes (AABBs) containing parts of the meshes.
2. Subdivide AABBs
      Input: Meshes with initial AABBs.
      Output: Refined AABBs for finer subdivision.
      Action: Subdivide AABBs into smaller regions if they contain parts of both meshes or if they are too large. 
              This helps in localizing the operations and making them more efficient.
3. Construct Winding Number Traces (WNTs)
   Input: Subdivided AABBs and mesh faces.
   Output: Annotated polygons (faces) with winding number trace values (Δwt).
   Action: For each polygon t, compute the winding number trace value Δwt by tracing the edges and determining 
           how crossing each edge affects the winding number.
4. Classify and Label Regions
   Input: Annotated polygons with Δwt.
   Output: Classified regions as inside, outside, or on the boundary.
   Action: Use the winding number information to classify regions within the subdivided AABBs. 
           Determine if a region is inside or outside each mesh by examining the winding numbers.
5. Perform Local Boolean Operations
   Input: Classified regions and winding number annotations.
   Output: Resulting mesh from the Boolean operation.
   Action: Perform Boolean operations (union, intersection, difference) 
           on the localized regions based on their classifications. 
           Merge the results from each region to obtain the final mesh.
6. Merge Results and Resolve Ambiguities
   Input: Local Boolean operation results.
   Output: Final Boolean operation result mesh.
   Action: Merge the results from all AABBs to form the final output mesh.
           Resolve any ambiguities or conflicts that arise during merging, 
           ensuring the robustness and correctness of the final mesh.
7. Post-process the Resulting Mesh
   Input: Final Boolean operation result mesh.
   Output: Cleaned and optimized mesh.
   Action: Clean up the resulting mesh by removing redundant vertices, edges, and faces. 
           Optimize the mesh structure for better performance and appearance.

