use env_logger::Builder;
use glam::{Mat4, Vec3};
use log::{info, LevelFilter};
use tobj::LoadOptions;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, Mesh, MeshError};
use tessellate::{files, gpu, TessError, TessResult};

pub fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}

fn main() -> TessResult<()> {
    // init_logger();
    let cow = files::obj::import_obj("examples/import_models/cow.obj", &LoadOptions::default())?;

    let bunny = files::ply::import_ply("examples/import_models/bunny.ply")?;
    let building = files::stl::import_stl("examples/import_models/at_t_building.stl")?;

    let meshes = vec![
        cow,
        // building,
        // bunny,
    ];
    Ok(gpu::visualize(meshes, Vec3::new(15.,0.,3.).into())?)
}
