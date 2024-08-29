import trimesh
import numpy as np

p1 = np.array([
    [-2.5, -2.5, 0],
    [2.5, -2.5, 0],
    [0, 0, 5],
])
p2 = np.array([
    [2.5, 2.5, 0],
    [2.5, -2.5, 0],
    [0, 0, 5],
])
polygon1 = trimesh.creation.Polygon(p1[:, :2])

# Create the second polygon
polygon2 = trimesh.creation.Polygon(p2[:, :2])

intersection_result = polygon2.intersects(polygon1)
# Print the result
print("Do polygons intersect?", intersection_result)