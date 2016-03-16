use vec3::Vec3;
use hitable::Hit;
use hitable::Hitable;

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) {
        Sphere { center: center, radius: radius }
    }
}

impl Hitable for Sphere {

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &Hit) -> bool {
        let oc = ray.origin() - center;
        let a  = ray.direction().dot(ray.direction());
        let b  = 2.0 * oc.dot(ray.direction());
        let c  = (oc.dot(oc)) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant > 0.0 {
            let t_l = (-b - (b * b - a * c).sqrt()) / a;
            let t_u = (-b + (b * b - a * c).sqrt()) / a;
            if ((t_l < t_max) && (t_l > t_min))
            || ((tmp < t_max) && (tmp > t_min)) {
                record.t = tmp;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                return true
            }
        }
        false
    }
}

