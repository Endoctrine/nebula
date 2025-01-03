mod bvh;
pub mod primitive;

use std::path::Path;
use std::sync::Arc;
use glam::{Mat4, Vec2, Vec3};
use crate::material::Material;
use crate::ray::Ray;
use crate::scene::bvh::*;
use primitive::Triangle;

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
    pub material: Material, // 碰撞点颜色
}

impl HitRecord {
    pub fn new(point: Vec3, normal: Vec3, t: f32, material: Material) -> Self {
        Self { point, normal: normal.normalize(), t, material }
    }
}

// 场景结构体
pub struct Scene {
    pub objects: Vec<Arc<dyn Hittable + Sync + Send>>,
    pub bvh: Option<BVHNode>,
}

impl Scene {
    const MAX_OBJECTS_PER_BVH_LEAF: usize = 5;

    pub fn new() -> Self {
        Scene { objects: Vec::new(), bvh: None }
    }

    // 将 .obj 模型加载到场景中
    pub fn add_obj(&mut self, file_path: &str, transform: Mat4) {
        // 读取并解析 .obj 文件
        let obj_data = tobj::load_obj(file_path, &tobj::GPU_LOAD_OPTIONS)
            .expect("Failed to load .obj file");
        let (models, materials) = obj_data;
        let materials = materials.expect("Fail to load .mtl file");
        let path = Path::new(file_path);
        let base_path = if let Some(base) = path.parent() {
            base.to_str().unwrap()
        } else {
            "/"
        };

        // 将 .obj 中的每个面转换为三角形
        for mesh in models.iter().map(|model| { &model.mesh }) {
            for index in mesh.indices.chunks(3) {
                // 使用 GPU_LOAD_OPTIONS 的情况下会将 single_index 设为 true，
                // 因此不需要使用 normal_indices、texcoord_indices 等
                let i0 = index[0] as usize;
                let i1 = index[1] as usize;
                let i2 = index[2] as usize;
                let material_id = mesh.material_id.expect("Material id not found!");
                let material = Material::from_mtl(
                    &materials[material_id],
                    base_path,
                );
                // 转换为 Vec3
                let v0 = Vec3::from_slice(&mesh.positions[i0 * 3..i0 * 3 + 3]);
                let v1 = Vec3::from_slice(&mesh.positions[i1 * 3..i1 * 3 + 3]);
                let v2 = Vec3::from_slice(&mesh.positions[i2 * 3..i2 * 3 + 3]);

                let v0 = transform.transform_point3(v0);
                let v1 = transform.transform_point3(v1);
                let v2 = transform.transform_point3(v2);

                let normals = if mesh.normals.is_empty() {
                    vec![]
                } else {
                    let n0 = Vec3::from_slice(&mesh.normals[i0 * 3..i0 * 3 + 3]);
                    let n1 = Vec3::from_slice(&mesh.normals[i1 * 3..i1 * 3 + 3]);
                    let n2 = Vec3::from_slice(&mesh.normals[i2 * 3..i2 * 3 + 3]);

                    let n0 = transform.transform_vector3(n0).normalize();
                    let n1 = transform.transform_vector3(n1).normalize();
                    let n2 = transform.transform_vector3(n2).normalize();

                    vec![n0, n1, n2]
                };
                let texcoords = if mesh.texcoords.is_empty() {
                    vec![]
                } else {
                    let uv0 = Vec2::from_slice(&mesh.texcoords[i0 * 2..i0 * 2 + 2]);
                    let uv1 = Vec2::from_slice(&mesh.texcoords[i1 * 2..i1 * 2 + 2]);
                    let uv2 = Vec2::from_slice(&mesh.texcoords[i2 * 2..i2 * 2 + 2]);

                    vec![uv0, uv1, uv2]
                };

                // 创建三角形
                let triangle = Triangle::new(
                    vec![v0, v1, v2], normals, texcoords, material,
                );
                self.add(Box::new(triangle));
            }
        }
    }

    // 将基本图元添加到场景中
    pub fn add(&mut self, object: Box<dyn Hittable + Sync + Send>) {
        self.objects.push(object.into());
        self.bvh = None;
    }

    pub fn build_bvh(&mut self) {
        self.bvh = Some(BVHNode::build(&mut self.objects, Self::MAX_OBJECTS_PER_BVH_LEAF));
    }

    // 检查光线与场景中的物体是否碰撞，返回最早发生的碰撞
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        assert!(!self.bvh.is_none());
        if let Some(bvh) = &self.bvh {
            bvh.hit(ray, t_min, t_max)
        } else {
            unreachable!()
        }
    }
}
