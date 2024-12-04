use glam::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        let direction = direction.normalize();
        Ray { origin, direction }
    }

    // 获取光线在时刻 t 到达的位置
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
