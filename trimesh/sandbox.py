import trimesh
import numpy as np

p1 = np.array([
    [0,0, 0],
    [0,1,0],
    [1,0,0],
])
p2 = np.array([
    [1.5,0,0],
    [0.5,0,0],
    [1.5, 1.5,0],
])
polygon1 = trimesh.creation.Polygon(p1[:, :2])

# Create the second polygon
polygon2 = trimesh.creation.Polygon(p2[:, :2])

intersection_result = polygon2.intersects(polygon2)
# Print the result
print(f"Do polygons intersect {intersection_result}")