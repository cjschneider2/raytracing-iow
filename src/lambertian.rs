use vec3::Vec3;
use ray::Ray;
use hitable::Hit;
use material::Material;
use util::rand_in_unit_sphere;

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Lambertian {
    pub fn new( _albedo: Vec3) -> Lambertian {
        Lambertian { albedo: _albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self,
               ray_in: &Ray,
               rec: &Hit,
               attenuation: &Vec3,
               scattered: &Ray) -> (bool, Ray, Vec3)
    {
        let target = rec.p + rec.normal + rand_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        let attenuation = self.albedo;
        ( true, scattered, attenuation )
    }
}
