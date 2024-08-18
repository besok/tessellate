The code taken from [this](https://www.gianmarcocherchi.com/pdf/interactive_exact_booleans.pdf#page=5.58)


```  
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
def split_polygons(polygon_A, polygon_B):
    # Initialize an empty list to store the resulting polygons
    result_polygons = []

    # Step 1: Detect and process intersections and coinciding edges
    intersection_points = []
    coinciding_segments = []

    for edge_A in polygon_A.edges:
        for edge_B in polygon_B.edges:
            if edges_coincide(edge_A, edge_B):
                # Handle coinciding edges
                coinciding_segment = get_coinciding_segment(edge_A, edge_B)
                coinciding_segments.append(coinciding_segment)
                process_coinciding_edges(polygon_A, polygon_B, coinciding_segment)
            else:
                intersection_point = find_edge_intersection(edge_A, edge_B)
                if intersection_point:
                    # Handle the intersection point
                    process_intersection_point(polygon_A, polygon_B, intersection_point)
                    intersection_points.append(intersection_point)

    # Step 2: Merge intersection points with the original vertices
    vertices_A = merge_vertices_with_intersections(polygon_A, intersection_points)
    vertices_B = merge_vertices_with_intersections(polygon_B, intersection_points)

    # Step 3: Sort the vertices to form valid polygon segments
    sorted_vertices_A = sort_vertices(vertices_A)
    sorted_vertices_B = sort_vertices(vertices_B)

    # Step 4: Split polygon_A using the sorted vertices
    split_polygons_A = split_polygon_by_vertices(polygon_A, sorted_vertices_A)

    # Step 5: Split polygon_B using the sorted vertices
    split_polygons_B = split_polygon_by_vertices(polygon_B, sorted_vertices_B)

    # Step 6: Add all resulting split polygons to the result list
    result_polygons.extend(split_polygons_A)
    result_polygons.extend(split_polygons_B)

    # Step 7: Return the list of resulting polygons
    return result_polygons


def edges_coincide(edge_A, edge_B):
    # Check if the edges coincide along a segment
    return are_edges_parallel(edge_A, edge_B) and is_overlapping(edge_A, edge_B)


def get_coinciding_segment(edge_A, edge_B):
    # Calculate the overlapping segment of the coinciding edges
    start_point = max(edge_A.start, edge_B.start, key=lambda p: p.position)
    end_point = min(edge_A.end, edge_B.end, key=lambda p: p.position)
    if start_point < end_point:
        return Edge(start_point, end_point)
    return None


def process_coinciding_edges(polygon_A, polygon_B, coinciding_segment):
    # Split the polygons at the coinciding segment
    split_polygon_at_segment(polygon_A, coinciding_segment)
    split_polygon_at_segment(polygon_B, coinciding_segment)


def find_edge_intersection(edge_A, edge_B):
    # Implement edge intersection detection logic
    # Return the intersection point if they intersect, otherwise return None
    return calculate_intersection(edge_A, edge_B)


def process_intersection_point(polygon_A, polygon_B, intersection_point):
    # Add the intersection point to the list of vertices
    add_intersection_point(polygon_A, intersection_point)
    add_intersection_point(polygon_B, intersection_point)


def merge_vertices_with_intersections(polygon, intersection_points):
    # Merge original vertices with intersection points
    return polygon.vertices + intersection_points


def sort_vertices(vertices):
    # Sort vertices to ensure they form a valid polygon
    return sorted(vertices, key=lambda v: (v.x, v.y, v.z))


def split_polygon_by_vertices(polygon, sorted_vertices):
    # Use the sorted vertices to split the polygon into smaller polygons
    return create_sub_polygons(polygon, sorted_vertices)


def split_polygon_at_segment(polygon, coinciding_segment):
    # Split the polygon at the coinciding segment
    if coinciding_segment:
        new_polygons = generate_polygons_from_segment(polygon, coinciding_segment)
        return new_polygons
    return [polygon]  # No splitting needed if no coinciding segment


def generate_polygons_from_segment(polygon, segment):
    # Generate new polygons by splitting the original polygon at the segment
    new_polygons = []

    # Logic to create new polygons from the split, for example:
    # new_polygons.append(new_polygon1)
    # new_polygons.append(new_polygon2)
    # ...

    return new_polygons


def add_intersection_point(polygon, intersection_point):
    # Add the intersection point to the polygon's vertex list
    polygon.vertices.append(intersection_point)

def create_sub_polygons(polygon, sorted_vertices):
    """
    Create sub-polygons from a given polygon using sorted vertices.

    :param polygon: The original polygon to be split.
    :param sorted_vertices: The list of sorted vertices (including intersection points) used to split the polygon.
    :return: A list of sub-polygons generated from the original polygon.
    """

    sub_polygons = []
    current_polygon = []

    # Initialize the first vertex
    current_polygon.append(sorted_vertices[0])

    # Iterate over the sorted vertices and create edges between them
    for i in range(1, len(sorted_vertices)):
        current_vertex = sorted_vertices[i]

        # Add the current vertex to the current sub-polygon
        current_polygon.append(current_vertex)

        # Check if this vertex closes a sub-polygon (by checking if it's on the original polygon)
        if is_vertex_on_polygon(current_vertex, polygon):
            # Add the sub-polygon to the list and start a new sub-polygon
            sub_polygons.append(current_polygon)
            current_polygon = [current_vertex]

    # Final sub-polygon
    if current_polygon:
        sub_polygons.append(current_polygon)

    return sub_polygons


def is_vertex_on_polygon(vertex, polygon):
    """
    Check if a vertex is on the boundary of the polygon.

    :param vertex: The vertex to check.
    :param polygon: The polygon whose boundary is being checked.
    :return: True if the vertex is on the boundary, False otherwise.
    """
    for edge in polygon.edges:
        if is_point_on_edge(vertex, edge):
            return True
    return False


def is_point_on_edge(point, edge):
    """
    Check if a point is on a given edge.

    :param point: The point to check.
    :param edge: The edge to check against.
    :return: True if the point is on the edge, False otherwise.
    """
    # Edge direction vector
    edge_vector = edge.end - edge.start
    point_vector = point - edge.start

    # Cross product must be zero (point must be collinear with the edge)
    cross_product = edge_vector.cross(point_vector)
    if cross_product.magnitude() > 1e-8:
        return False

    # Check if the point lies within the segment range
    t = point_vector.dot(edge_vector) / edge_vector.dot(edge_vector)
    return 0 <= t <= 1

```