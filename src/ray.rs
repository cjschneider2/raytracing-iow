use vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(x: Vec3, y: Vec3) -> Ray {
        Ray { a: x, b: y, }
    }

    pub fn origin(self) -> Vec3 {
        self.a
    }

    pub fn direction(self) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3 {
        self.a + (self.b * t)
    }
}
