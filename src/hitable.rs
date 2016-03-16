use ray::Ray;
use vec3::Vec3;

struct Hit {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

impl Hit {
    pub fn new() -> Hit {
        Hit {
            t: 0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
        };
    }
}

trait Hitable {
    fn hit(ray: &Ray, t_min: f32, t_max: f32, record: &Hit) -> bool;
}
