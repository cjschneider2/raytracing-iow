use vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray<T> {
    a: Vec3<T>,
    b: Vec3<T>,
}

impl Ray<f32> {
    pub fn new(x: Vec3<f32>, y: Vec3<f32>) -> Ray<f32> {
        Ray::<f32> { a: x, b: y, }
    }

    pub fn origin(self) -> Vec3<f32> {
        self.a
    }

    pub fn direction(self) -> Vec3<f32> {
        self.b
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3<f32> {
        self.a + (self.b * t)
    }
}
