use ray::Ray;
use hitable::Hit;
use vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray<f32>, rec: &Hit<f32>,
               attenuation: &Vec3<f32>, scattered: &Ray<f32>)
        -> (bool, Ray<f32>, Vec3<f32>);
}
