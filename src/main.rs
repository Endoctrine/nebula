mod ray;
mod scene;
mod camera;
mod render;

use std::time::Instant;
use glam::Vec3;
use crate::camera::Camera;
use crate::scene::{Scene, Sphere};

fn create_test_scene() -> Scene {
    let mut scene = Scene::new();

    // 在场景中创建 10000 个球体
    for i in 0..100 {
        for j in 0..100 {
            let sphere = Sphere::new(
                Vec3::new(
                    (i as f32 - 50.0) / 20.0,
                    (j as f32 - 50.0) / 20.0,
                    0.0,
                ),
                0.02,
            );
            scene.add(Box::new(sphere));
        }
    }

    scene
}

fn create_camera() -> Camera {
    let look_from = Vec3::new(0.0, 0.0, 4.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    Camera::new(look_from, look_at, vup, 60.0, 16.0 / 9.0, 4.0, 0.0)
}


fn main() {
    let mut scene = create_test_scene();
    let camera = create_camera();

    let image_width = 2500;
    let image_height = 1600;

    let start = Instant::now();
    let image_data = render::render(&mut scene, &camera, image_width, image_height);
    let duration = start.elapsed();
    println!("Used {:?} to render 1 image.", duration);
    render::save_image_as_ppm(image_data, image_width, image_height, "output.ppm");
}