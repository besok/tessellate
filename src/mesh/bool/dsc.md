
```text
function EMBER_Boolean(meshA, meshB, operation):
    // Convert input meshes to plane-based representation
    planesA = ConvertToPlaneBased(meshA)
    planesB = ConvertToPlaneBased(meshB)
    
    // Compute bounding box
    boundingBox = ComputeBoundingBox(planesA, planesB)
    
    // Perform adaptive recursive subdivision
    result = AdaptiveSubdivision(boundingBox, planesA, planesB, operation)
    
    return result

function AdaptiveSubdivision(box, planesA, planesB, operation):
    if CanTerminateEarly(box, planesA, planesB):
        return EmptyMesh()
    
    if IsSmallEnough(box):
        return ComputeLocalArrangement(box, planesA, planesB, operation)
    
    // Subdivide the box
    subBoxes = SubdivideBox(box)
    results = []
    
    for each subBox in subBoxes:
        subResult = AdaptiveSubdivision(subBox, planesA, planesB, operation)
        results.append(subResult)
    
    return MergeResults(results)

function ComputeLocalArrangement(box, planesA, planesB, operation):
    // Compute pairwise intersections
    intersections = ComputePairwiseIntersections(planesA, planesB)
    
    // Integrate intersections into local BSPs
    localBSPs = IntegrateIntoBSPs(intersections)
    
    // Classify polygons using winding number tracing
    classifiedPolygons = ClassifyPolygons(localBSPs, operation)
    
    return classifiedPolygons

function ClassifyPolygons(localBSPs, operation):
    classifiedPolygons = []
    for each polygon in localBSPs:
        windingNumber = TraceWindingNumber(polygon)
        if ShouldKeepPolygon(windingNumber, operation):
            classifiedPolygons.append(polygon)
    return classifiedPolygons

function TraceWindingNumber(polygon):
    referencePoint = GetLocalReferencePoint()
    segments = GenerateTraceSegments(polygon, referencePoint)
    return ComputeWindingNumberAlongSegments(segments)

// Helper functions
function CanTerminateEarly(box, planesA, planesB):
    // Implement early termination criteria
    ...

function IsSmallEnough(box):
    // Check if box size is below threshold
    ...

function SubdivideBox(box):
    // Implement box subdivision
    ...

function MergeResults(results):
    // Merge subresults
    ...
```

 