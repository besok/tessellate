import trimesh
import numpy as np

p1 = np.array([
    [-2.5, -2.5, 0.0],
    [2.5, -2.5, 0.0],
    [0.0, 0.0, 5.0],
])
p2 = np.array([
    [2.5, -2.5, 0.0],
    [2.5, 2.5, 0.0],
    [0.0, 0.0, 5.0],
])
polygon1 = trimesh.creation.Polygon(p1[:, :2])
polygon2 = trimesh.creation.Polygon(p2[:, :2])

intersection_result = polygon1.intersects(polygon2)
# Print the result
print(f"Do polygons intersect {intersection_result}")