use glam::{Vec2, Vec3};


/// 生成 tent 滤波下的 [0, 1] 的随机数
pub fn random_unit_tent() -> f32 {
    let rand = rand::random::<f32>() * 2.0;
    if rand < 1.0 {
        rand.sqrt() / 2.0
    } else {
        1.0 - (2.0 - rand).sqrt() / 2.0
    }
}

/// 生成单位圆盘内的均匀采样
pub fn random_in_unit_disk() -> Vec2 {
    loop {
        let p = Vec2::new(rand::random::<f32>(), rand::random::<f32>());
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

/// 生成随机单位向量
pub fn random_unit_vector() -> Vec3 {
    loop {
        let x: f32 = rand::random::<f32>() - 0.5;
        let y: f32 = rand::random::<f32>() - 0.5;
        let z: f32 = rand::random::<f32>() - 0.5;
        let vector = Vec3 { x, y, z };
        if vector.length_squared() > f32::EPSILON {
            return vector.normalize();
        }
    }
}

/// 在给定半球内生成余弦加权分布的随机向量
pub fn random_unit_vector_cosine(normal: Vec3) -> Vec3 {
    // 随机生成二维点
    let r1: f32 = rand::random::<f32>();
    let r2: f32 = rand::random::<f32>();

    let r = r1.sqrt();
    let theta = 2.0 * std::f32::consts::PI * r2;

    // 圆盘坐标
    let x = r * theta.cos();
    let y = r * theta.sin();
    let z = (1.0 - r1).sqrt();

    // 构造正交基
    let tangent = if normal.x.abs() > 0.1 {
        Vec3::new(0.0, 1.0, 0.0).cross(normal).normalize()
    } else {
        Vec3::new(1.0, 0.0, 0.0).cross(normal).normalize()
    };
    let bitangent = normal.cross(tangent);

    // 转换到世界坐标
    tangent * x + bitangent * y + normal * z
}


