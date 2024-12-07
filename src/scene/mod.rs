mod bvh;

use std::rc::Rc;
use glam::Vec3;
use crate::ray::Ray;
use crate::scene::bvh::*;

// 定义一个表示光线与物体碰撞的 trait
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}

// 记录光线与物体的碰撞信息
#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub point: Vec3,      // 交点
    pub normal: Vec3,     // 交点处的物体表面法向量，是单位向量
    pub t: f32,           // 碰撞时间
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32) -> Self {
        Self { point, normal: normal.normalize(), t }
    }
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
            return Some(HitRecord::new(point, normal, root));
        }
        None
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        )
    }
}

// 场景结构体
pub struct Scene {
    pub objects: Vec<Rc<dyn Hittable>>,
    pub bvh: Option<BVHNode>,
}

impl Scene {
    const MAX_OBJECTS_PER_BVH_LEAF: usize = 5;

    pub fn new() -> Self {
        Scene { objects: Vec::new(), bvh: None }
    }

    // 将物体添加到场景中
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object.into());
        self.bvh = None;
    }

    fn build_bvh(&mut self) {
        self.bvh = Some(BVHNode::build(&mut self.objects, Self::MAX_OBJECTS_PER_BVH_LEAF));
    }

    // 检查光线与场景中的物体是否碰撞，返回最早发生的碰撞
    pub fn hit(&mut self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bvh.is_none() {
            self.build_bvh();
        }
        if let Some(bvh) = &self.bvh {
            bvh.hit(ray, t_min, t_max)
        } else {
            unreachable!()
        }
    }
}
