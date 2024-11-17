use env_logger::Builder;
use glam::{Mat4, Vec3};
use log::{info, LevelFilter};
use tessellate::gpu::options::{CameraOptions, GpuOptions, LightOptions};
use tessellate::mesh::material::{Color, RgbaColor};
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh, MeshError};
use tessellate::{files, gpu, TessError, TessResult};
use tobj::LoadOptions;

pub fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}

fn main() -> TessResult<()> {
    // init_logger();
    let mut cow =
        files::obj::import_obj("examples/import_models/cow.obj", &LoadOptions::default())?;
    cow.set_color(Color::Mesh(RgbaColor::GREEN));
    cow.attributes_mut().with_affected_by_light(false);
    // let bunny = files::ply::import_ply("examples/import_models/bunny.ply")?;
    // let building = files::stl::import_stl("examples/import_models/at_t_building.stl")?;

    let meshes = vec![
        cow,
        // building,
        // bunny,
    ];
    let options = GpuOptions::new(
        CameraOptions::new_position(Vec3::new(2., 3., 15.)),
        LightOptions::new_position(Vec3::new(2., 5., 0.))
            .with_show_source(true)
            .clone(),
    );

    Ok(gpu::visualize(meshes, options)?)
}
