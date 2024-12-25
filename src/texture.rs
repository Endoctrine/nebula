use std::collections::HashMap;
use image::{DynamicImage, GenericImageView, Pixel};
use glam::Vec3;
use once_cell::unsync::Lazy;

static mut TEXTURE_STORAGE: Lazy<HashMap<u32, DynamicImage>> =
    Lazy::new(|| HashMap::new());

static mut NEXT_TEXTURE_ID: u32 = 0;

#[derive(Debug, Copy, Clone)]
pub struct Texture {
    id: u32, // 全局的贴图 ID
}

impl Texture {
    // 从文件加载贴图，不支持并发加载
    pub fn load_from_file(file_path: &str) -> Self {
        let image = image::open(file_path).expect("Failed to load texture image");

        let id = unsafe {
            let id = NEXT_TEXTURE_ID;
            TEXTURE_STORAGE.insert(id, image);
            NEXT_TEXTURE_ID += 1;
            id
        };

        Texture { id }
    }

    /// 通过 uv 坐标获取颜色值，其中 u，v 属于 [0.0, 1.0]
    pub fn sample(&self, u: f32, v: f32) -> Vec3 {
        unsafe {
            let image = TEXTURE_STORAGE.get(&self.id).unwrap();
            let (width, height) = image.dimensions();

            let x = (u * width as f32) as u32;
            let y = ((1.0 - v) * height as f32) as u32; // v 轴需要翻转

            let pixel = image.get_pixel(x.min(width - 1), y.min(height - 1));
            let rgb = pixel.to_rgb();

            Vec3::new(
                rgb[0] as f32 / 255.0,
                rgb[1] as f32 / 255.0,
                rgb[2] as f32 / 255.0,
            )
        }
    }
}
