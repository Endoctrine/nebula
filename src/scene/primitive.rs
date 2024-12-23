use glam::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::scene::{HitRecord, Hittable};
use crate::scene::bvh::AABB;

// 球体结构体，用于测试
#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,  // 球心
    pub radius: f32,   // 半径
    pub material: Material, // 材质
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    /// 在时刻 `t` 光线到达的点为 `t * ray.direction`，
    /// 设圆心到光线始点的向量为 `oc = ray.origin - self.center`，
    /// 则交点满足方程 `|(t * ray.direction + oc)| = self.radius`。
    /// 求解交点即为求解此一元二次方程。
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - c;
        if discriminant > 0.0 {
            let sqrt_d = discriminant.sqrt();
            let mut root = -half_b - sqrt_d;
            if root < t_min || root > t_max {
                root = -half_b + sqrt_d;
                if root < t_min || root > t_max {
                    return None;
                }
            }

            let point = ray.at(root);
            let normal = (point - self.center) / self.radius;
            return Some(HitRecord::new(point, normal, root, self.material()));
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        )
    }

    fn material(&self) -> Material {
        self.material
    }
}
