# Opt1

```text
function EMBER_Boolean(MeshA, MeshB, Operation):
    // Initialize data structures
    result = EmptyMesh()
    spatialGrid = CreateSpatialHashGrid()
    
    // Insert triangles into spatial grid
    for each triangle in MeshA and MeshB:
        spatialGrid.Insert(triangle)
    
    // Adaptive recursive subdivision
    function ProcessRegion(boundingBox):
        if CanEarlyTerminate(boundingBox):
            return
        
        if IsSmallEnough(boundingBox):
            // Perform local arrangement
            arrangement = ConstructLocalArrangement(boundingBox)
            subPolygons = ClassifySubPolygons(arrangement)
            for each subPolygon in subPolygons:
                if ShouldInclude(subPolygon, Operation):
                    result.Add(subPolygon)
        else:
            // Subdivide and recurse
            subRegions = SubdivideBoundingBox(boundingBox)
            for each subRegion in subRegions:
                ProcessRegion(subRegion)
    
    // Start the recursive process
    initialBoundingBox = ComputeBoundingBox(MeshA, MeshB)
    ProcessRegion(initialBoundingBox)
    
    // Post-processing
    result = StitchMesh(result)
    return result

function ConstructLocalArrangement(boundingBox):
    relevantTriangles = spatialGrid.QueryRegion(boundingBox)
    arrangement = CreateEmptyArrangement()
    for each triangle in relevantTriangles:
        arrangement.AddTriangle(triangle)
    arrangement.ComputeIntersections()
    return arrangement

function ClassifySubPolygons(arrangement):
    subPolygons = []
    for each face in arrangement:
        classification = ComputeWindingNumbers(face)
        subPolygons.Add(face, classification)
    return subPolygons

function ShouldInclude(subPolygon, Operation):
    switch Operation:
        case UNION:
            return subPolygon.InA || subPolygon.InB
        case INTERSECTION:
            return subPolygon.InA && subPolygon.InB
        case DIFFERENCE:
            return subPolygon.InA && !subPolygon.InB
```

This pseudocode outlines the main components of the EMBER algorithm:
- Initialization: Create a spatial hash grid for efficient intersection tests.
- Adaptive Recursive Subdivision: The ProcessRegion function recursively subdivides the bounding box, applying early termination criteria when possible.
- Local Arrangement Construction: For sufficiently small regions, construct a 2D arrangement of intersecting triangles.
- Sub-polygon Classification: Classify sub-polygons as inside or outside each input mesh using generalized winding numbers.
- Boolean Evaluation: Determine which sub-polygons should be included in the result based on the desired Boolean operation.
- Mesh Stitching: Connect adjacent sub-polygons to form a watertight output mesh (implemented in the StitchMesh function).

# Opt2

```text
Function EMBER(meshA, meshB, operation):
    # Step 1: Perform initial setup
    verticesA, facesA = meshA.getVerticesAndFaces()
    verticesB, facesB = meshB.getVerticesAndFaces()

    # Step 2: Compute the bounding volume hierarchy (BVH) for both meshes
    bvhA = computeBVH(verticesA, facesA)
    bvhB = computeBVH(verticesB, facesB)

    # Step 3: Find intersecting faces using BVH
    intersectingFaces = findIntersectingFaces(bvhA, bvhB)

    # Step 4: Compute exact intersections between intersecting faces
    intersectionPoints = computeExactIntersections(intersectingFaces)

    # Step 5: Classify vertices and faces into inside, outside, and boundary regions
    classifyVerticesAndFaces(verticesA, facesA, verticesB, facesB, intersectionPoints)

    # Step 6: Generate new mesh based on Boolean operation type
    If operation == "Union":
        resultMesh = generateUnionMesh(verticesA, facesA, verticesB, facesB, intersectionPoints)
    Else If operation == "Intersection":
        resultMesh = generateIntersectionMesh(verticesA, facesA, verticesB, facesB, intersectionPoints)
    Else If operation == "Difference":
        resultMesh = generateDifferenceMesh(verticesA, facesA, verticesB, facesB, intersectionPoints)
    Else:
        raise Exception("Invalid Boolean operation")

    # Step 7: Cleanup and return the resulting mesh
    cleanupTemporaryData()
    return resultMesh

Function computeBVH(vertices, faces):
    # Implementation of BVH computation
    Initialize BVH
    For each face in faces:
        Add face to BVH
    Return BVH

Function findIntersectingFaces(bvhA, bvhB):
    # Implementation of intersecting face detection using BVH
    Initialize list of intersecting faces
    For each node in bvhA:
        If node intersects with bvhB:
            Add intersecting faces to list
    Return intersectingFaces

Function computeExactIntersections(intersectingFaces):
    # Implementation of exact intersection computation
    Initialize list of intersection points
    For each pair of intersecting faces:
        Compute exact intersection points
        Add points to list
    Return intersectionPoints

Function classifyVerticesAndFaces(verticesA, facesA, verticesB, facesB, intersectionPoints):
    # Implementation of vertex and face classification
    For each vertex in verticesA:
        Classify vertex as inside, outside, or boundary with respect to meshB
    For each vertex in verticesB:
        Classify vertex as inside, outside, or boundary with respect to meshA

Function generateUnionMesh(verticesA, facesA, verticesB, facesB, intersectionPoints):
    # Implementation of union mesh generation
    Initialize result mesh
    Add vertices and faces from meshA and meshB
    Add intersection points
    Merge and simplify mesh
    Return result mesh

Function generateIntersectionMesh(verticesA, facesA, verticesB, facesB, intersectionPoints):
    # Implementation of intersection mesh generation
    Initialize result mesh
    Add intersecting regions from meshA and meshB
    Add intersection points
    Merge and simplify mesh
    Return result mesh

Function generateDifferenceMesh(verticesA, facesA, verticesB, facesB, intersectionPoints):
    # Implementation of difference mesh generation
    Initialize result mesh
    Add vertices and faces from meshA that are outside meshB
    Add intersection points
    Merge and simplify mesh
    Return result mesh

Function cleanupTemporaryData():
    # Implementation of temporary data cleanup
    Remove all temporary data structures

```