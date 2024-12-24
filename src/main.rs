mod ray;
mod scene;
mod camera;
mod render;
mod material;
mod rand_util;

use std::sync::Arc;
use std::time::Instant;
use glam::{Mat4, Vec3};
use crate::camera::Camera;
use crate::scene::{Scene};

fn create_test_scene() -> Scene {
    let mut scene = Scene::new();


    // 加载测试场景
    let transform = Mat4::IDENTITY;
    scene.add_obj("scenes/cornell_box/CornellBox-Empty-CO.obj", transform);
    let translate = Mat4::from_translation(
        Vec3::new(0.0, 0.5, -0.5)
    );
    let squeeze = Mat4::from_scale(Vec3::new(1.0, 0.5, 1.0));
    let rotate = Mat4::from_rotation_x(std::f32::consts::PI / 4.0);
    scene.add_obj("scenes/my_name/name.obj",  translate * rotate * squeeze);

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