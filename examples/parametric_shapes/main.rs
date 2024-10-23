use env_logger::{init, Builder};
use glam::{Mat4, Vec3};
use log::{info, LevelFilter};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::mesh::material::{Color, RgbaColor};
use tessellate::mesh::shape::parametric::bohemian_dome::BohemianDome;
use tessellate::mesh::shape::parametric::bour::Bour;
use tessellate::mesh::shape::parametric::boy::Boy;
use tessellate::mesh::shape::parametric::conic_spiral::ConicSpiral;
use tessellate::mesh::shape::parametric::dini::Dini;
use tessellate::mesh::shape::parametric::ellipsoid::Ellipsoid;
use tessellate::mesh::shape::parametric::mobius::MobiusStrip;
use tessellate::mesh::shape::parametric::pseudo_sphere::Pseudosphere;
use tessellate::mesh::shape::parametric::super_ellipsoid::SuperEllipsoid;
use tessellate::mesh::shape::parametric::super_toroid::Supertoroid;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::{HasMesh, MeshResult};
use tessellate::{gpu, TessResult};

pub fn init_logger() {
    Builder::new().filter(None, LevelFilter::Info).init();
    info!("Logger initialized");
}

fn main() -> TessResult<()> {
    init_logger();
    let meshes = vec![
        pseudo_sphere().into(),
        supertoroid()?.into(),
        bohemian_dome().into(),
        // super_toroidoid().into(),
        // super_ellipsoid().into(),
        // mobius_strip().into(),
        // ellipsoid().into(),
        // spiral().into(),
        // dini().into(),
        // bour().into(),
        // boy().into(),
    ];

    let camera = CameraPosition::new(Vec3::new(-3.5, 0.0, 0.0), 0.0, 0.0);
    Ok(gpu::visualize(meshes, camera )?)
}

fn supertoroid() -> MeshResult<Supertoroid> {
    let mut elem = Supertoroid::create(
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
    );
    let _ = elem.transform(Mat4::from_translation(Vec3::new(2.0, 2.0, 2.0)));
    Ok(elem)
}
fn bohemian_dome() -> BohemianDome {
    let mut elem = BohemianDome::create(Vec3::new(0.0, 0.0, 0.0), 1.0, 1.0, 1.0, 20, 20, Color::default());
    let _ = elem.transform(Mat4::from_translation(Vec3::new(-2.0, -2.0, -2.0)));
    elem
}
fn bour() -> Bour {
    Bour::create(Vec3::new(0.0, 0.0, 0.0), 50, 4.0, 50, 0., 0.5, Color::default())
}
fn boy() -> Boy {
    Boy::create(Vec3::new(0.0, 0.0, 0.0), 100, Color::default())
}

fn dini() -> Dini {
    Dini::create(Vec3::new(0.0, 0.0, 0.0), 100, 1., 0.2, Color::default())
}
fn spiral() -> ConicSpiral {
    ConicSpiral::default()
}
fn ellipsoid() -> Ellipsoid {
    Ellipsoid::default()
}
fn mobius_strip() -> MobiusStrip {
    MobiusStrip::default()
}
fn super_ellipsoid() -> SuperEllipsoid {
    SuperEllipsoid::default()
}
fn pseudo_sphere() -> Pseudosphere {
    Pseudosphere::default()
}
