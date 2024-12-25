use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use glam::Vec3;
use image::{Rgb, RgbImage};
use crate::scene::Scene;
use crate::camera::Camera;
use crate::rand_util::random_unit_tent;
use crate::ray::Ray;

const T_MIN: f32 = 0.001;
const T_MAX: f32 = 100000.0;

pub fn render(
    scene: Arc<Scene>,
    camera: Arc<Camera>,
    image_width: u32,
    image_height: u32,
    max_depth: u32,
    samples_per_pixel: u32,
) -> Vec<u8> {
    let image_data_raw = vec![0.0; (image_width * image_height * 3) as usize];
    let image_data_raw = Arc::new(Mutex::new(image_data_raw));

    (0..image_height).rev().collect::<Vec<_>>().par_iter().map(|j| {
        let j = *j;
        let image_data_raw = image_data_raw.clone();
        let scene = scene.clone();
        let camera = camera.clone();
        for i in 0..image_width {
            let mut color = Vec3::ZERO;
            for _ in 0..samples_per_pixel {
                // 在一个像素内进行采样
                let shift_u = random_unit_tent();
                let shift_v = random_unit_tent();
                let u = (i as f32 + shift_u) / image_width as f32;
                let v = (j as f32 + shift_v) / image_height as f32;
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &*scene, 0, max_depth);
            }
            color /= samples_per_pixel as f32;
            color = color.clamp(Vec3::ZERO, Vec3::ONE);
            let mut image_data_raw = image_data_raw.lock().unwrap();
            image_data_raw[((i + (image_height - 1 - j) * image_width) * 3) as usize] = color.x; // R
            image_data_raw[((i + (image_height - 1 - j) * image_width) * 3 + 1) as usize] = color.y; // G
            image_data_raw[((i + (image_height - 1 - j) * image_width) * 3 + 2) as usize] = color.z; // B
        }
    }).collect::<()>();

    let image_data_raw = image_data_raw.lock().unwrap();
    (&*image_data_raw).iter().map(|x| { (x * 255.99) as u8 }).collect::<Vec<_>>()
}

/// 光线颜色计算
fn ray_color(ray: &Ray, scene: &Scene, depth: u32, max_depth: u32) -> Vec3 {
    if let Some(hit) = scene.hit(ray, T_MIN, T_MAX) {
        let m = hit.material;
        let mut color = m.ambient_color() + m.emissive_color();
        // 如果弹射次数大于设定的次数，就不再弹射了
        if depth > max_depth {
            return color;
        }
        // 光线照射到物体后被分散为若干光线
        let scattered_rays = m.scatter(ray, hit);
        for scattered_ray in &scattered_rays {
            color += ray_color(&scattered_ray.ray, scene, depth + 1, max_depth)
                * scattered_ray.coefficient;
        }
        return color;
    }

    // 背景颜色为黑色
    Vec3::ZERO
}


/// 将渲染结果保存为 PPM 文件
pub fn save_image_as_ppm(image_data: Vec<u8>, width: u32, height: u32, filename: &str) {
    let mut file = File::create(filename).unwrap();
    writeln!(file, "P6\n{} {}\n255", width, height).unwrap();
    file.write_all(&image_data).unwrap();
}

/// 将渲染结果保存为 png 文件
pub fn save_image_as_png(image_data: Vec<u8>, width: u32, height: u32, filename: &str) {
    let mut img = RgbImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let index = ((y * width + x) * 3) as usize;
            let pixel = Rgb([
                image_data[index],
                image_data[index + 1],
                image_data[index + 2],
            ]);
            img.put_pixel(x, y, pixel);
        }
    }

    img.save(filename).expect("Failed to save PNG image");
}

