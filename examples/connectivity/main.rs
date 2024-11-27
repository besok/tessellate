use glam::{Mat4, Quat, Vec3};
use tessellate::gpu::camera::position::CameraPosition;
use tessellate::gpu::options::{CameraOptions, GpuOptions, LightOptions};
use tessellate::mesh::attributes::Attributes;
use tessellate::mesh::material::{Color, RgbaColor};
use tessellate::mesh::parts::face::FaceType;
use tessellate::mesh::parts::vertex::Vertex;
use tessellate::mesh::shape::cone::Cone;
use tessellate::mesh::shape::cuboid::cube::Cube;
use tessellate::mesh::shape::cylinder::Cylinder;
use tessellate::mesh::shape::icosahedron::Icosahedron;
use tessellate::mesh::shape::pyramid::Pyramid;
use tessellate::mesh::shape::ring::Ring;
use tessellate::mesh::shape::sphere::Sphere;
use tessellate::mesh::shape::torus::Torus;
use tessellate::mesh::transform::Transform;
use tessellate::mesh::HasMesh;
use tessellate::{files, gpu, TessError, TessResult};
use tobj::LoadOptions;

fn main() -> TessResult<()> {
    let mut foot_bones = files::ply::import_ply("examples/connectivity/footbones.ply")?;
    foot_bones.attributes_mut().set_color(RgbaColor::BLUE.into());


    let mut largest = foot_bones.query().extract_largest_connected_region()?;
    largest.attributes_mut().set_color(RgbaColor::RED.into());

    let mut closest = foot_bones
        .query()
        .extract_closest_connected_region(&Vertex::new(1.0, 1.0, 0.0))?;
    closest.attributes_mut().set_color(RgbaColor::GREEN.into());


    let meshes = vec![foot_bones, largest,closest];
    let options = GpuOptions::new(
        CameraOptions::new_position(Vec3::new(2., 3., 15.)),
        LightOptions::new_position(Vec3::new(2., 5., 0.))
            .with_show_source(true)
            .clone()
            .with_background_color(RgbaColor::GRAY)
            .clone(),
    );

    Ok(gpu::visualize(meshes, options)?)
}
