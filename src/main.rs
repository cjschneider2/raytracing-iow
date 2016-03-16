mod vec3;
mod ray;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use vec3::Vec3;
use ray::Ray;

fn color(ray:Ray<f32>) -> Vec3<f32> {
    let record = Hit::new();
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * Vec3::new(n.x+1.0, n.y+1.0, n.z+1.0);
    }
    let unit_dir = ray.direction().unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + ( t * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
    let x:i32 = 400;
    let y:i32 = 200;

    let path = Path::new("image.ppm");

    let mut file = File::create(&path).unwrap();

    // Header
    write!(file,"P3\n{} {}\n255\n",x, y).unwrap();

    let ll_corner  = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new( 4.0,  0.0,  0.0);
    let vertical   = Vec3::new( 0.0,  2.0,  0.0);
    let origin     = Vec3::new( 0.0,  0.0,  0.0);

    for idy in (0..y).rev() {
        for idx in 0..x {
            let u = idx as f32 / x as f32;
            let v = idy as f32 / y as f32;
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
