use vec3::Vec3;
use ray::Ray;
use hitable::Hit;
use material::Material;
use util::rand_in_unit_sphere;

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3<f32>
}

impl Lambertian {
    pub fn new( _albedo: Vec3<f32>) -> Lambertian {
        Lambertian { albedo: _albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self,
               ray_in: &Ray<f32>,
               rec: &Hit<f32>,
               attenuation: &Vec3<f32>,
               scattered: &Ray<f32>) -> (bool, Ray<f32>, Vec3<f32>)
    {
        let target = rec.p + rec.normal + rand_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        let attenuation = self.albedo;
        ( true, scattered, attenuation )
    }
}
