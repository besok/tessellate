use env_logger::{init, Builder};
use glam::Vec3;
use log::{info, LevelFilter};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::{Color, RgbaColor};
use tessellate::mesh::shape::parametric::supertoroid::Supertoroid;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::HasMesh;
use tessellate::{gpu, TessResult};
use tessellate::mesh::shape::parametric::bohemian_dome::BohemianDome;

pub fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}

fn main() -> TessResult<()> {
    // init_logger();
    let meshes = vec![
        bohemian_dome().into()
    ];

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera)?)
}

fn supertoroid() -> Supertoroid {
    Supertoroid::create(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        0.5,
        1.0,
        1.0,
        1.0,
        0.25,
        0.25,
        100,
        Color::default(),
    )
}
fn bohemian_dome() -> BohemianDome {
    BohemianDome::create(
        Vec3::new(0.0, 0.0, 0.0),
        1.0,
        1.0,
        1.0,
        20,
        20,
        Color::default(),
    )
}
