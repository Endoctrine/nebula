use std::str::FromStr;
use glam::Vec3;
use crate::ray::Ray;
use crate::scene::HitRecord;
use crate::rand_util;

/// 光线经物体表面作用后出射的光线
#[derive(Debug, Copy, Clone)]
pub struct ScatteredRay {
    pub ray: Ray,
    pub coefficient: Vec3,
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub ambient: Vec3, // 环境光，分量属于[0.0, 1.0]
    pub diffuse: Vec3, // 漫反射，分量属于[0.0, 1.0]
    pub specular: Vec3, // 镜面反射，分量属于[0.0, 1.0]
    pub emissive: Vec3, // 自发光，分量属于[0.0, 1.0)
    pub transmission_filter: Vec3, // 透光颜色，分量属于[0.0, 1.0]
    pub dissolve: f32, // 透明度，属于[0.0, 1.0]
    pub specular_exponent: f32, // 镜面反射指数，属于(-inf, +inf)
    pub optical_density: f32, // 折射率，属于[1.0, +inf)
}

impl Material {
    const FUZZ: f32 = 0.1; // 镜面反射的模糊因子
    const AMBIENT_STRENGTH: f32 = 0.1; // 环境光强度因子

    // 石膏
    pub const PLASTER: Self = Self {
        ambient: Vec3::new(0.1, 0.1, 0.1),
        diffuse: Vec3::new(0.8, 0.8, 0.8),
        specular: Vec3::new(0.8, 0.8, 0.8),
        emissive: Vec3::ZERO,
        transmission_filter: Vec3::ZERO,
        dissolve: 0.0,
        specular_exponent: 0.0,
        optical_density: 1.0,
    };

    // 发光体
    pub const LUMINOUS: Self = Self {
        ambient: Vec3::ZERO,
        diffuse: Vec3::ZERO,
        specular: Vec3::ZERO,
        emissive: Vec3::ONE,
        transmission_filter: Vec3::ZERO,
        dissolve: 0.0,
        specular_exponent: 0.0,
        optical_density: 1.0,
    };

    // 镜面
    pub const MIRROR: Self = Self {
        ambient: Vec3::ZERO,
        diffuse: Vec3::ZERO,
        specular: Vec3::new(2.0, 2.0, 2.0),
        emissive: Vec3::ZERO,
        transmission_filter: Vec3::ZERO,
        dissolve: 0.0,
        specular_exponent: 1000.0,
        optical_density: 1.0,
    };

    // 玻璃
    pub const GLASS: Self = Self {
        ambient: Vec3::ZERO,
        diffuse: Vec3::ZERO,
        specular: Vec3::new(2.0, 2.0, 2.0),
        emissive: Vec3::ZERO,
        transmission_filter: Vec3::ONE,
        dissolve: 0.9,
        specular_exponent: 1000.0,
        optical_density: 1.5,
    };

    pub fn from_mtl(material: &tobj::Material) -> Self {
        let ambient = material.ambient.expect("Ambient not found");
        let diffuse = material.diffuse.expect("Diffuse not found");
        let specular = material.specular.expect("Specular not found");
        let dissolve = material.dissolve.unwrap_or(0.0);
        let specular_exponent = material.shininess.expect("Shininess not found!");
        let optical_density = material.optical_density.expect("Optical density not found!");

        let emissive = material.unknown_param.get(&String::from("Ke"));
        let emissive = if let Some(emissive) = emissive {
            let emissive = emissive.split(" ")
                .filter(|x| { !x.is_empty() })
                .map(|x| {
                    f32::from_str(x).unwrap()
                }).collect::<Vec<_>>();
            Vec3::from_slice(&emissive)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        };

        Self {
            ambient: Vec3::from_slice(&ambient),
            diffuse: Vec3::from_slice(&diffuse),
            specular: Vec3::from_slice(&specular),
            emissive,
            transmission_filter: Vec3::ONE,
            dissolve,
            specular_exponent,
            optical_density,
        }
    }

    /// 入射光线照射到某材质被分散成若干条出射光线
    ///
    /// 入射光颜色 = 出射光线颜色 * 系数 + 自发光颜色 + 环境光颜色
    pub fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Vec<ScatteredRay> {
        let mut scattered_rays = vec![];
        let normal = hit_record.normal;
        let origin = hit_record.point;

        // 漫反射
        let diffuse_coefficient = self.diffuse * 0.5 * (1.0 - self.dissolve);
        let diffuse_direction = rand_util::random_unit_vector_cosine(normal);
        let diffuse_ray = Ray::new(origin, diffuse_direction);

        if diffuse_coefficient.max_element() > 0.0 {
            scattered_rays.push(ScatteredRay {
                ray: diffuse_ray,
                coefficient: diffuse_coefficient,
            });
        }

        // 镜面反射
        let specular_coefficient = self.specular * 0.5 * (1.0 - self.dissolve);
        let mut specular_direction = ray.direction.reflect(hit_record.normal);
        specular_direction +=
            Self::FUZZ.powf(self.specular_exponent) * rand_util::random_unit_vector();
        specular_direction = specular_direction.normalize();
        let specular_ray = Ray::new(origin, specular_direction);

        if specular_coefficient.max_element() > 0.0 {
            scattered_rays.push(ScatteredRay {
                ray: specular_ray,
                coefficient: specular_coefficient,
            });
        }

        // 透射，这里认为反射与折射的能量分配总是平权的
        let transmissive_coefficient = self.transmission_filter * self.dissolve;

        if transmissive_coefficient.max_element() > 0.0 {
            if let Some(transmissive_direction) = self.refract(ray, hit_record.normal) {
                let transmissive_ray = Ray::new(origin, transmissive_direction);
                scattered_rays.push(ScatteredRay {
                    ray: transmissive_ray,
                    coefficient: transmissive_coefficient,
                });
            }
        }

        scattered_rays
    }

    /// 计算自发光颜色
    pub fn emissive_color(&self, ray: &Ray, normal: Vec3) -> Vec3 {
        self.emissive * 5.0
    }

    /// 计算材质的环境光颜色
    pub fn ambient_color(&self) -> Vec3 {
        self.ambient * (1.0 - self.dissolve) * Self::AMBIENT_STRENGTH
    }

    /// 计算折射光线的方向
    fn refract(&self, ray: &Ray, normal: Vec3) -> Option<Vec3> {
        let cos_theta = ray.direction.dot(normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if cos_theta > 0.0 {
            // 光线射出当前材料
            let sin_phi = sin_theta * self.optical_density;
            let cos_phi = (1.0 - sin_phi * sin_phi).sqrt();
            // 发生全反射
            if sin_phi > 1.0 {
                return None;
            }
            // 直射
            if sin_phi < f32::EPSILON {
                return Some(normal);
            }

            let u = normal.cross(ray.direction.cross(normal)).normalize();
            let v = normal;
            Some((sin_phi * u + cos_phi * v).normalize())
        } else {
            // 光线射入当前材料
            let sin_phi = sin_theta / self.optical_density;
            let cos_phi = (1.0 - sin_phi * sin_phi).sqrt();
            // 直射
            if sin_phi < f32::EPSILON {
                return Some(-normal);
            }

            let u = normal.cross(ray.direction.cross(normal)).normalize();
            let v = -normal;
            Some((sin_phi * u + cos_phi * v).normalize())
        }
    }
}
