mod basic;

use env_logger::Builder;
use glam::{Mat4, Vec3};
use log::{info, LevelFilter};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::RgbaColor;
use tessellate::mesh::parts::{BoundingBox, FaceType};
use tessellate::mesh::shape::cone::Cone;
use tessellate::{gpu, TessError};

fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}

fn main() {}