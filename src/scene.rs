use glam::Vec3;
use crate::ray::Ray;

// 定义一个表示光线与物体碰撞的 trait
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

// 记录光线与物体的碰撞信息
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,      // 交点
    pub normal: Vec3,     // 交点处的物体表面法向量
    pub t: f32,           // 碰撞时间
}

// 球体结构体，用于测试
#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,  // 球心
    pub radius: f32,   // 半径
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    /// 在时刻 t 光线到达的点为 t * ray.direction（简写为 d），
    /// 设圆心到光线始点的向量为 oc = ray.origin - self.center，
    /// 则交点满足方程 |(t * ray.direction + oc)| = self.radius。
    ///
    /// 由于 ray.direction 为单位向量，则该方程实际形式为：
    /// t^2 + 2td·oc + oc^2 = r^2。
    /// 其中：a = 1，b = 2d·oc，c = oc^2 - r^2。
    ///
    /// 求解交点即为求解此方程。
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
            return Some(HitRecord { point, normal, t: root });
        }
        None
    }
}

// 场景结构体
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene { objects: Vec::new() }
    }

    // 将物体添加到场景中
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    // 检查光线与场景中的物体是否碰撞，返回最早发生的碰撞
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut earliest_hit: Option<HitRecord> = None; // 记录最早发生的碰撞
        let mut t = t_max;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, t) {
                earliest_hit = Some(hit);
                t = hit.t;
            }
        }
        earliest_hit
    }
}
