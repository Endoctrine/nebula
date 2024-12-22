use std::sync::Arc;
use glam::Vec3;
use crate::ray::Ray;
use crate::scene::{HitRecord, Hittable};

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        AABB { min, max }
    }

    // 检查光线是否与包围盒相交，使用 slabs 方法
    pub fn hit(&self, ray: &Ray) -> bool {
        let (mut t_min, mut t_max) = (f32::MIN, f32::MAX);
        // 遍历所有轴
        for i in 0..3 {
            // 如果沿该轴方向速度为零，则检测是否夹在两个 slab 中间
            if ray.direction[i].abs() < f32::EPSILON {
                if ray.origin[i] <= self.min[i] || ray.origin[i] >= self.min[i] {
                    return false;
                }
            } else {
                let mut t0 = (self.min[i] - ray.origin[i]) / ray.direction[i];
                let mut t1 = (self.max[i] - ray.origin[i]) / ray.direction[i];

                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                t_min = t_min.max(t0);
                t_max = t_max.min(t1);

                if t_min > t_max || t_max <= 0.0 {
                    return false;
                }
            }
        }

        true
    }

    // 合并两个 aabb
    pub fn merge(&self, rhs: &Self) -> Self {
        AABB {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }

    pub fn surface_area_half(&self) -> f32 {
        let [a, b, c] = (self.max - self.min).to_array();
        a * b + b * c + c * a
    }
}

pub enum BVHNode {
    Internal { left: Box<BVHNode>, right: Box<BVHNode>, bbox: AABB },
    Leaf { objects: Vec<Arc<dyn Hittable + Sync + Send>>, bbox: AABB },
}

impl BVHNode {
    // 构建 BVH
    pub fn build(objects: &mut [Arc<dyn Hittable + Sync + Send>], max_objects_per_leaf: usize) -> Self {
        if objects.len() <= max_objects_per_leaf {
            let mut bbox = objects[0].bounding_box();
            for object in objects.iter() {
                bbox = bbox.merge(&object.bounding_box());
            }
            return BVHNode::Leaf {
                objects: objects.iter().map(|x| x.clone()).collect::<Vec<_>>(),
                bbox,
            };
        }

        // 使用表面积启发确定分割位置
        let (mut best_axis, mut best_division_index, mut min_cost) = (0, 0, f32::MAX);
        // 遍历所有轴
        for axis in 0..3 {
            objects.sort_by(|a, b| {
                let a_center = a.bounding_box().min[axis];
                let b_center = b.bounding_box().min[axis];
                a_center.partial_cmp(&b_center).unwrap()
            });
            // 从右向左，计算右子树的代价
            let mut cost_r2l = vec![];
            let mut bbox = AABB::new(Vec3::ZERO, Vec3::ZERO);
            for (index, object) in objects[1..].iter().rev().enumerate() {
                bbox = bbox.merge(&object.bounding_box());
                cost_r2l.push(bbox.surface_area_half() * (index + 1) as f32);
            }
            cost_r2l.reverse();
            bbox = AABB::new(Vec3::ZERO, Vec3::ZERO);
            // 从左向右，计算整体代价
            for i in 0..objects.len() - 1 {
                bbox = bbox.merge(&objects[i].bounding_box());
                let cost = bbox.surface_area_half() * (i + 1) as f32 + cost_r2l[i];
                if cost < min_cost {
                    (best_axis, best_division_index, min_cost) = (axis, i, cost);
                }
            }
        }

        objects.sort_by(|a, b| {
            let a_center = a.bounding_box().min[best_axis];
            let b_center = b.bounding_box().min[best_axis];
            a_center.partial_cmp(&b_center).unwrap()
        });

        let left = BVHNode::build(&mut objects[..best_division_index + 1].to_vec(), max_objects_per_leaf);
        let right = BVHNode::build(&mut objects[best_division_index + 1..].to_vec(), max_objects_per_leaf);

        let bbox = left.bbox().merge(&right.bbox());

        BVHNode::Internal { left: Box::new(left), right: Box::new(right), bbox }
    }

    // 获取节点的包围盒
    pub fn bbox(&self) -> AABB {
        match self {
            BVHNode::Internal { bbox, .. } => *bbox,
            BVHNode::Leaf { bbox, .. } => *bbox,
        }
    }

    // 检查光线与 BVH 中的物体是否相交
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bbox().hit(ray) {
            return None;
        }

        match self {
            BVHNode::Internal { left, right, .. } => {
                let mut closest_hit = None;
                let mut closest_t = t_max;


                if let Some(hit) = left.hit(ray, t_min, closest_t) {
                    closest_hit = Some(hit);
                    closest_t = hit.t;
                }

                if let Some(hit) = right.hit(ray, t_min, closest_t) {
                    closest_hit = Some(hit);
                }

                closest_hit
            }
            BVHNode::Leaf { objects, .. } => {
                let mut closest_hit = None;
                let mut closest_t = t_max;

                for object in objects {
                    if let Some(hit) = object.hit(ray, t_min, closest_t) {
                        closest_hit = Some(hit);
                        closest_t = hit.t;
                    }
                }

                closest_hit
            }
        }
    }
}