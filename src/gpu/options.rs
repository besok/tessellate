use crate::mesh::material::RgbaColor;
use glam::Vec3;

#[derive(Debug, Clone, Default)]
pub struct GpuOptions {
    light_opts: LightOptions,
    camera_opts: CameraOptions,
}

impl GpuOptions {
    pub fn new(camera_opts: CameraOptions, light_opts: LightOptions) -> Self {
        Self {
            camera_opts,
            light_opts,
        }
    }

    pub fn new_only_camera_pos(pos: Vec3) -> Self {
        Self {
            camera_opts: CameraOptions::new_position(pos),
            light_opts: LightOptions::default(),
        }
    }

    pub fn new_only_camera_light_pos(camera_pos: Vec3, light_pos: Vec3) -> Self {
        Self {
            camera_opts: CameraOptions::new_position(camera_pos),
            light_opts: LightOptions::new_position(light_pos),
        }
    }

    pub fn with_light(&mut self, light: LightOptions) -> &Self {
        self.light_opts = light;
        self
    }

    pub fn with_camera(&mut self, camera: CameraOptions) -> &Self {
        self.camera_opts = camera;
        self
    }

    pub fn camera_opts(&self) -> &CameraOptions {
        &self.camera_opts
    }

    pub fn light_opts(&self) -> &LightOptions {
        &self.light_opts
    }
}

#[derive(Debug, Clone)]
pub struct CameraOptions {
    position: Vec3,
    fovy: f32,
    znear: f32,
    zfar: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraOptions {
    pub fn new_position(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn with_position(&mut self, position: Vec3) -> &Self {
        self.position = position;
        self
    }

    pub fn with_fovy(&mut self, fovy: f32) -> &Self {
        self.fovy = fovy;
        self
    }

    pub fn with_znear(&mut self, znear: f32) -> &Self {
        self.znear = znear;
        self
    }

    pub fn with_zfar(&mut self, zfar: f32) -> &Self {
        self.zfar = zfar;
        self
    }

    pub fn with_speed(&mut self, speed: f32) -> &Self {
        self.speed = speed;
        self
    }

    pub fn with_sensitivity(&mut self, sensitivity: f32) -> &Self {
        self.sensitivity = sensitivity;
        self
    }
}

impl CameraOptions {
    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn fovy(&self) -> f32 {
        self.fovy
    }

    pub fn znear(&self) -> f32 {
        self.znear
    }

    pub fn zfar(&self) -> f32 {
        self.zfar
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn sensitivity(&self) -> f32 {
        self.sensitivity
    }
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 3.0, 3.0),
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            speed: 0.1,
            sensitivity: 0.1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LightOptions {
    position: Vec3,
    ambient: Vec3,
    diffuse: Vec3,
    specular: Vec3,
    show_source: bool,
}

impl Default for LightOptions {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 3.0, 3.0),
            ambient: Vec3::new(0.4, 0.4, 0.4),
            diffuse: Vec3::new(0.8, 0.8, 0.8),
            specular: Vec3::new(0.8, 0.8, 0.8),
            show_source: false,
        }
    }
}

impl LightOptions {
    pub fn new(
        position: Vec3,
        ambient: Vec3,
        diffuse: Vec3,
        specular: Vec3,
        show_source: bool,
    ) -> Self {
        Self {
            position,
            ambient,
            diffuse,
            specular,
            show_source,
        }
    }
    pub fn new_position(position: Vec3) -> Self {
        Self {
            position,
            ..Default::default()
        }
    }

    pub fn with_position(&mut self, position: Vec3) -> &Self {
        self.position = position;
        self
    }

    pub fn with_ambient(&mut self, ambient: Vec3) -> &Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(&mut self, diffuse: Vec3) -> &Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(&mut self, specular: Vec3) -> &Self {
        self.specular = specular;
        self
    }

    pub fn with_show_source(&mut self, show_source: bool) -> &Self {
        self.show_source = show_source;
        self
    }
}

impl LightOptions {
    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn ambient(&self) -> Vec3 {
        self.ambient
    }

    pub fn diffuse(&self) -> Vec3 {
        self.diffuse
    }

    pub fn specular(&self) -> Vec3 {
        self.specular
    }

    pub fn show_source(&self) -> bool {
        self.show_source
    }
}

impl Into<RgbaColor> for &LightOptions {
    fn into(self) -> RgbaColor {
        let combined = self.ambient + self.diffuse + self.specular;
        let max_component = combined.max_element();
        let normalized = if max_component > 1.0 {
            combined / max_component
        } else {
            combined
        };

        RgbaColor([
            (normalized.x * 255.0).clamp(0.0, 255.0) as u8,
            (normalized.y * 255.0).clamp(0.0, 255.0) as u8,
            (normalized.z * 255.0).clamp(0.0, 255.0) as u8,
            255,
        ])
    }
}
