use ray::Ray;
use vec3::Vec3;

pub struct Camera {
    ll_corner  : Vec3<f32>,
    horizontal : Vec3<f32>,
    vertical   : Vec3<f32>,
    origin     : Vec3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            ll_corner  : Vec3::new(-2.0, -1.0, -1.0),
            horizontal : Vec3::new( 4.0,  0.0,  0.0),
            vertical   : Vec3::new( 0.0,  2.0,  0.0),
            origin     : Vec3::new( 0.0,  0.0,  0.0),
        }
    }
    pub fn get_ray(&self, u: f32, v:f32) -> Ray<f32> {
        Ray::new(self.origin,
                 self.ll_corner +
                 u * self.horizontal +
                 v * self.vertical)
    }
}
