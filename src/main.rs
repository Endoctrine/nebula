mod ray;
mod scene;
mod camera;
mod render;
mod material;
mod rand_util;

use std::sync::Arc;
use std::time::Instant;
use glam::Vec3;
use crate::camera::Camera;
use crate::material::Material;
use crate::scene::{Scene, primitive::*};

fn create_test_scene() -> Scene {
    let mut scene = Scene::new();

    let light = Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        0.5,
        Material::LUMINOUS,
    );
    scene.add(Box::new(light));

    // let mirror_sphere = Sphere::new(
    //     Vec3::new(1.0, -1.0, 0.1),
    //     0.4,
    //     Material::MIRROR,
    // );
    // scene.add(Box::new(mirror_sphere));
    //
    // let plaster_sphere = Sphere::new(
    //     Vec3::new(-1.0, -1.0, -0.1),
    //     0.4,
    //     Material::PLASTER,
    // );
    // scene.add(Box::new(plaster_sphere));
    //
    // let glass_sphere = Sphere::new(
    //     Vec3::new(0.0, -1.0, 0.0),
    //     0.4,
    //     Material::GLASS,
    // );
    // scene.add(Box::new(glass_sphere));

    // 加载测试场景
    scene.add_obj("scenes/cornell_box/CornellBox-Original.obj", Material::PLASTER);

    scene
}

fn create_camera() -> Camera {
    let look_from = Vec3::new(0.0, 1.0, 3.0);
    let look_at = Vec3::new(0.0, 1.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    Camera::new(look_from, look_at, vup, 60.0, 16.0 / 10.0, 4.0, 0.0)
}


fn main() {
    let mut scene = create_test_scene();
    let camera = create_camera();

    let image_width = 640;
    let image_height = 400;

    let start = Instant::now();
    scene.build_bvh();
    let image_data = render::render(Arc::new(scene), Arc::new(camera), image_width, image_height);
    let duration = start.elapsed();
    println!("Used {:?} to render 1 image(time for building bvh included).", duration);
    render::save_image_as_ppm(image_data, image_width, image_height, "output.ppm");
}