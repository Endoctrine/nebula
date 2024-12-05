mod ray;
mod scene;
mod camera;
mod render;

use glam::Vec3;
use crate::camera::Camera;
use crate::scene::{Scene, Sphere};

fn create_test_scene() -> Scene {
    let mut scene = Scene::new();

    // 创建三个球体，分别放置在不同的位置
    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.1);  // 中心球体
    let sphere2 = Sphere::new(Vec3::new(2.0, 0.0, -10.0), 0.1);   // 右侧球体
    let sphere3 = Sphere::new(Vec3::new(-2.0, 0.0, 1.0), 0.1);  // 左侧球体

    scene.add(Box::new(sphere1));
    scene.add(Box::new(sphere2));
    scene.add(Box::new(sphere3));

    scene
}

fn create_camera() -> Camera {
    let look_from = Vec3::new(0.0, 0.0, 4.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    Camera::new(look_from, look_at, vup, 60.0, 16.0 / 9.0, 4.0, 0.0)
}


fn main() {
    let scene = create_test_scene();
    let camera = create_camera();

    let image_width = 800;
    let image_height = 450;

    let image_data = render::render(&scene, &camera, image_width, image_height);
    render::save_image_as_ppm(image_data, image_width, image_height, "output.ppm");
}