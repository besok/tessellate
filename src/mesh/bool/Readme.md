The code taken from [this](https://www.gianmarcocherchi.com/pdf/interactive_exact_booleans.pdf#page=5.58)


```python
def BooleanOperation(mesh_A, mesh_B, operation_type):
    # Step 1: Preprocessing
    octree_A = buildOctree(mesh_A)
    octree_B = buildOctree(mesh_B)
    
    # Step 2: Intersection Detection
    intersection_triangles = []
    for each leaf_node_A in octree_A:
        overlapping_nodes_B = findOverlappingLeafNodes(leaf_node_A, octree_B)
        for each leaf_node_B in overlapping_nodes_B:
            triangles_A = leaf_node_A.triangles
            triangles_B = leaf_node_B.triangles
            for each triangle_A in triangles_A:
                for each triangle_B in triangles_B:
                    if checkIntersection(triangle_A, triangle_B):
                        split_tris = splitTriangles(triangle_A, triangle_B)
                        intersection_triangles.extend(split_tris)
    
    # Step 3: Inside/Outside Testing
    final_triangles = []
    for each triangle in intersection_triangles:
        if operation_type == "Union":
            if isOutside(triangle, mesh_B):
                final_triangles.append(triangle)
        elif operation_type == "Intersection":
            if isInside(triangle, mesh_A) and isInside(triangle, mesh_B):
                final_triangles.append(triangle)
        elif operation_type == "Difference":
            if isOutside(triangle, mesh_B):
                final_triangles.append(triangle)
            else if operation_type == "Difference_BA":
                if isOutside(triangle, mesh_A):
                    final_triangles.append(triangle)
    
    # Step 4: Mesh Reconstruction
    final_mesh = reconstructMesh(final_triangles)
    return final_mesh
 


# Step 2: Intersection Detection
def checkIntersection(triangle_A, triangle_B):
    # Implement geometric intersection test between two triangles
    return intersecting_segments  # Return true if they intersect

def splitTriangles(triangle_A, triangle_B):
    # Step 1: Find intersection points (if any) between the edges of triangle_A and triangle_B
    intersection_points = []

    for each edge_A in edges_of(triangle_A):
        for each edge_B in edges_of(triangle_B):
            intersection_point = findEdgeIntersection(edge_A, edge_B)
            if intersection_point is not None:
                intersection_points.append(intersection_point)

    # Step 2: If there are no intersection points, return the original triangles
    if intersection_points is empty:
        return [triangle_A, triangle_B]

    # Step 3: Sort intersection points to form intersection segments
    intersection_segments = sortIntersectionPoints(intersection_points)

    # Step 4: Split triangle_A using the intersection segments
    split_triangles_A = splitTriangleBySegments(triangle_A, intersection_segments)

    # Step 5: Split triangle_B using the intersection segments
    split_triangles_B = splitTriangleBySegments(triangle_B, intersection_segments)

    # Step 6: Return the list of resulting triangles from both splits
    return split_triangles_A + split_triangles_B

def findEdgeIntersection(edge_A, edge_B):
    # Determine if edge_A and edge_B intersect, and if so, find the intersection point
    # Return the intersection point if they intersect, otherwise return None

    # Implement intersection logic (e.g., using parametric line equations)
    return intersection_point if they intersect, else return None

def sortIntersectionPoints(intersection_points):
    # Sort the intersection points to form a valid intersection segment
    # Sorting criteria will depend on the geometry of the triangle and intersection points
    return sorted_intersection_segments

def splitTriangleBySegments(triangle, segments):
    # Step 1: Identify how the segments divide the triangle
    split_triangles = []

    for each segment in segments:
        # Determine if the segment splits the triangle into two or more parts
        # Generate new triangles based on the split

        # Add new triangles to the list
        new_triangles = generateTrianglesFromSegment(triangle, segment)
        split_triangles.extend(new_triangles)

    # Step 2: Return the new smaller triangles
    return split_triangles

def generateTrianglesFromSegment(triangle, segment):
    # Given a triangle and a segment, generate new triangles formed by the split
    # This involves identifying the vertices and edges formed by the split and constructing new triangles

    new_triangles = []

    # Logic to create new triangles based on the segment split
    # Example: If a segment splits two edges of the triangle, create two new triangles

    return new_triangles
    
    
def findIntersectionPoints(triangle_A, triangle_B):
    # Compute intersection points between the edges of the triangles
    return points

def splitTrianglesAtPoints(triangle_A, triangle_B, points):
    # Split triangles based on intersection points
    return new_triangles  # Return the set of split triangles


# Step 3: Inside/Outside Testing
def isInside(triangle, mesh):
    # Determine if a triangle is inside a mesh using ray-casting or winding number method
    return inside  # Boolean

def isOutside(triangle, mesh):
    return not isInside(triangle, mesh)


# Step 4: Mesh Reconstruction
def reconstructMesh(triangles):
    # Step 4.1: Vertex Merging
    vertices, faces = mergeVertices(triangles)
    
    # Step 4.2: Edge Stitching
    faces = stitchEdges(vertices, faces)
    
    # Step 4.3: Remove Degenerate Triangles
    faces = removeDegenerateTriangles(faces)
    
    # Step 4.4: Mesh Simplification (Optional)
    faces = simplifyMesh(vertices, faces)
    
    # Step 4.5: Boundary Fixing (Optional)
    faces = fixBoundaries(vertices, faces)
    
    # Step 4.6: Final Validation
    validateMesh(vertices, faces)
    
    return createFinalMesh(vertices, faces)

def mergeVertices(triangles):
    # Merges vertices that are close together into unique vertices
    unique_vertices = {}
    faces = []
    for each triangle in triangles:
        for each vertex in triangle.vertices:
            if vertex is not in unique_vertices:
                unique_vertices[vertex] = len(unique_vertices)
        face = [unique_vertices[vertex] for vertex in triangle.vertices]
        faces.append(face)
    return list(unique_vertices.keys()), faces

def stitchEdges(vertices, faces):
    # Ensure that adjacent triangles are connected by their edges
    stitched_faces = []
    edge_map = {}  # Maps edges to adjacent faces
    for face in faces:
        for edge in getEdges(face):
            if edge in edge_map:
                edge_map[edge].append(face)
            else:
                edge_map[edge] = [face]
    
    # Merge faces sharing the same edge
    for edge, adjacent_faces in edge_map.items():
        if len(adjacent_faces) == 2:
            mergeFaces(adjacent_faces)
    return stitched_faces

def removeDegenerateTriangles(faces):
    # Removes triangles that have zero area or overlapping vertices
    cleaned_faces = []
    for face in faces:
        if not isDegenerate(face):
            cleaned_faces.append(face)
    return cleaned_faces

def simplifyMesh(vertices, faces):
    # Simplifies the mesh by reducing the number of triangles
    # This might involve collapsing edges, removing small features, etc.
    return simplified_faces

def fixBoundaries(vertices, faces):
    # Ensures that the mesh has no holes or gaps (watertight mesh)
    return fixed_faces

def validateMesh(vertices, faces):
    # Ensures the final mesh is valid (manifold, no holes, etc.)
    if not isManifold(faces):
        raise "Mesh is not manifold!"
    if hasHoles(faces):
        raise "Mesh has holes!"

def createFinalMesh(vertices, faces):
    return Mesh(vertices, faces)


```

## Split triangles

```python
def splitTriangles(triangle_A, triangle_B):
    """
    Splits two intersecting triangles into smaller triangles based on their intersection.
    
    :param triangle_A: The first triangle to be split.
    :param triangle_B: The second triangle to be split.
    :return: A list of resulting sub-triangles after the split.
    """

    # Step 1: Initialize an empty list to store intersection points
    intersection_points = []

    # Step 2: Iterate over each edge of triangle_A and triangle_B to find intersection points
    for edge_A in edges_of(triangle_A):
        for edge_B in edges_of(triangle_B):
            # Find the intersection point between edge_A and edge_B
            intersection_point = findEdgeIntersection(edge_A, edge_B)

            # If an intersection point is found, add it to the list
            if intersection_point is not None:
                intersection_points.append(intersection_point)

    # Step 3: If no intersection points are found, return the original triangles
    if len(intersection_points) == 0:
        return [triangle_A, triangle_B]

    # Step 4: Sort the intersection points to form valid intersection segments
    sorted_intersection_segments = sortIntersectionPoints(intersection_points)

    # Step 5: Split triangle_A using the sorted intersection segments
    split_triangles_A = splitTriangleBySegments(triangle_A, sorted_intersection_segments)

    # Step 6: Split triangle_B using the sorted intersection segments
    split_triangles_B = splitTriangleBySegments(triangle_B, sorted_intersection_segments)

    # Step 7: Combine the split triangles from both triangle_A and triangle_B
    resulting_triangles = split_triangles_A + split_triangles_B

    # Step 8: Return the list of resulting sub-triangles
    return resulting_triangles


def edges_of(triangle):
    """
    Returns the edges of a triangle.
    
    :param triangle: A triangle object.
    :return: A list of edges (each edge is a pair of vertices).
    """
    return [
        (triangle.vertices[0], triangle.vertices[1]),
        (triangle.vertices[1], triangle.vertices[2]),
        (triangle.vertices[2], triangle.vertices[0])
    ]


def findEdgeIntersection(edge_A, edge_B):
    """
    Finds the intersection point between two edges, if any.
    
    :param edge_A: The first edge (pair of vertices).
    :param edge_B: The second edge (pair of vertices).
    :return: The intersection point, or None if they do not intersect.
    """
    # Implement geometric intersection test between the two edges
    # Use parametric equations or other methods to find the intersection point
    # Return the intersection point if they intersect, otherwise return None
    pass  # Replace with actual implementation


def sortIntersectionPoints(intersection_points):
    """
    Sorts the intersection points to form valid intersection segments.
    
    :param intersection_points: A list of intersection points.
    :return: A list of sorted intersection segments.
    """
    # Sort the points based on their position on the edges (e.g., using distance along the edge)
    # Return the sorted list of points or segments
    pass  # Replace with actual implementation


def splitTriangleBySegments(triangle, segments):
    """
    Splits a triangle into smaller triangles based on intersection segments.
    
    :param triangle: The original triangle to be split.
    :param segments: The intersection segments that divide the triangle.
    :return: A list of smaller triangles resulting from the split.
    """
    split_triangles = []

    # Step 1: For each segment, determine how it splits the triangle
    for segment in segments:
        # Check how the segment intersects the triangle's edges
        # Determine the new vertices and edges formed by the segment

        # Generate new triangles based on the split
        new_triangles = generateTrianglesFromSegment(triangle, segment)

        # Add the new triangles to the list of split triangles
        split_triangles.extend(new_triangles)

    # Step 2: Return the list of resulting sub-triangles
    return split_triangles


def generateTrianglesFromSegment(triangle, segment):
    """
    Generates new triangles from a triangle split by a segment.
    
    :param triangle: The original triangle.
    :param segment: The segment that splits the triangle.
    :return: A list of new triangles formed by the split.
    """
    new_triangles = []

    # Logic to create new triangles based on the split
    # Example: If a segment splits two edges of the triangle, create two new triangles
    # This typically involves finding new vertices and forming new edges

    # Placeholder for the actual implementation logic
    pass  # Replace with actual implementation

    return new_triangles


```
