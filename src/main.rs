extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod sphere;
mod camera;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::f32;

use hitable::{Hit, Hitable};
use vec3::Vec3;
use ray::Ray;
use sphere::Sphere;
use camera::Camera;

fn color<T: Hitable<f32>> (ray:Ray<f32>, world: &mut Vec<T>) -> Vec3<f32> {
    match hitable::hit_in_list(&ray, 0.0, f32::MAX, world) {
        Some(rec) => {
            0.5 * Vec3::new(rec.normal.x + 1.0,
                            rec.normal.y + 1.0,
                            rec.normal.z + 1.0)
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

    let mut world = Vec::<Sphere<f32>>::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Vec3::new(0.0,-100.5, -1.0), 100.0));

    for idy in (0..y).rev() {
        for idx in 0..x {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for ids in 0..s {
                let ru = rand::random::<f32>();
                let rv = rand::random::<f32>();
                let u = ((idx as f32) + ru )/ x as f32;
                let v = ((idy as f32) + rv )/ y as f32;
                //let u = (idx as f32) / x as f32;
                //let v = (idy as f32) / y as f32;
                let r = camera.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col = col + color(r, &mut world);
            }
            col = col / (s as f32);
            let r:i32 = (255.99 * col.x) as i32;
            let g:i32 = (255.99 * col.y) as i32;
            let b:i32 = (255.99 * col.z) as i32;
            write!(file, "{} {} {}\n", r, g, b);
        }
    }
}
