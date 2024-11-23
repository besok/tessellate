# Gnu Options
The options module provides the following options for the visualization 

## Camera options
### Camera position
The initial positions for the camera. The position is a vector in the space. The default value is [0, 3, 3].

### Camera fovy
The field of view of the camera. The default value is 45. That means that the camera has a 45 degrees field of view.

### Camera znear
The near clipping plane of the camera. The default value is 0.1.
It is the distance from the camera to the near clipping plane.

### Camera zfar
The far clipping plane of the camera. The default value is 1000.
It is the distance from the camera to the far clipping plane.

### Camera speed and sensitivity
The speed and sensitivity of the camera. The default value is 0.1.
It is the speed of the camera when moving and the sensitivity of the camera when rotating.

## Light options

### Light position
The initial positions for the light. The position is a vector in the space. The default value is [0, 3, 3].

### Light ambient
Light ambient color. The ambient affects all objects in the scene equally.
This helps to ensure that no part of the scene is completely dark, 
even if it is not directly illuminated by a light source.
The default value is [0.4, 0.4, 0.4].

### Light diffuse
Light diffuse color. The diffuse color is the color of the light that is reflected off a surface.
The default value is [0.8, 0.8, 0.8].

### Light specular
Light specular color. The specular color is the color of the light that is reflected off a surface.
The default value is [0.8, 0.8, 0.8].

### Show light source
Show the light source in the scene. The default value is false.

###  How to affect intensity of the light
The intensity of the light is calculated as the sum of the ambient, diffuse, and specular colors.
To get higher intensity, you can increase the values of the colors.