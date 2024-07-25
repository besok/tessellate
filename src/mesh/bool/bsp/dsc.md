```pseudo
// Data structures

struct Point3D:
    x, y, z: float

struct Plane:
    normal: Point3D
    distance: float

struct Polygon:
    vertices: list of Point3D

struct BSPNode:
    plane: Plane
    front: BSPNode
    back: BSPNode
    polygons: list of Polygon

// Main BSP tree construction function
function BuildBSPTree(polygons: list of Polygon) -> BSPNode:
    if polygons is empty:
        return null

    root = new BSPNode()
    root.plane = SelectSplittingPlane(polygons)
    root.polygons = []
    frontList = []
    backList = []

    for each polygon in polygons:
        classification = ClassifyPolygon(polygon, root.plane)
        if classification == COINCIDENT:
            root.polygons.append(polygon)
        else if classification == FRONT:
            frontList.append(polygon)
        else if classification == BACK:
            backList.append(polygon)
        else: // SPANNING
            frontPart, backPart = SplitPolygon(polygon, root.plane)
            frontList.append(frontPart)
            backList.append(backPart)

    root.front = BuildBSPTree(frontList)
    root.back = BuildBSPTree(backList)

    return root

// Helper functions

function SelectSplittingPlane(polygons: list of Polygon) -> Plane:
    // Simple selection: use the plane of the first polygon
    // More advanced methods could be implemented here
    return CreatePlaneFromPolygon(polygons[0])

function CreatePlaneFromPolygon(polygon: Polygon) -> Plane:
    normal = ComputeNormal(polygon)
    point = polygon.vertices[0]
    distance = DotProduct(normal, point)
    return Plane(normal, distance)

function ComputeNormal(polygon: Polygon) -> Point3D:
    v1 = SubtractPoints(polygon.vertices[1], polygon.vertices[0])
    v2 = SubtractPoints(polygon.vertices[2], polygon.vertices[0])
    return CrossProduct(v1, v2)

function ClassifyPolygon(polygon: Polygon, plane: Plane) -> Classification:
    numFront = 0
    numBack = 0
    
    for each vertex in polygon.vertices:
        distance = DotProduct(plane.normal, vertex) - plane.distance
        if distance > EPSILON:
            numFront++
        else if distance < -EPSILON:
            numBack++
        
    if numFront == 0 and numBack == 0:
        return COINCIDENT
    else if numBack == 0:
        return FRONT
    else if numFront == 0:
        return BACK
    else:
        return SPANNING

function SplitPolygon(polygon: Polygon, plane: Plane) -> (Polygon, Polygon):
    frontPoly = new Polygon()
    backPoly = new Polygon()
    
    for i from 0 to polygon.vertices.length - 1:
        current = polygon.vertices[i]
        next = polygon.vertices[(i + 1) % polygon.vertices.length]
        
        currentDist = DotProduct(plane.normal, current) - plane.distance
        nextDist = DotProduct(plane.normal, next) - plane.distance
        
        if currentDist >= 0:
            frontPoly.vertices.append(current)
        if currentDist <= 0:
            backPoly.vertices.append(current)
        
        if (currentDist > 0 and nextDist < 0) or (currentDist < 0 and nextDist > 0):
            t = currentDist / (currentDist - nextDist)
            intersectionPoint = InterpolatePoints(current, next, t)
            frontPoly.vertices.append(intersectionPoint)
            backPoly.vertices.append(intersectionPoint)
    
    return frontPoly, backPoly

function DotProduct(v1: Point3D, v2: Point3D) -> float:
    return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z

function CrossProduct(v1: Point3D, v2: Point3D) -> Point3D:
    return Point3D(
        v1.y * v2.z - v1.z * v2.y,
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x
    )

function SubtractPoints(p1: Point3D, p2: Point3D) -> Point3D:
    return Point3D(p1.x - p2.x, p1.y - p2.y, p1.z - p2.z)

function InterpolatePoints(p1: Point3D, p2: Point3D, t: float) -> Point3D:
    return Point3D(
        p1.x + t * (p2.x - p1.x),
        p1.y + t * (p2.y - p1.y),
        p1.z + t * (p2.z - p1.z)
    )

// Constants
EPSILON = 0.0001

// Enum for polygon classification
enum Classification:
    COINCIDENT, FRONT, BACK, SPANNING

// Usage example
function Main():
    polygons = LoadPolygonsFromFile("3d_model.obj")
    bspTree = BuildBSPTree(polygons)
    // Use the BSP tree for rendering, collision detection, etc.
```