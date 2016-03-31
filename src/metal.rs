use vec3::Vec3;
use ray::Ray;
use hitable::Hit;
use material::Material;
use util::rand_in_unit_sphere;

pub struct Metal {
    pub albedo: Vec3<f32>
}

impl Metal {
    pub fn new( _albedo: Vec3<f32>) -> Metal {
        Metal { albedo: _albedo }
    }
}

impl Material for Metal {
    fn scatter(&self,
               ray_in: &Ray<f32>,
               rec: &Hit<f32>,
               attenuation: &Vec3<f32>,
               scattered: &Ray<f32>) -> (bool, Ray<f32>, Vec3<f32>)
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
