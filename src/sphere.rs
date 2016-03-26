use vec3::Vec3;
use hitable::Hit;
use hitable::Hitable;
use ray::Ray;

pub struct Sphere<T> {
    pub center: Vec3<T>,
    pub radius: T,
}

impl Sphere<f32> {
    pub fn new(center: Vec3<f32>, radius: f32) -> Sphere<f32> {
        Sphere { center: center, radius: radius }
    }
}

impl Hitable<f32> for Sphere<f32> {

    fn hit(&self, ray: &Ray<f32>, t_min: f32, t_max: f32, record: &mut Hit<f32>) -> bool {
        let oc = ray.origin() - self.center;
        let a  = ray.direction().dot(ray.direction());
        let b  = 2.0 * oc.dot(ray.direction());
        let c  = (oc.dot(oc)) - (self.radius * self.radius);
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant > 0.0 {
            let t_l = (-b - (b * b - a * c).sqrt()) / a;
            let t_u = (-b + (b * b - a * c).sqrt()) / a;
            if (t_l < t_max) && (t_l > t_min) {
                record.t = t_l;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                return true
            }
            if (t_u < t_max) && (t_u > t_min) {
                record.t = t_u;
                record.p = ray.point_at_parameter(record.t);
                record.normal = (record.p - self.center) / self.radius;
                return true
            }
        }
        false
    }
}

