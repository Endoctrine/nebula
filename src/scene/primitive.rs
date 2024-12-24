use glam::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::scene::{HitRecord, Hittable};
use crate::scene::bvh::AABB;

/// 球体
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
            return Some(HitRecord::new(point, normal, root, self.material));
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

/// 三角面
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub n0: Vec3,
    pub n1: Vec3,
    pub n2: Vec3,
    pub material: Material,
}

impl Triangle {
    pub fn new(vertices: Vec<Vec3>,
               normals: Vec<Vec3>,
               material: Material) -> Self {
        assert_eq!(vertices.len(), 3);
        let (v0, v1, v2) = (vertices[0], vertices[1], vertices[2]);
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;

        if normals.is_empty() {
            // 没有提供顶点法向的情况下，按 v0 v1 v2 顺序使用右手法则确定法线方向
            let normal = edge1.cross(edge2).normalize();
            Self {
                v0,
                v1,
                v2,
                n0: normal,
                n1: normal,
                n2: normal,
                material,
            }
        } else {
            assert_eq!(normals.len(), 3);
            let (n0, n1, n2) = (normals[0], normals[1], normals[2]);
            Self { v0, v1, v2, n0, n1, n2, material }
        }
    }
}

impl Hittable for Triangle {
    /// 使用 Moller-Trumbore 方法判定光线与三角面的相交情况，
    /// 即解方程 `[-ray.direction, edge1, edge2][t, v, w]^T=[ray.origin-v0]`。
    /// 使用 Cramer's Rule 求解。
    ///
    /// 交点 `p` 满足 `p=u*v0+v*v1+w*v2`，其中 `u+v+w=1`
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;

        // edge1.dot(h) = det([-ray.direction, edge1, edge2])，使用混合积计算，下同
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        // 判断是否平行于三角面
        if a.abs() < f32::EPSILON {
            return None;
        }
        let f = 1.0 / a;

        // s.dot(h) = det([-ray.direction, ray.origin - v0, edge2])
        let s = ray.origin - self.v0;
        let v = f * s.dot(h);

        // 检查参数 v 是否在 [0, 1] 范围内
        if v < 0.0 || v > 1.0 {
            return None;
        }

        // ray.direction.dot(q) = det([-ray.direction, edge1, ray.origin - v0])
        let q = s.cross(edge1);
        let w = f * ray.direction.dot(q);

        // 检查参数 w 是否在 [0, 1] 范围内，且 v + w <= 1
        if w < 0.0 || v + w > 1.0 {
            return None;
        }

        // edge2.dot(q) = det([ray.origin - v0, edge1, edge2])
        let t = f * edge2.dot(q);

        // 检查交点是否在光线范围内
        if t < t_min || t > t_max {
            return None;
        }

        let hit_point = ray.at(t);
        // 使用重心坐标进行插值
        let normal = (1.0 - v - w) * self.n0 + v * self.n1 + w * self.n2;

        Some(HitRecord {
            point: hit_point,
            normal,
            t,
            material: self.material,
        })
    }

    fn bounding_box(&self) -> AABB {
        let max = self.v0.max(self.v1).max(self.v2);
        let min = self.v0.min(self.v1).min(self.v2);
        AABB::new(min, max)
    }
}