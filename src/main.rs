mod ray;
mod scene;
mod camera;
mod render;
mod material;
mod rand_util;
mod texture;

use std::sync::Arc;
use std::time::Instant;
use glam::{Mat4, Vec3};
use crate::camera::Camera;
use crate::scene::{Scene};

fn create_test_scenes() -> Vec<Scene> {
    let mut scenes = vec![];

    // 场景一：CornellBoxMirror
    // 场景一包含光源、石膏材质、镜面材质
    let mut scene = Scene::new();
    scene.add_obj("scenes/CornellBoxMirror.obj", Mat4::IDENTITY);
    scenes.push(scene);

    // 场景二：CornellBoxSphere
    // 场景二包含光源、石膏材质、镜面材质、透明材质
    let mut scene = Scene::new();
    scene.add_obj("scenes/CornellBoxSphere.obj", Mat4::IDENTITY);
    scenes.push(scene);

    // 场景三：CornellBoxName + MyName
    // CornellBoxName 为一个空的 Cornell Box，后方墙壁上贴了一张漫反射贴图，上面是我的学号和名字 :)
    // MyName 中是我的名字，分别使用了石膏材质、镜面材质、透明材质
    let mut scene = Scene::new();

    scene.add_obj("scenes/CornellBoxName.obj", Mat4::IDENTITY);
    let translate = Mat4::from_translation(Vec3::new(0.0, 0.5, -0.5));
    let scale = Mat4::from_scale(Vec3::new(1.2, 0.6, 1.2));
    let rotate = Mat4::from_rotation_x(std::f32::consts::PI / 4.0);
    scene.add_obj("scenes/MyName.obj", translate * rotate * scale);

    scenes.push(scene);

    scenes
}

fn create_camera(aspect_ratio: f32) -> Camera {
    let look_from = Vec3::new(0.0, 1.0, 3.0);
    let look_at = Vec3::new(0.0, 1.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    Camera::new(look_from, look_at, vup, 60.0, aspect_ratio, 4.0, 0.0)
}

fn main() {
    let mut scenes = create_test_scenes();

    let image_width = 640;
    let image_height = 400;
    let max_depth = 5;
    let samples_per_pixel = 10000;

    let camera = Arc::new(create_camera(image_width as f32 / image_height as f32));

    let mut scene_number = 0;

    for mut scene in scenes {
        scene_number += 1;
        println!("Start to render scene_{scene_number}.");
        let start = Instant::now();
        scene.build_bvh();
        let image_data = render::render(
            Arc::new(scene),
            camera.clone(),
            image_width,
            image_height,
            max_depth,
            samples_per_pixel,
        );
        let duration = start.elapsed();
        println!("{:?} for rendering scene_{scene_number} (time for building bvh included).", duration);
        render::save_image_as_png(
            image_data,
            image_width,
            image_height,
            &format!("scene_{scene_number}.png"),
        );
        println!("Result saved as scene_{scene_number}.png\n");
    }
}