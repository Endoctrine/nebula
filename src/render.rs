use std::fs::File;
use std::io::Write;
use glam::Vec3;
use crate::scene::Scene;
use crate::camera::Camera;
use crate::ray::Ray;

pub fn render(scene: &Scene, camera: &Camera, image_width: u32, image_height: u32) -> Vec<u8> {
    let mut image_data = Vec::with_capacity((image_width * image_height * 3) as usize);

    // 从左到右，从上到下进行渲染
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut color = Vec3::ZERO;
            let u = (i as f32 + 0.5) / image_width as f32;
            let v = (j as f32 + 0.5) / image_height as f32;
            let ray = camera.get_ray(u, v);
            color += ray_color(&ray, scene);
            image_data.push((color.x * 255.99) as u8); // R
            image_data.push((color.y * 255.99) as u8); // G
            image_data.push((color.z * 255.99) as u8); // B
        }
    }

    image_data
}

// 简单的光线颜色计算（无材质，只有背景）
fn ray_color(ray: &Ray, scene: &Scene) -> Vec3 {
    const T_MIN: f32 = 0.1;
    const T_MAX: f32 = 100.0;
    if let Some(hit) = scene.hit(ray, T_MIN, T_MAX) {
        return hit.normal.dot(-ray.direction).clamp(0.0, 1.0) * Vec3::ONE; // 返回法线的着色
    }
    // 背景颜色
    let t = 0.5 * (ray.direction.y + 1.0);
    (1.0 - t) * Vec3::ONE + t * Vec3::new(0.5, 0.7, 1.0)
}

// 将渲染结果保存为 PPM 文件
pub fn save_image_as_ppm(image_data: Vec<u8>, width: u32, height: u32, filename: &str) {
    let mut file = File::create(filename).unwrap();
    writeln!(file, "P6\n{} {}\n255", width, height).unwrap();
    file.write_all(&image_data).unwrap();
}



