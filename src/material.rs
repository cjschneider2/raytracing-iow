use ray::Ray;
use hitable::Hit;
use vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &Hit,
        attenuation: &Vec3,
        scattered: &Ray)
        -> (bool, Ray, Vec3);
}
