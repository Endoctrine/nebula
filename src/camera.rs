use glam::{Vec3};
use crate::ray::Ray;
use crate::rand_util;

/// 摄像机，使用薄透镜模型
#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3, // 摄像机原点，即透镜光心
    pub lower_left_corner: Vec3, // 视口左下角
    pub u: Vec3, // 右方向单位向量
    pub v: Vec3, // 上方向单位向量
    pub w: Vec3, // 视线方向反方向单位向量，保证坐标系为右手系
    pub horizontal: Vec3, // 视口水平向量，即 u * viewport_width
    pub vertical: Vec3, // 视口的垂直向量，即 v * viewport_height
    pub focal_length: f32, // 焦距，即原点到视口平面的距离
    pub lens_radius: f32, // 透镜半径，即理想光圈半径
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3, // 上方向向量
        vertical_fov: f32, // 视场角，角度制
        aspect_ratio: f32, // 宽高比
        focal_length: f32,
        lens_radius: f32,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan() * focal_length;
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = vup.normalize();

        let lower_left_corner = look_from
            - u * viewport_width / 2.0
            - v * viewport_height / 2.0
            - w * focal_length;

        Camera {
            origin: look_from,
            lower_left_corner,
            u,
            v,
            w,
            horizontal: u * viewport_width,
            vertical: v * viewport_height,
            focal_length,
            lens_radius,
        }
    }

    /// 根据像素位置生成光线
    pub fn get_ray(&self, horizontal_ratio: f32, vertical_ratio: f32) -> Ray {
        let random_in_lens = self.lens_radius * rand_util::random_in_unit_disk();
        let offset = self.u * random_in_lens.x + self.v * random_in_lens.y;

        // 焦平面上任意一点发出的光经薄透镜折射后，光的方向与透镜光心与该点连线平行
        let direction = self.lower_left_corner
            + self.horizontal * horizontal_ratio
            + self.vertical * vertical_ratio
            - self.origin;

        Ray::new(self.origin + offset, direction)
    }
}
