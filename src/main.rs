mod vec3;
mod ray;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use vec3::Vec3;
use ray::Ray;

fn hit_sphere(center:Vec3<f32>, radius:f32, ray:Ray<f32>) -> bool {
    let oc = ray.origin() - center;
    let a  = ray.direction().dot(ray.direction());
    let b  = 2.0 * oc.dot(ray.direction());
    let c  = (oc.dot(oc)) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);
    if discriminant > 0.0 {
        true
    } else {
        false
    }
}

fn color(r:Ray<f32>) -> Vec3<f32> {
    if hit_sphere(Vec3::new(0.0, 1.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_dir = r.direction().unit_vec();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + ( t * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
    let nx:i32 = 400;
    let ny:i32 = 200;

    let path = Path::new("image.ppm");

    let mut file = File::create(&path).unwrap();

    // Header
    write!(file,"P3\n{} {}\n255\n",nx, ny).unwrap();

    let ll_corner  = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new( 4.0,  0.0,  0.0);
    let vertical   = Vec3::new( 0.0,  2.0,  0.0);
    let origin     = Vec3::new( 0.0,  0.0,  0.0);

    for idy in (0..nx).rev() {
        for idx in 0..ny {
            let u = idx as f32 / nx as f32;
            let v = idy as f32 / ny as f32;
            let r = Ray::new(origin,
                             ll_corner +
                             u * horizontal +
                             v * vertical);
            let col = color(r);
            let r:i32 = (255.99 * col.x) as i32;
            let g:i32 = (255.99 * col.y) as i32;
            let b:i32 = (255.99 * col.z) as i32;
            write!(file, "{} {} {}\n", r, g, b);
        }
    }
}
