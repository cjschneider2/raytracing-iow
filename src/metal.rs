use vec3::Vec3;
use ray::Ray;
use hitable::Hit;
use material::Material;
use util::rand_in_unit_sphere;

pub struct Metal {
    pub albedo: Vec3
}

impl Metal {
    pub fn new( _albedo: Vec3) -> Metal {
        Metal { albedo: _albedo }
    }
}

impl Material for Metal {
    fn scatter(&self,
               ray_in: &Ray,
               rec: &Hit,
               attenuation: &Vec3,
               scattered: &Ray) -> (bool, Ray, Vec3)
    {
        let reflected = ray_in.direction()
                              .unit_vector()
                              .reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        let result =
            if scattered.direction().dot(rec.normal) > 0.0 {
                true
            } else {
                false
            };
        ( result , scattered, attenuation )
    }
}
