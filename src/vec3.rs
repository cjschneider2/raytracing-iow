use std::ops::Neg;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::ops::Mul;

#[derive(Clone, Copy, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl Vec3<f32> {

    pub fn new(a:f32, b:f32, c:f32) -> Vec3<f32> {
        Vec3 { x: a, y: b, z: c}
    }

    pub fn length(self) -> f32 {
        ((self.x * self.x) +
         (self.y * self.y) +
         (self.z * self.z)).sqrt()
    }

    pub fn sqared_length(self) -> f32 {
        ((self.x * self.x) +
         (self.y * self.y) +
         (self.z * self.z))
    }

    pub fn unit_vector(self) -> Vec3<f32> {
        self / self.length()
    }

    pub fn cross(self, other: Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: ((self.y * other.z) - (self.z * other.y)),
            y: (-(self.x * other.z) - (self.z * other.x)),
            z: ((self.x * other.y) - (self.y * other.x)),
        }
    }

    pub fn dot(self, other: Vec3<f32>) -> f32 {
        ((self.x * other.x) +
         (self.y * other.y) +
         (self.z * other.z) )
    }
}

impl Neg for Vec3<f32> {
    type Output = Vec3<f32>;
    fn neg(self) -> Vec3<f32> {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3<f32> {
    type Output = Vec3<f32>;
    fn add(self, other:Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3<f32> {
    type Output = Vec3<f32>;
    fn sub(self, other:Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Div<Vec3<f32>> for Vec3<f32> {
    type Output = Vec3<f32>;
    fn div(self, other:Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f32> for Vec3<f32> {
    type Output = Vec3<f32>;
    fn div(self, divisor:f32) -> Vec3<f32> {
        Vec3 {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
        }
    }
}

impl Mul<Vec3<f32>> for Vec3<f32> {
    type Output = Vec3<f32>;
    fn mul(self, other:Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3<f32> {
    type Output = Vec3<f32>;
    fn mul(self, factor:f32) -> Vec3<f32> {
        Vec3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl Mul<Vec3<f32>> for f32 {
    type Output = Vec3<f32>;
    fn mul(self, vec:Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}
