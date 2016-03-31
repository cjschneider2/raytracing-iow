extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;
mod util;
mod material;
mod lambertian;
mod metal;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::f32;

use hitable::{Hit, Hitable};
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use camera::Camera;
use util::rand_in_unit_sphere;
use lambertian::Lambertian;
use metal::Metal;

fn color<T: Hitable<f32>> (ray:Ray<f32>, world: &mut Vec<T>) -> Vec3<f32> {
    match hitable::hit_in_list(&ray, 0.001, f32::MAX, world) {
        Some(rec) => {
            let target = rec.p + rec.normal + rand_in_unit_sphere();
            //let ( result, scattered, attenuation ) = ::scatter
            0.5 * color( Ray::new(rec.p, target-rec.p), world)
        },
        None => {
            let unit_dir = ray.direction().unit_vector();
            let t = 0.5 * (unit_dir.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0)
                      + ( t * Vec3::new(0.5, 0.7, 1.0))
        },
    }
}

fn main() {
    let x:i32 = 400;
    let y:i32 = 200;
    let s:i32 = 30;

    let path = Path::new("image.ppm");

    let mut file = File::create(&path).unwrap();

    // Header
    write!(file,"P3\n{} {}\n255\n",x, y).unwrap();

    let camera = Camera::new();

    let lambert_1 = Lambertian::new(Vec3::new(0.8, 0.3, 0.3));
    let lambert_2 = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));

    let mut world = Vec::<Sphere<f32>>::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, lambert_1));
    world.push(Sphere::new(Vec3::new(0.0,-100.5, -1.0), 100.0, lambert_1));

    for idy in (0..y).rev() {
        for idx in 0..x {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..s {
                let ru = rand::random::<f32>();
                let rv = rand::random::<f32>();
                let u = ((idx as f32) + ru )/ x as f32;
                let v = ((idy as f32) + rv )/ y as f32;
                //let u = (idx as f32) / x as f32;
                //let v = (idy as f32) / y as f32;
                let r = camera.get_ray(u, v);
                //let p = r.point_at_parameter(2.0);
                col = col + color(r, &mut world);
            }
            col = col / (s as f32);
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let r:i32 = (255.99 * col.x) as i32;
            let g:i32 = (255.99 * col.y) as i32;
            let b:i32 = (255.99 * col.z) as i32;
            write!(file, "{} {} {}\n", r, g, b);
        }
    }
}
